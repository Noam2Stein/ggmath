use core::{
    mem::{transmute, transmute_copy},
    ops::{Add, Mul, Neg, Sub},
};

use crate::{
    Usize,
    vector::{Scalar, VecAligned, VecAlignment, VecLen, VecLenEnum, VecPacked, Vector},
};

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// A constant with an unknown valid value.
    /// Can be used to initialize temporary vectors.
    pub const GARBAGE: Self = match A::IS_ALIGNED {
        true => match Usize::<N>::ENUM {
            VecLenEnum::Two => unsafe {
                transmute_copy::<Vector<2, T, VecAligned>, Vector<N, T, A>>(&Vector::<
                    2,
                    T,
                    VecAligned,
                >(
                    T::INNER_ALIGNED_VEC2_GARBAGE,
                ))
            },
            VecLenEnum::Three => unsafe {
                transmute_copy::<Vector<3, T, VecAligned>, Vector<N, T, A>>(&Vector::<
                    3,
                    T,
                    VecAligned,
                >(
                    T::INNER_ALIGNED_VEC3_GARBAGE,
                ))
            },
            VecLenEnum::Four => unsafe {
                transmute_copy::<Vector<4, T, VecAligned>, Vector<N, T, A>>(&Vector::<
                    4,
                    T,
                    VecAligned,
                >(
                    T::INNER_ALIGNED_VEC4_GARBAGE,
                ))
            },
        },
        false => unsafe {
            transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(&Vector([T::GARBAGE; N]))
        },
    };

    /// Creates a new vector from an array.
    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        let mut output = Self::GARBAGE;

        *output.as_array_mut() = array;

        output
    }

    /// Converts the vector to an array.
    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        unsafe { transmute_copy::<Vector<N, T, A>, [T; N]>(&self) }
    }

    /// Converts a vector reference to an array reference.
    #[inline(always)]
    pub const fn as_array(&self) -> &[T; N] {
        unsafe { transmute::<&Vector<N, T, A>, &[T; N]>(self) }
    }

    /// Converts a mutable vector reference to a mutable array reference.
    #[inline(always)]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute::<&mut Vector<N, T, A>, &mut [T; N]>(self) }
    }

    /// Returns a pointer to the first element of the vector.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.as_array().as_ptr()
    }

    /// Returns a mutable pointer to the first element of the vector.
    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.as_array_mut().as_mut_ptr()
    }

    /// Creates a new vector where each component is evaluated from the given function called with the component index.
    /// The function is called in order.
    #[inline(always)]
    pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self
    where
        Usize<N>: VecLen,
    {
        Vector::from_array(core::array::from_fn(f))
    }

    /// Maps each component of the vector to a new value using the given function.
    #[inline(always)]
    pub fn map<T2: Scalar, F: Fn(T) -> T2>(self, f: F) -> Vector<N, T2, A>
    where
        Usize<N>: VecLen,
    {
        Vector::from_array(self.as_array().map(f))
    }

    /// Folds the vector into a single value using the given function.
    /// The function is applied on the output for each component in order.
    #[inline(always)]
    pub fn fold(self, mut f: impl FnMut(T, T) -> T) -> T {
        let mut output = self[0];

        for i in 1..N {
            output = f(output, self[i]);
        }

        output
    }

    /// Returns true if all components of the vector satisfy the given predicate.
    /// If a component does not satisfy the predicate,
    /// the function returns false immediately without evaluating the remaining components.
    #[inline(always)]
    pub fn all(self, f: impl FnMut(T) -> bool) -> bool {
        self.into_iter().all(f)
    }

    /// Returns true if any component of the vector satisfies the given predicate.
    /// If a component satisfies the predicate,
    /// the function returns true immediately without evaluating the remaining components.
    #[inline(always)]
    pub fn any(self, f: impl FnMut(T) -> bool) -> bool {
        self.into_iter().any(f)
    }

    /// Returns the number of components that satisfy the given predicate.
    #[inline(always)]
    pub fn count(self, mut f: impl FnMut(T) -> bool) -> usize {
        self.into_iter().filter(|x| f(*x)).count()
    }

    /// Creates a new vector where each component is the same value.
    #[inline(always)]
    pub fn splat(value: T) -> Self {
        T::vec_splat(value)
    }

    /// Converts the vector to an aligned vector.
    #[inline(always)]
    pub const fn align(self) -> Vector<N, T, VecAligned> {
        Vector::from_array(self.to_array())
    }

    /// Converts the vector to a packed vector.
    #[inline(always)]
    pub const fn pack(self) -> Vector<N, T, VecPacked> {
        Vector::from_array(self.to_array())
    }

    /// Converts the vector to the specified alignment.
    #[inline(always)]
    pub const fn to_storage<A2: VecAlignment>(self) -> Vector<N, T, A2> {
        Vector::from_array(self.to_array())
    }

    /// Returns the number of components in the vector.
    #[inline(always)]
    pub const fn len(self) -> usize {
        N
    }

    /// Returns true if the vector is aligned.
    /// The output is strictly determined by the type of the vector.
    #[inline(always)]
    pub const fn is_aligned(self) -> bool {
        A::IS_ALIGNED
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are equal.
    #[inline(always)]
    pub fn eq_mask<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_eq_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are not equal.
    #[inline(always)]
    pub fn ne_mask<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_ne_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are less than the corresponding component of the other vector.
    #[inline(always)]
    pub fn lt_mask<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_lt_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are less than or equal to the corresponding component of the other vector.
    #[inline(always)]
    pub fn le_mask<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_le_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are greater than the corresponding component of the other vector.
    #[inline(always)]
    pub fn gt_mask<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_gt_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are greater than or equal to the corresponding component of the other vector.
    #[inline(always)]
    pub fn ge_mask<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_ge_mask(self, other)
    }

    /// Returns a vector with the smaller elements between `self` and `other`.
    #[inline(always)]
    pub fn min(self, other: Vector<N, T, impl VecAlignment>) -> Self
    where
        Usize<N>: VecLen,
        T: PartialOrd,
    {
        T::vec_min(self, other)
    }

    /// Returns a vector with the larger elements between `self` and `other`.
    #[inline(always)]
    pub fn max(self, other: Vector<N, T, impl VecAlignment>) -> Self
    where
        Usize<N>: VecLen,
        T: PartialOrd,
    {
        T::vec_max(self, other)
    }

    /// Returns a vector with the elements of `self` clamped between `min` and `max`.
    #[inline(always)]
    pub fn clamp(
        self,
        min: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self
    where
        Usize<N>: VecLen,
        T: PartialOrd,
    {
        T::vec_clamp(self, min, max)
    }

    /// Sums the components of the vector.
    #[inline(always)]
    pub fn sum(self) -> T
    where
        Usize<N>: VecLen,
        T: Add<Output = T>,
    {
        T::vec_sum(self)
    }

    /// Multiplies the components of the vector.
    #[inline(always)]
    pub fn product(self) -> T
    where
        Usize<N>: VecLen,
        T: Mul<Output = T>,
    {
        T::vec_product(self)
    }

    /// Returns the squared magnitude of the vector.
    /// This is faster than `mag` because it avoids a square root operation.
    ///
    /// Note that `mag` only exists for specific primitives and not as a generic method.
    #[inline(always)]
    pub fn mag_sq(self) -> T
    where
        Usize<N>: VecLen,
        T: Add<Output = T> + Mul<Output = T>,
    {
        T::vec_mag_sq(self)
    }

    /// Returns the dot product of the vector and another vector.
    #[inline(always)]
    pub fn dot<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> T
    where
        Usize<N>: VecLen,
        T: Mul<T2, Output = T> + Add<Output = T>,
    {
        T::vec_dot(self, other)
    }

    /// Returns the absolute difference between the vector and another vector.
    #[inline(always)]
    pub fn abs_diff(self, other: Vector<N, T, impl VecAlignment>) -> Self
    where
        T: PartialOrd + Sub<Output = T>,
    {
        T::vec_abs_diff(self, other)
    }

    /// Returns the squared distance between the vector and another vector.
    #[inline(always)]
    pub fn distance_sq(self, other: Vector<N, T, impl VecAlignment>) -> T
    where
        T: PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        T::vec_distance_sq(self, other)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Version of `Vector::splat` that can be called from const contexts.
    /// This version may be less performant than the normal version.
    ///
    /// When rust's const capabilities are expanded, this function will be removed.
    #[inline(always)]
    pub const fn const_splat(value: T) -> Self {
        Vector::from_array([value; N])
    }
}

impl<const N: usize, T: Scalar> Vector<N, T, VecPacked>
where
    Usize<N>: VecLen,
{
    /// Converts an array reference to a vector reference.
    ///
    /// This requires `VecPacked` alignment because a SIMD aligned vector reference is incompatible with an array reference.
    #[inline(always)]
    pub const fn from_array_ref(array: &[T; N]) -> &Self {
        unsafe { transmute::<&[T; N], &Vector<N, T, VecPacked>>(array) }
    }

    /// Converts a mutable array reference to a mutable vector reference.
    ///
    /// This requires `VecPacked` alignment because a SIMD aligned vector reference is incompatible with an array reference.
    #[inline(always)]
    pub const fn from_array_mut(array: &mut [T; N]) -> &mut Self {
        unsafe { transmute::<&mut [T; N], &mut Vector<N, T, VecPacked>>(array) }
    }
}

impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns `self` rotated 90 degrees counter-clockwise.
    #[inline(always)]
    pub fn perp(self) -> Self
    where
        T: Neg<Output = T>,
    {
        T::vec_perp(self)
    }

    /// Returns `self` rotated 90 degrees clockwise.
    #[inline(always)]
    pub fn perp_clockwise(self) -> Self
    where
        T: Neg<Output = T>,
    {
        T::vec_perp_clockwise(self)
    }
}

impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns the cross product of the vector and another vector.
    #[inline(always)]
    pub fn cross(self, other: Vector<3, T, impl VecAlignment>) -> Self
    where
        T: Mul<T, Output = T>,
        T: Sub<T, Output = T>,
    {
        T::vec_cross(self, other)
    }
}
