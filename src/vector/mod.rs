//! A module with vector related items

use std::array::IntoIter;
use std::fmt::{Debug, Display};
use std::mem::{ManuallyDrop, transmute, transmute_copy};
use std::ops::{
    Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Rem, Shl, Shr, Sub,
};
use std::slice::IterMut;

use crate::{Construct, sealed::Sealed};

pub use crate::declare_vector_aliases;

mod constructor;
mod deref;
mod dir;
mod ops;
mod primitive_api;
mod primitive_impls;
pub use constructor::*;
pub use dir::*;

#[cfg(feature = "swizzle")]
mod swizzle;

/// A generic vector type.
///
/// Is generic over 3 parameters:
/// - `N` - the number of elements in the vector
/// - `T` - the type of elements in the vector
/// - `S` - whether or not the vector is SIMD-backed
///
/// ## SIMD
///
/// - [`Simd`] vectors are SIMD aligned and use explicit SIMD instructions.
/// Their inner implementation is controlled by [`T as Scalar<N, Simd>`][Scalar].
///
/// - [`NonSimd`] vectors are stored as arrays and don't use SIMD instructions
///
/// It is recommended to use [`Simd`] as the default, and to only use [`NonSimd`] in memory-critical scenarios.
///
/// ## Type aliases
///
/// There are type aliases for both [`Simd`] and [`NonSimd`] vectors (e.g., [`Vec2<T>`], and [`Vec2S<T>`]).
///
/// Additionaly, the `primitive_aliases` feature enables type aliases for primitive vectors
/// (e.g., [`FVec2`], [`DVec4S`], and [`U8Vec3`]).
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar<N, S>, S: Simdness>(pub T::InnerVectorType);

/// Type alias for [`Vector<2, T, Simd>`].
pub type Vec2<T> = Vector<2, T, Simd>;
/// Type alias for [`Vector<3, T, Simd>`].
pub type Vec3<T> = Vector<3, T, Simd>;
/// Type alias for [`Vector<4, T, Simd>`].
pub type Vec4<T> = Vector<4, T, Simd>;

/// Type alias for [`Vector<2, T, NonSimd>`] ("s" stands for "scalar").
pub type Vec2S<T> = Vector<2, T, NonSimd>;
/// Type alias for [`Vector<3, T, NonSimd>`] ("s" stands for "scalar").
pub type Vec3S<T> = Vector<3, T, NonSimd>;
/// Type alias for [`Vector<4, T, NonSimd>`] ("s" stands for "scalar").
pub type Vec4S<T> = Vector<4, T, NonSimd>;

/// Macro to declare type-aliases for vectors of a specific element type with a given prefix.
///
/// # Example
///
/// ```
/// use ggmath::declare_vector_aliases;
///
/// declare_vector_aliases!(pub type F => f32);
///
/// // Generates:
/// // pub type FVec2 = Vector<2, f32, Simd>;
/// // pub type FVec3 = Vector<3, f32, Simd>;
/// // pub type FVec4 = Vector<4, f32, Simd>;
/// // pub type FVec2S = Vector<2, f32, NonSimd>;
/// // pub type FVec3S = Vector<3, f32, NonSimd>;
/// // pub type FVec4S = Vector<4, f32, NonSimd>;
/// ```
#[macro_export]
macro_rules! declare_vector_aliases {
    ($vis:vis type $prefix:ident => $T:ty) => {
        $crate::hidden::paste! {
            #[doc = "Type alias for [`Vector<2, " $T ", Simd>`]."]
            $vis type [<$prefix Vec2>] = $crate::Vector<2, $T, $crate::Simd>;
            #[doc = "Type alias for [`Vector<3, " $T ", Simd>`]."]
            $vis type [<$prefix Vec3>] = $crate::Vector<3, $T, $crate::Simd>;
            #[doc = "Type alias for [`Vector<4, " $T ", Simd>`]."]
            $vis type [<$prefix Vec4>] = $crate::Vector<4, $T, $crate::Simd>;

            #[doc = "Type alias for [`Vector<2, " $T ", NonSimd>`] (\"s\" stands for \"scalar\")."]
            $vis type [<$prefix Vec2S>] = $crate::Vector<2, $T, $crate::NonSimd>;
            #[doc = "Type alias for [`Vector<3, " $T ", NonSimd>`] (\"s\" stands for \"scalar\")."]
            $vis type [<$prefix Vec3S>] = $crate::Vector<3, $T, $crate::NonSimd>;
            #[doc = "Type alias for [`Vector<4, " $T ", NonSimd>`] (\"s\" stands for \"scalar\")."]
            $vis type [<$prefix Vec4S>] = $crate::Vector<4, $T, $crate::NonSimd>;
        }
    };
}

pub unsafe trait TransmuteTo<T>: Sized {
    fn transmute_to(self) -> T {
        const { assert!(size_of::<Self>() == size_of::<T>()) }
        const { assert!(align_of::<Self>() >= align_of::<T>()) }

        let value = ManuallyDrop::new(self);

        unsafe { transmute_copy::<ManuallyDrop<Self>, T>(&value) }
    }
}

pub unsafe trait InnerVectorType<const N: usize, T: Construct> {
    const PADDING: Option<Self>;
}

/// A trait for [`Vector`] element types.
/// Is generic over `N` and `S`, which means that `T: Scalar<N, S>` can be put in [`Vector<N, T, S>`]
///
/// ## SIMD
///
/// This trait controls:
/// - The inner type contained inside [`Vector<N, T, S>`]
/// - The implementations of [`Vector<N, T, S>`] functions
///
/// This allows implementations to add SIMD optimizations to vectors.
///
/// ## Example
///
/// TODO: add example
pub trait Scalar<const N: usize, S: Simdness>: Construct {
    /// The inner type contained inside [`Vector<N, Self, S>`].
    type InnerVectorType: Construct + InnerVectorType<N, Self>;

    /// Overridable implementation of [`Vector::from_array`].
    #[inline(always)]
    fn vec_from_array(array: [Self; N]) -> Vector<N, Self, S> {
        Vector::const_from_array(array)
    }

    /// Overridable implementation of [`Vector`] splat, which can be used through the [`vec2!`], [`vec3!`], etc. macros.
    #[inline(always)]
    fn vec_splat(value: Self) -> Vector<N, Self, S> {
        Vector::from_array([value; N])
    }

    /// Overridable implementation of [`Vector::as_array`].
    #[inline(always)]
    fn vec_as_array(vec: Vector<N, Self, S>) -> [Self; N] {
        *vec.as_array_ref()
    }

    /// Overridable implementation of [`Vector::reverse`].
    #[inline(always)]
    fn vec_reverse(vec: Vector<N, Self, S>) -> Vector<N, Self, S> {
        // SAFETY: index is in bounds because (N - 1) is the last valid index and i is in 0..=N-1.
        Vector::from_fn(|i| unsafe { vec.get_unchecked((N - 1) - i) })
    }

    /// Overridable implementation of [`Vector::get_const_vec2`].
    ///
    /// ## Safety
    ///
    /// Calling this function with out of bounds indices is undefined behavior.
    /// Implementations can assume that the indices are in bounds.
    #[inline(always)]
    unsafe fn vec_get_const_vec2<const X_SRC: usize, const Y_SRC: usize>(
        vec: Vector<N, Self, S>,
    ) -> Vector<2, Self, S>
    where
        Self: Scalar<2, S>,
    {
        vec2g!(vec[X_SRC], vec[Y_SRC])
    }

    /// Overridable implementation of [`Vector::get_const_vec3`].
    ///
    /// ## Safety
    ///
    /// Calling this function with out of bounds indices is undefined behavior.
    /// Implementations can assume that the indices are in bounds.
    #[inline(always)]
    unsafe fn vec_get_const_vec3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vector<N, Self, S>,
    ) -> Vector<3, Self, S>
    where
        Self: Scalar<3, S>,
    {
        vec3g!(vec[X_SRC], vec[Y_SRC], vec[Z_SRC])
    }

    /// Overridable implementation of [`Vector::get_const_vec4`].
    ///
    /// ## Safety
    ///
    /// Calling this function with out of bounds indices is undefined behavior.
    /// Implementations can assume that the indices are in bounds.
    #[inline(always)]
    unsafe fn vec_get_const_vec4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vector<N, Self, S>,
    ) -> Vector<4, Self, S>
    where
        Self: Scalar<4, S>,
    {
        vec4g!(vec[X_SRC], vec[Y_SRC], vec[Z_SRC], vec[W_SRC])
    }

    /// Overridable implementation of [`Vector::eq`].
    #[inline(always)]
    fn vec_eq(vec: Vector<N, Self, S>, other: Vector<N, Self, S>) -> bool
    where
        Self: PartialEq,
    {
        vec.as_array() == other.as_array()
    }

    /// Overridable implementation of [`Vector::ne`].
    #[inline(always)]
    fn vec_ne(vec: Vector<N, Self, S>, other: Vector<N, Self, S>) -> bool
    where
        Self: PartialEq,
    {
        !(vec == other)
    }

    /// Overridable implementation of [`Vector::neg`].
    #[inline(always)]
    fn vec_neg(vec: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Neg<Output = Self>,
    {
        vec.map(Neg::neg)
    }

    /// Overridable implementation of [`Vector::not`].
    #[inline(always)]
    fn vec_not(vec: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Not<Output = Self>,
    {
        vec.map(Not::not)
    }

    /// Overridable implementation of [`Vector::add`].
    #[inline(always)]
    fn vec_add(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Add<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a + b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::sub`].
    #[inline(always)]
    fn vec_sub(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Sub<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a - b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::mul`].
    #[inline(always)]
    fn vec_mul(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Mul<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a * b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::div`].
    #[inline(always)]
    fn vec_div(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Div<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a / b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::rem`].
    #[inline(always)]
    fn vec_rem(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Rem<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a % b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::shl`].
    #[inline(always)]
    fn vec_shl(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Shl<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a << b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::shr`].
    #[inline(always)]
    fn vec_shr(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Shr<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a >> b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::bitand`].
    #[inline(always)]
    fn vec_bitand(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: BitAnd<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a & b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::bitor`].
    #[inline(always)]
    fn vec_bitor(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: BitOr<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a | b)
            .as_storage()
    }

    /// Overridable implementation of [`Vector::bitxor`].
    #[inline(always)]
    fn vec_bitxor(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: BitXor<Output = Self>,
    {
        vec.as_nonsimd()
            .zip(rhs.as_nonsimd())
            .map(|(a, b)| a ^ b)
            .as_storage()
    }
}

/// Sealed trait implemented by [`Simd`] and [`NonSimd`].
/// This trait is used to mark vectors as either SIMD-optimized, or not.
pub trait Simdness: Sealed + 'static {
    /// Is true for [`Simd`] and false for [`NonSimd`].
    const IS_SIMD: bool;
}

/// Marker type for SIMD vectors.
pub struct Simd;

/// Marker type for non-SIMD vectors.
pub struct NonSimd;

impl<const N: usize, T: Scalar<N, S>, S: Simdness> Vector<N, T, S> {
    /// Creates a vector from an array.
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{vec2, Vec2};
    ///
    /// assert_eq!(Vec2::from_array([1.0, 2.0]), vec2!(1.0, 2.0));
    /// ```
    #[inline(always)]
    pub fn from_array(array: [T; N]) -> Self {
        T::vec_from_array(array)
    }

    /// Creates a vector by calling a function for each element.
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{vec4, Vec4};
    ///
    /// assert_eq!(Vec4::from_fn(|i| i as f32), vec4!(0.0, 1.0, 2.0, 3.0));
    /// ```
    #[inline(always)]
    pub fn from_fn(f: impl FnMut(usize) -> T) -> Self {
        Vector::from_array(core::array::from_fn(f))
    }

    /// Creates a vector from an array in a const context.
    /// This is slower than [`Vector::from_array`] and should only be used for const evaluation.
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{Vec3};
    ///
    /// const CONST_VECTOR: Vec3<f32> = Vec3::const_from_array([1.0, 2.0, 3.0]);
    /// ```
    #[inline(always)]
    pub const fn const_from_array(array: [T; N]) -> Self {
        if let Some(padding) = T::VECTOR_PADDING {
            let mut output = padding;
            *output.as_mut_array() = array;

            output
        } else {
            let inner_vector = unsafe {
                // SAFETY:
                // Because T::VECTOR_PADDING is None, we know that T::InnerVectorType has the memory layout of [T; N]
                transmute_copy::<[T; N], T::InnerVectorType>(&array)
            };

            Self(inner_vector)
        }
    }

    /// Converts a vector to an array.
    #[inline(always)]
    pub fn as_array(self) -> [T; N] {
        T::vec_as_array(self)
    }

    /// Converts a vector reference to an array reference.
    #[inline(always)]
    pub const fn as_array_ref(&self) -> &[T; N] {
        // SAFETY: It is guaranteed that Vector<N, T, S> starts with N T elements in memory
        unsafe { transmute::<&Vector<N, T, S>, &[T; N]>(self) }
    }

    /// Converts a mutable vector reference to a mutable array reference.
    #[inline(always)]
    pub const fn as_mut_array(&mut self) -> &mut [T; N] {
        // SAFETY: It is guaranteed that Vector<N, T, S> starts with N T elements in memory
        unsafe { transmute::<&mut Vector<N, T, S>, &mut [T; N]>(self) }
    }

    /// Converts a vector to a vector with a different "simdness".
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{NonSimd, vec3, Vec3, Vec3S};
    ///
    /// let simd_vec: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
    /// let nonsimd_vec: Vec3S<f32> = simd_vec.as_storage::<NonSimd>();
    /// ```
    #[inline(always)]
    pub fn as_storage<S2: Simdness>(self) -> Vector<N, T, S2>
    where
        T: Scalar<N, S2>,
    {
        Vector::from_array(self.as_array())
    }

    /// Converts a vector to a SIMD vector.
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{vec3s, Vec3, Vec3S};
    ///
    /// let nonsimd_vec: Vec3S<f32> = vec3s!(1.0, 2.0, 3.0);
    /// let simd_vec: Vec3<f32> = nonsimd_vec.as_simd();
    /// ```
    #[inline(always)]
    pub fn as_simd(self) -> Vector<N, T, Simd>
    where
        T: Scalar<N, Simd>,
    {
        self.as_storage::<Simd>()
    }

    /// Converts a vector to a non-SIMD vector.
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{vec3, Vec3, Vec3S};
    ///
    /// let simd_vec: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
    /// let nonsimd_vec: Vec3S<f32> = simd_vec.as_nonsimd();
    /// ```
    #[inline(always)]
    pub fn as_nonsimd(self) -> Vector<N, T, NonSimd> {
        self.as_storage::<NonSimd>()
    }

    /// Returns the number of elements in the vector.
    #[inline(always)]
    pub const fn len(self) -> usize {
        N
    }

    /// Returns true for [`Simd`] vectors and false for [`NonSimd`] vectors.
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{vec3, vec3s, Vec3, Vec3S};
    ///
    /// let simd_vec: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
    /// let nonsimd_vec: Vec3S<f32> = vec3s!(1.0, 2.0, 3.0);
    ///
    /// assert!(simd_vec.is_simd());
    /// assert!(!nonsimd_vec.is_simd());
    /// ```
    #[inline(always)]
    pub fn is_simd(self) -> bool {
        S::IS_SIMD
    }

    /// Returns the element at the given index, or `None` if the index is out of bounds.
    #[inline(always)]
    pub fn get(self, index: usize) -> Option<T> {
        if index < N {
            // SAFETY: index is in bounds
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index, or `None` if the index is out of bounds.
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < N {
            // SAFETY: index is in bounds
            Some(unsafe { self.as_mut_array().get_unchecked_mut(index) })
        } else {
            None
        }
    }

    /// Returns the element at the given index, without checking if the index is in bounds.
    ///
    /// ## Safety
    ///
    /// Calling this function with an out of bounds index is undefined behavior.
    #[inline(always)]
    pub unsafe fn get_unchecked(self, index: usize) -> T {
        // SAFETY: index is in bounds
        unsafe { *self.as_array_ref().get_unchecked(index) }
    }

    /// Returns a mutable reference to the element at the given index, without checking if the index is in bounds.
    ///
    /// ## Safety
    ///
    /// Calling this function with an out of bounds index is undefined behavior.
    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        // SAFETY: index is in bounds
        unsafe { self.as_mut_array().get_unchecked_mut(index) }
    }

    /// Returns an iterator over the elements of the vector.
    #[inline(always)]
    pub fn iter(self) -> IntoIter<T, N> {
        self.as_array().into_iter()
    }

    /// Returns a mutable iterator over the elements of the vector.
    #[inline(always)]
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        self.as_mut_array().iter_mut()
    }

    /// Maps a vector to a new vector by applying a function to each element.
    #[inline(always)]
    pub fn map<T2: Scalar<N, S>>(self, f: impl FnMut(T) -> T2) -> Vector<N, T2, S> {
        Vector::from_array(self.as_array().map(f))
    }

    /// Creates a vector where each element is a tuple of the corresponding elements of the two input vectors.
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{vec3, Vec3};
    ///
    /// let vec_a: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
    /// let vec_b: Vec3<f32> = vec3!(4.0, 5.0, 6.0);
    /// let vec_c: Vec3<(f32, f32)> = vec_a.zip(vec_b);
    ///
    /// assert_eq!(vec_c, vec3!((1.0, 4.0), (2.0, 5.0), (3.0, 6.0)));
    /// ```
    #[inline(always)]
    pub fn zip<T2: Scalar<N, S>>(self, other: Vector<N, T2, S>) -> Vector<N, (T, T2), S>
    where
        (T, T2): Scalar<N, S>,
    {
        // SAFETY: index is in bounds because we know that all vectors have the same length
        unsafe { Vector::from_fn(|i| (self.get_unchecked(i), other.get_unchecked(i))) }
    }

    /// Creates a vector with the elements in reverse order.
    ///
    /// ## Example
    ///
    /// ```
    /// use ggmath::{vec3, Vec3};
    ///
    /// let vec: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
    /// let reversed_vec: Vec3<f32> = vec.reverse();
    ///
    /// assert_eq!(reversed_vec, vec3!(3.0, 2.0, 1.0));
    /// ```
    #[inline(always)]
    pub fn reverse(self) -> Self {
        T::vec_reverse(self)
    }

    /// Returns a vector2 with the elements at the given indices which are known at compile time.
    pub fn get_const_vec2<const X_SRC: usize, const Y_SRC: usize>(self) -> Vector<2, T, S>
    where
        T: Scalar<2, S>,
    {
        const {
            assert!(X_SRC < N, "X Index out of bounds");
            assert!(Y_SRC < N, "Y Index out of bounds");
        }

        // SAFETY: indices are in bounds
        unsafe { T::vec_get_const_vec2::<X_SRC, Y_SRC>(self) }
    }

    /// Returns a vector3 with the elements at the given indices which are known at compile time.
    pub fn get_const_vec3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        self,
    ) -> Vector<3, T, S>
    where
        T: Scalar<3, S>,
    {
        const {
            assert!(X_SRC < N, "X Index out of bounds");
            assert!(Y_SRC < N, "Y Index out of bounds");
            assert!(Z_SRC < N, "Z Index out of bounds");
        }

        // SAFETY: indices are in bounds
        unsafe { T::vec_get_const_vec3::<X_SRC, Y_SRC, Z_SRC>(self) }
    }

    /// Returns a vector4 with the elements at the given indices which are known at compile time.
    pub fn get_const_vec4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        self,
    ) -> Vector<4, T, S>
    where
        T: Scalar<4, S>,
    {
        const {
            assert!(X_SRC < N, "X Index out of bounds");
            assert!(Y_SRC < N, "Y Index out of bounds");
            assert!(Z_SRC < N, "Z Index out of bounds");
            assert!(W_SRC < N, "W Index out of bounds");
        }

        // SAFETY: indices are in bounds
        unsafe { T::vec_get_const_vec4::<X_SRC, Y_SRC, Z_SRC, W_SRC>(self) }
    }
}

impl<const N: usize, T: Scalar<N, S>, S: Simdness> Clone for Vector<N, T, S> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T: Scalar<N, S>, S: Simdness> Copy for Vector<N, T, S> {}

impl<const N: usize, T: Scalar<N, S>, S: Simdness> Index<usize> for Vector<N, T, S> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_array_ref()[index]
    }
}

impl<const N: usize, T: Scalar<N, S>, S: Simdness> IndexMut<usize> for Vector<N, T, S> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_array()[index]
    }
}

impl<const N: usize, T: Scalar<N, S>, S: Simdness> IntoIterator for Vector<N, T, S> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize, T: Scalar<N, S>, S: Simdness> IntoIterator for &'a mut Vector<N, T, S> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize, T: PartialEq + Scalar<N, S>, S: Simdness> PartialEq for Vector<N, T, S> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        T::vec_eq(*self, *other)
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        T::vec_ne(*self, *other)
    }
}

impl<const N: usize, T: Eq + Scalar<N, S>, S: Simdness> Eq for Vector<N, T, S> {}

impl<const N: usize, T: Debug + Scalar<N, S>, S: Simdness> Debug for Vector<N, T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{:?}", self[i])?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl<const N: usize, T: Display + Scalar<N, S>, S: Simdness> Display for Vector<N, T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", self[i])?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl<const N: usize, T: Scalar<N, Simd>> Scalar<N, NonSimd> for T {
    type InnerVectorType = [T; N];
}

impl<const N: usize, T: Construct, const N2: usize> Scalar<N, Simd> for [T; N2] {
    type InnerVectorType = [[T; N2]; N];
}

impl<const N: usize> Scalar<N, Simd> for () {
    type InnerVectorType = [(); N];
}

impl<const N: usize, T0: Construct> Scalar<N, Simd> for (T0,) {
    type InnerVectorType = [(T0,); N];
}

impl<const N: usize, T0: Construct, T1: Construct> Scalar<N, Simd> for (T0, T1) {
    type InnerVectorType = [(T0, T1); N];
}

impl<const N: usize, T0: Construct, T1: Construct, T2: Construct> Scalar<N, Simd> for (T0, T1, T2) {
    type InnerVectorType = [(T0, T1, T2); N];
}

impl<const N: usize, T0: Construct, T1: Construct, T2: Construct, T3: Construct> Scalar<N, Simd>
    for (T0, T1, T2, T3)
{
    type InnerVectorType = [(T0, T1, T2, T3); N];
}

impl<const N: usize, T: Construct> Scalar<N, Simd> for Option<T> {
    type InnerVectorType = [Option<T>; N];
}

// SAFETY:
// [T; N] starts with exactly N Ts which guarantees:
// - that it does exactly that (requirement of InnerVectorType)
// - that it does not have padding
unsafe impl<const N: usize, T: Construct> InnerVectorType<N, T> for [T; N] {
    const PADDING: Option<Self> = None;
}

impl Simdness for Simd {
    const IS_SIMD: bool = true;
}
impl Simdness for NonSimd {
    const IS_SIMD: bool = false;
}

#[doc(hidden)]
impl Sealed for Simd {}
#[doc(hidden)]
impl Sealed for NonSimd {}

const _VERIFY_VECTOR_IS_CONSTRUCT: () = {
    fn verify_t_is_construct<T: Construct>() {}

    fn _helper<const N: usize, T: Scalar<N, S>, S: Simdness>() {
        verify_t_is_construct::<Vector<N, T, S>>();
    }
};
