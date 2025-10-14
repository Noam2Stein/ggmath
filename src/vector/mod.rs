//! A module with vector related items

use std::array::IntoIter;
use std::fmt::{Debug, Display};
use std::mem::{ManuallyDrop, transmute, transmute_copy};
use std::ops::{
    Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Rem, Shl, Shr, Sub,
};
use std::slice::IterMut;

use crate::match_simdness;
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
pub struct Vector<const N: usize, T: Scalar<N>, S: Simdness>(pub S::InnerVectorType<N, T>);

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

/// TODO
pub unsafe trait TransmuteTo<T>: Sized {
    /// Safely transmutes `self` to `T`.
    fn transmute_to(self) -> T {
        const { assert!(size_of::<Self>() == size_of::<T>()) }
        const { assert!(align_of::<Self>() >= align_of::<T>()) }

        let value = ManuallyDrop::new(self);

        unsafe { transmute_copy::<ManuallyDrop<Self>, T>(&value) }
    }
}

/// A trait for types that are safe to use as inner vector types for a given element type and number of elements.
pub unsafe trait InnerVectorType<const N: usize, T: Construct>: Construct {
    /// TODO
    const PADDING: Option<Self>;
}

/// A trait for elements of [`Vector`] of a given length.
/// This trait is nessesary to implement in order to use a type inside a vector.
///
/// ## SIMD
///
/// This trait controls:
/// - The inner type contained inside [`Vector<N, T, Simd>`]
/// - The implementations of [`Vector<N, T, Simd>`] functions
///
/// This allows implementations to add SIMD optimizations to vectors.
///
/// ## Example
///
/// TODO: add example
pub trait Scalar<const N: usize>: Construct {
    /// The inner type contained inside [`Vector<N, Self, S>`].
    type InnerSimdVectorType: InnerVectorType<N, Self>;

    /// Overridable implementation of [`Vector::from_array`].
    #[inline(always)]
    fn vec_from_array(array: [Self; N]) -> Vector<N, Self, Simd> {
        Vector::const_from_array(array)
    }

    /// Overridable implementation of [`Vector`] splat, which can be used through the [`vec2!`], [`vec3!`], etc. macros.
    #[inline(always)]
    fn vec_splat(value: Self) -> Vector<N, Self, Simd> {
        Vector::from_array([value; N])
    }

    /// Overridable implementation of [`Vector::as_array`].
    #[inline(always)]
    fn vec_as_array(vec: Vector<N, Self, Simd>) -> [Self; N] {
        *vec.as_array_ref()
    }

    /// Overridable implementation of [`Vector::reverse`].
    #[inline(always)]
    fn vec_reverse(vec: Vector<N, Self, Simd>) -> Vector<N, Self, Simd> {
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
        vec: Vector<N, Self, Simd>,
    ) -> Vector<2, Self, Simd>
    where
        Self: Scalar<2>,
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
        vec: Vector<N, Self, Simd>,
    ) -> Vector<3, Self, Simd>
    where
        Self: Scalar<3>,
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
        vec: Vector<N, Self, Simd>,
    ) -> Vector<4, Self, Simd>
    where
        Self: Scalar<4>,
    {
        vec4g!(vec[X_SRC], vec[Y_SRC], vec[Z_SRC], vec[W_SRC])
    }

    /// Overridable implementation of [`Vector::eq`].
    #[inline(always)]
    fn vec_eq(vec: Vector<N, Self, Simd>, other: Vector<N, Self, Simd>) -> bool
    where
        Self: PartialEq,
    {
        vec.as_array() == other.as_array()
    }

    /// Overridable implementation of [`Vector::ne`].
    #[inline(always)]
    fn vec_ne(vec: Vector<N, Self, Simd>, other: Vector<N, Self, Simd>) -> bool
    where
        Self: PartialEq,
    {
        !(vec == other)
    }

    /// Overridable implementation of [`Vector::neg`].
    #[inline(always)]
    fn vec_neg(vec: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Neg<Output = Self>,
    {
        vec.map(Neg::neg)
    }

    /// Overridable implementation of [`Vector::not`].
    #[inline(always)]
    fn vec_not(vec: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Not<Output = Self>,
    {
        vec.map(Not::not)
    }

    /// Overridable implementation of [`Vector::add`].
    #[inline(always)]
    fn vec_add(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Add<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] + rhs[i])
    }

    /// Overridable implementation of [`Vector::sub`].
    #[inline(always)]
    fn vec_sub(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Sub<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] - rhs[i])
    }

    /// Overridable implementation of [`Vector::mul`].
    #[inline(always)]
    fn vec_mul(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Mul<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] * rhs[i])
    }

    /// Overridable implementation of [`Vector::div`].
    #[inline(always)]
    fn vec_div(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Div<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] / rhs[i])
    }

    /// Overridable implementation of [`Vector::rem`].
    #[inline(always)]
    fn vec_rem(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Rem<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] % rhs[i])
    }

    /// Overridable implementation of [`Vector::shl`].
    #[inline(always)]
    fn vec_shl(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Shl<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] << rhs[i])
    }

    /// Overridable implementation of [`Vector::shr`].
    #[inline(always)]
    fn vec_shr(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Shr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] >> rhs[i])
    }

    /// Overridable implementation of [`Vector::bitand`].
    #[inline(always)]
    fn vec_bitand(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: BitAnd<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] & rhs[i])
    }

    /// Overridable implementation of [`Vector::bitor`].
    #[inline(always)]
    fn vec_bitor(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: BitOr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] | rhs[i])
    }

    /// Overridable implementation of [`Vector::bitxor`].
    #[inline(always)]
    fn vec_bitxor(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: BitXor<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] ^ rhs[i])
    }
}

/// Sealed trait implemented by [`Simd`] and [`NonSimd`].
/// This trait is used to mark vectors as either SIMD-backed or not.
pub trait Simdness: Sealed + 'static {
    #[doc(hidden)]
    type InnerVectorType<const N: usize, T: Scalar<N>>: InnerVectorType<N, T>;

    /// Is true for [`Simd`] and false for [`NonSimd`].
    const IS_SIMD: bool;
}

/// Marker type for SIMD vectors.
pub struct Simd;

/// Marker type for non-SIMD vectors.
pub struct NonSimd;

impl<const N: usize, T: Scalar<N>, S: Simdness> Vector<N, T, S> {
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
        match_simdness! {
            for array;

            Simd => |array: [T; N]| -> Vector<N, T, Simd> {
                T::vec_from_array(array)
            },
            NonSimd => |array: [T; N]| -> Vector<N, T, NonSimd> {
                Vector(array)
            }
        }
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
        if let Some(padding) = S::InnerVectorType::PADDING {
            let mut output = Self(padding);
            *output.as_mut_array() = array;

            output
        } else {
            let inner_vector = unsafe {
                // SAFETY:
                // Because PADDING is None, we know that S::InnerVectorType has the memory layout of [T; N]
                transmute_copy::<[T; N], S::InnerVectorType<N, T>>(&array)
            };

            Self(inner_vector)
        }
    }

    /// Converts a vector to an array.
    #[inline(always)]
    pub fn as_array(self) -> [T; N] {
        match_simdness! {
            for self;

            Simd => |vec: Vector<N, T, Simd>| -> [T; N] {
                T::vec_as_array(vec)
            },
            NonSimd => |vec: Vector<N, T, NonSimd>| -> [T; N] {
                vec.0
            }
        }
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
    pub fn as_storage<S2: Simdness>(self) -> Vector<N, T, S2> {
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
    pub fn as_simd(self) -> Vector<N, T, Simd> {
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
    pub fn map<T2: Scalar<N>>(self, f: impl FnMut(T) -> T2) -> Vector<N, T2, S> {
        Vector::from_array(self.as_array().map(f))
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
        match_simdness! {
            for self;

            Simd => |vec: Vector<N, T, Simd>| -> Vector<N, T, Simd> {
                T::vec_reverse(vec)
            },
            NonSimd => |vec: Vector<N, T, NonSimd>| -> Vector<N, T, NonSimd> {
                Vector::from_fn(|i| vec[N - 1 - i])
            }
        }
    }

    /// Returns a vector2 with the elements at the given indices which are known at compile time.
    pub fn get_const_vec2<const X_SRC: usize, const Y_SRC: usize>(self) -> Vector<2, T, S>
    where
        T: Scalar<2>,
    {
        const {
            assert!(X_SRC < N, "X Index out of bounds");
            assert!(Y_SRC < N, "Y Index out of bounds");
        }

        match_simdness! {
            for self;

            Simd => |vec: Vector<N, T, Simd>| -> Vector<2, T, Simd> {
                // SAFETY: indices are in bounds
                unsafe { T::vec_get_const_vec2::<X_SRC, Y_SRC>(vec) }
            },
            NonSimd => |vec: Vector<N, T, NonSimd>| -> Vector<2, T, NonSimd> {
                vec2s!(vec[X_SRC], vec[Y_SRC])
            }
        }
    }

    /// Returns a vector3 with the elements at the given indices which are known at compile time.
    pub fn get_const_vec3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        self,
    ) -> Vector<3, T, S>
    where
        T: Scalar<3>,
    {
        const {
            assert!(X_SRC < N, "X Index out of bounds");
            assert!(Y_SRC < N, "Y Index out of bounds");
            assert!(Z_SRC < N, "Z Index out of bounds");
        }

        match_simdness! {
            for self;

            Simd => |vec: Vector<N, T, Simd>| -> Vector<3, T, Simd> {
                // SAFETY: indices are in bounds
                unsafe { T::vec_get_const_vec3::<X_SRC, Y_SRC, Z_SRC>(vec) }
            },
            NonSimd => |vec: Vector<N, T, NonSimd>| -> Vector<3, T, NonSimd> {
                vec3s!(vec[X_SRC], vec[Y_SRC], vec[Z_SRC])
            }
        }
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
        T: Scalar<4>,
    {
        const {
            assert!(X_SRC < N, "X Index out of bounds");
            assert!(Y_SRC < N, "Y Index out of bounds");
            assert!(Z_SRC < N, "Z Index out of bounds");
            assert!(W_SRC < N, "W Index out of bounds");
        }

        match_simdness! {
            for self;

            Simd => |vec: Vector<N, T, Simd>| -> Vector<4, T, Simd> {
                // SAFETY: indices are in bounds
                unsafe { T::vec_get_const_vec4::<X_SRC, Y_SRC, Z_SRC, W_SRC>(vec) }
            },
            NonSimd => |vec: Vector<N, T, NonSimd>| -> Vector<4, T, NonSimd> {
                vec4s!(vec[X_SRC], vec[Y_SRC], vec[Z_SRC], vec[W_SRC])
            }
        }
    }
}

impl<const N: usize, T: Scalar<N>, S: Simdness> Clone for Vector<N, T, S> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T: Scalar<N>, S: Simdness> Copy for Vector<N, T, S> {}

impl<const N: usize, T: Scalar<N>, S: Simdness> Index<usize> for Vector<N, T, S> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_array_ref()[index]
    }
}

impl<const N: usize, T: Scalar<N>, S: Simdness> IndexMut<usize> for Vector<N, T, S> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_array()[index]
    }
}

impl<const N: usize, T: Scalar<N>, S: Simdness> IntoIterator for Vector<N, T, S> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize, T: Scalar<N>, S: Simdness> IntoIterator for &'a mut Vector<N, T, S> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize, T: PartialEq + Scalar<N>, S: Simdness> PartialEq for Vector<N, T, S> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        match_simdness! {
            for *self, *other;

            Simd => |vec: Vector<N, T, Simd>, other: Vector<N, T, Simd>| -> bool {
                T::vec_eq(vec, other)
            },
            NonSimd => |vec: Vector<N, T, NonSimd>, other: Vector<N, T, NonSimd>| -> bool {
                (0..N).all(|i| vec[i] == other[i])
            }
        }
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        match_simdness! {
            for *self, *other;

            Simd => |vec: Vector<N, T, Simd>, other: Vector<N, T, Simd>| -> bool {
                T::vec_ne(vec, other)
            },
            NonSimd => |vec: Vector<N, T, NonSimd>, other: Vector<N, T, NonSimd>| -> bool {
                (0..N).any(|i| vec[i] != other[i])
            }
        }
    }
}

impl<const N: usize, T: Eq + Scalar<N>, S: Simdness> Eq for Vector<N, T, S> {}

impl<const N: usize, T: Debug + Scalar<N>, S: Simdness> Debug for Vector<N, T, S> {
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

impl<const N: usize, T: Display + Scalar<N>, S: Simdness> Display for Vector<N, T, S> {
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

impl<const N: usize, T: Construct> Scalar<N> for Option<T> {
    type InnerSimdVectorType = [Option<T>; N];
}

// SAFETY:
// [T; N] starts with exactly N Ts which guarantees:
// - that it does exactly that (requirement of InnerVectorType)
// - that it does not have padding
unsafe impl<const N: usize, T: Construct> InnerVectorType<N, T> for [T; N] {
    const PADDING: Option<Self> = None;
}

// SAFETY:
// Because T is transmutable to TInner, they share the same layout.
// Because of that, their vectors also share their layout.
unsafe impl<const N: usize, T: Construct + TransmuteTo<TInner>, TInner: Scalar<N>>
    InnerVectorType<N, T> for Vector<N, TInner, Simd>
{
    const PADDING: Option<Self> = match TInner::InnerSimdVectorType::PADDING {
        Some(padding) => Some(Self(padding)),
        None => None,
    };
}

impl Simdness for Simd {
    type InnerVectorType<const N: usize, T: Scalar<N>> = T::InnerSimdVectorType;

    const IS_SIMD: bool = true;
}

impl Simdness for NonSimd {
    type InnerVectorType<const N: usize, T: Scalar<N>> = [T; N];

    const IS_SIMD: bool = false;
}

#[doc(hidden)]
impl Sealed for Simd {}
#[doc(hidden)]
impl Sealed for NonSimd {}

const _VERIFY_VECTOR_IS_CONSTRUCT: () = {
    fn verify_t_is_construct<T: Construct>() {}

    fn _helper<const N: usize, T: Scalar<N>, S: Simdness>() {
        verify_t_is_construct::<Vector<N, T, S>>();
    }
};

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! match_simdness {
    { for $($arg:expr),*; Simd => $simd_closure:expr, NonSimd => $non_simd_closure:expr $(,)? } => {
        (const {
            if S::IS_SIMD {
                let closure: $crate::match_simdness!(@fn_ty $($arg),*) = $simd_closure;
                unsafe {
                    core::mem::transmute_copy::<
                        $crate::match_simdness!(@fn_ty $($arg),*),
                        $crate::match_simdness!(@fn_ty $($arg),*),
                    >(&closure)
                }
            } else {
                let closure: $crate::match_simdness!(@fn_ty $($arg),*) = $non_simd_closure;
                unsafe {
                    core::mem::transmute_copy::<
                        $crate::match_simdness!(@fn_ty $($arg),*),
                        $crate::match_simdness!(@fn_ty $($arg),*),
                    >(&closure)
                }
            }
        })
        ($($arg),*)
    };

    (@fn_ty) => {
        fn() -> _
    };
    (@fn_ty $_arg0:expr) => {
        fn(_) -> _
    };
    (@fn_ty $_arg0:expr, $_arg1:expr) => {
        fn(_, _) -> _
    };
    (@fn_ty $_arg0:expr, $_arg1:expr, $_arg2:expr) => {
        fn(_, _, _) -> _
    };
    (@fn_ty $_arg0:expr, $_arg1:expr, $_arg2:expr, $_arg3:expr) => {
        fn(_, _, _, _) -> _
    };
}
