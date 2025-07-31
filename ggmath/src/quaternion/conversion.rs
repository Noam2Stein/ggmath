use std::mem::transmute;

use super::*;

// T

impl<T: Scalar, A: VecAlignment> Quaternion<T, A> {
    /// Converts the quaternion to a different scalar type using the `From` trait.
    #[inline(always)]
    pub fn to_t<TOutput: Scalar + From<T>>(self) -> Quaternion<TOutput, A> {
        Quaternion {
            inner: self.inner.to_t::<TOutput>(),
        }
    }
}

// A

impl<T: Scalar, A: VecAlignment> Quaternion<T, A> {
    /// Aligns the quaternion to a [`VecAligned`] vector.
    #[inline(always)]
    pub const fn align(self) -> Quaternion<T, VecAligned> {
        Quaternion {
            inner: self.inner.align(),
        }
    }

    /// Unaligns the quaternion to a [`VecPacked`] vector.
    ///
    /// This is always a zero cost operation.
    #[inline(always)]
    pub const fn unalign(self) -> Quaternion<T, VecPacked> {
        Quaternion {
            inner: self.inner.unalign(),
        }
    }
}

impl<T: Scalar> From<Quaternion<T, VecAligned>> for Quaternion<T, VecPacked> {
    #[inline(always)]
    fn from(value: Quaternion<T, VecAligned>) -> Self {
        Quaternion {
            inner: value.inner.into(),
        }
    }
}

impl<T: Scalar> From<Quaternion<T, VecPacked>> for Quaternion<T, VecAligned> {
    #[inline(always)]
    fn from(value: Quaternion<T, VecPacked>) -> Self {
        Quaternion {
            inner: value.inner.into(),
        }
    }
}

// Layout

impl<T: Scalar, A: VecAlignment> Quaternion<T, A> {
    /// Converts the quaternion to the specified memory-layout generics.
    ///
    /// A quaternion's memory layout generics is just its alignment.
    #[inline(always)]
    pub const fn to_layout<A2: VecAlignment>(self) -> Quaternion<T, A2> {
        Quaternion {
            inner: self.inner.to_layout(),
        }
    }
}

// Array

impl<T: Scalar, A: VecAlignment> Quaternion<T, A> {
    /// Constructs a quaternion from an array.
    #[inline(always)]
    pub const fn from_array(array: [T; 4]) -> Self {
        Self {
            inner: Vector::from_array(array),
        }
    }

    /// Converts the quaternion to an array.
    #[inline(always)]
    pub const fn to_array(self) -> [T; 4] {
        self.inner.to_array()
    }

    // Ref

    /// Returns a reference to the quaternion as an array.
    pub const fn as_array_ref(&self) -> &[T; 4] {
        self.inner.as_array_ref()
    }

    // Mut

    /// Returns a mutable reference to the quaternion as an array.
    pub const fn as_array_mut(&mut self) -> &mut [T; 4] {
        self.inner.as_array_mut()
    }
}

impl<T: Scalar> Quaternion<T, VecPacked> {
    // Ref

    /// Casts an array reference to a quaternion reference.
    ///
    /// This requires `VecPacked` alignment to guarantee that the type has the same memory layout as an array.
    #[inline(always)]
    pub const fn from_array_ref(array: &[T; 4]) -> &Self {
        Self::from_vec_ref(Vector::from_array_ref(array))
    }

    // Mut

    /// Casts an array mutable reference to a quaternion mutable reference.
    ///
    /// This requires `VecPacked` alignment to guarantee that the type has the same memory layout as an array.
    #[inline(always)]
    pub const fn from_array_mut(array: &mut [T; 4]) -> &mut Self {
        Self::from_vec_mut(Vector::from_array_mut(array))
    }
}

impl<T: Scalar, A: VecAlignment> From<[T; 4]> for Quaternion<T, A> {
    #[inline(always)]
    fn from(array: [T; 4]) -> Self {
        Self::from_array(array)
    }
}

impl<T: Scalar, A: VecAlignment> From<Quaternion<T, A>> for [T; 4] {
    #[inline(always)]
    fn from(value: Quaternion<T, A>) -> Self {
        value.to_array()
    }
}

// Vector

impl<T: Scalar, A: VecAlignment> Quaternion<T, A> {
    /// Converts a vector to a quaternion.
    #[inline(always)]
    pub const fn from_vec(vector: Vector<4, T, A>) -> Self {
        Self { inner: vector }
    }

    /// Converts the quaternion to a vector.
    #[inline(always)]
    pub const fn to_vec(self) -> Vector<4, T, A> {
        self.inner
    }

    // Ref

    /// Casts a vector reference to a quaternion reference.
    #[inline(always)]
    pub const fn from_vec_ref(vector: &Vector<4, T, A>) -> &Self {
        unsafe { transmute::<&Vector<4, T, A>, &Quaternion<T, A>>(vector) }
    }

    /// Returns a reference to the quaternion as a vector reference.
    #[inline(always)]
    pub const fn as_vec_ref(&self) -> &Vector<4, T, A> {
        &self.inner
    }

    // Mut

    /// Casts a vector mutable reference to a quaternion mutable reference.
    #[inline(always)]
    pub const fn from_vec_mut(vector: &mut Vector<4, T, A>) -> &mut Self {
        unsafe { transmute::<&mut Vector<4, T, A>, &mut Quaternion<T, A>>(vector) }
    }

    /// Returns a mutable reference to the quaternion as a vector mutable reference.
    #[inline(always)]
    pub const fn as_vec_mut(&mut self) -> &mut Vector<4, T, A> {
        &mut self.inner
    }
}
