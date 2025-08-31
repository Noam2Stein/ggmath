use std::{
    mem::{transmute, transmute_copy},
    ops::{Add, Mul, Sub},
    ptr::copy_nonoverlapping,
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
                > {
                    inner: T::INNER_ALIGNED_VEC2_GARBAGE,
                })
            },
            VecLenEnum::Three => unsafe {
                transmute_copy::<Vector<3, T, VecAligned>, Vector<N, T, A>>(&Vector::<
                    3,
                    T,
                    VecAligned,
                > {
                    inner: T::INNER_ALIGNED_VEC3_GARBAGE,
                })
            },
            VecLenEnum::Four => unsafe {
                transmute_copy::<Vector<4, T, VecAligned>, Vector<N, T, A>>(&Vector::<
                    4,
                    T,
                    VecAligned,
                > {
                    inner: T::INNER_ALIGNED_VEC4_GARBAGE,
                })
            },
        },
        false => unsafe {
            transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(&Vector {
                inner: [T::GARBAGE; N],
            })
        },
    };

    /// Creates a new vector from an array.
    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        let mut output = Self::GARBAGE;

        unsafe {
            copy_nonoverlapping::<T>(array.as_ptr(), output.as_mut_ptr(), N);
        }

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
        Vector::from_array(std::array::from_fn(f))
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
    pub fn all(self, mut f: impl FnMut(T) -> bool) -> bool {
        for i in 0..N {
            if !f(self[i]) {
                return false;
            }
        }

        true
    }

    /// Returns true if any component of the vector satisfies the given predicate.
    /// If a component satisfies the predicate,
    /// the function returns true immediately without evaluating the remaining components.
    #[inline(always)]
    pub fn any(self, mut f: impl FnMut(T) -> bool) -> bool {
        for i in 0..N {
            if f(self[i]) {
                return true;
            }
        }

        false
    }

    /// Creates a new vector where each component is the same value.
    #[inline(always)]
    pub const fn splat(value: T) -> Self {
        Vector::from_array([value; N])
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

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are equal.
    #[inline(always)]
    pub fn eq_mask<T2: Scalar>(
        &self,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_eq_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are not equal.
    #[inline(always)]
    pub fn ne_mask<T2: Scalar>(
        &self,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_ne_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are less than the corresponding component of the other vector.
    #[inline(always)]
    pub fn lt_mask<T2: Scalar>(
        &self,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_lt_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are less than or equal to the corresponding component of the other vector.
    #[inline(always)]
    pub fn le_mask<T2: Scalar>(
        &self,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_le_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are greater than the corresponding component of the other vector.
    #[inline(always)]
    pub fn gt_mask<T2: Scalar>(
        &self,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_gt_mask(self, other)
    }

    /// Compares each component of the vector to the corresponding component of another vector and returns a vector of bools indicating if the components are greater than or equal to the corresponding component of the other vector.
    #[inline(always)]
    pub fn ge_mask<T2: Scalar>(
        &self,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_ge_mask(self, other)
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
    pub fn dot<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> T::Output
    where
        Usize<N>: VecLen,
        T: Mul<T2, Output: Scalar>,
        T::Output: Add<Output = T::Output>,
    {
        T::vec_dot(self, other)
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

#[cfg(test)]
mod tests {
    use crate::{vec2, vec2p, vec3, vec3p, vec4, vec4p};

    #[test]
    fn test_swizzle() {
        assert_eq!(vec2!(1, 2).yx(), vec2!(2, 1));
        assert_eq!(vec2!(1, 2).yxy(), vec3!(2, 1, 2));
        assert_eq!(vec2!(1, 2).yxyy(), vec4!(2, 1, 2, 2));
        assert_eq!(vec3!(1, 2, 3).yz(), vec2!(2, 3));
        assert_eq!(vec3!(1, 2, 3).yxz(), vec3!(2, 1, 3));
        assert_eq!(vec3!(1, 2, 3).yxyz(), vec4!(2, 1, 2, 3));
        assert_eq!(vec4!(1, 2, 3, 4).yx(), vec2!(2, 1));
        assert_eq!(vec4!(1, 2, 3, 4).yxy(), vec3!(2, 1, 2));
        assert_eq!(vec4!(1, 2, 3, 4).yxww(), vec4!(2, 1, 4, 4));

        assert_eq!(vec2p!(1, 2).yx(), vec2!(2, 1));
        assert_eq!(vec2p!(1, 2).yxy(), vec3!(2, 1, 2));
        assert_eq!(vec2p!(1, 2).yxyy(), vec4!(2, 1, 2, 2));
        assert_eq!(vec3p!(1, 2, 3).yz(), vec2!(2, 3));
        assert_eq!(vec3p!(1, 2, 3).yxz(), vec3!(2, 1, 3));
        assert_eq!(vec3p!(1, 2, 3).yxyz(), vec4!(2, 1, 2, 3));
        assert_eq!(vec4p!(1, 2, 3, 4).yx(), vec2!(2, 1));
        assert_eq!(vec4p!(1, 2, 3, 4).yxy(), vec3!(2, 1, 2));
        assert_eq!(vec4p!(1, 2, 3, 4).yxww(), vec4!(2, 1, 4, 4));
    }
}
