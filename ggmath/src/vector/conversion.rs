use std::{
    mem::transmute,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use super::*;

// T

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Converts the vector to a new vector with a different scalar type using the `From` trait.
    #[inline(always)]
    pub fn to_t<T2: Scalar + From<T>>(self) -> Vector<N, T2, A> {
        self.map(T2::from)
    }
}

// A

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Aligns the vector.
    /// - Cost: Nothing if `self` is already aligned. If `self` is packed, moves the vector to satisfy the alignment.
    #[inline(always)]
    pub const fn align(self) -> Vector<N, T, VecAligned> {
        self.to_layout()
    }

    /// "Unaligns" the vector.
    /// This just means casting the vector from `VecAligned` to `VecPacked`.
    ///
    /// This has no cost since an aligned vector always satisfies packed alignment.
    #[inline(always)]
    pub const fn unalign(self) -> Vector<N, T, VecPacked> {
        self.to_layout()
    }
}

impl<const N: usize, T: Scalar> From<Vector<N, T, VecAligned>> for Vector<N, T, VecPacked>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn from(value: Vector<N, T, VecAligned>) -> Self {
        value.unalign()
    }
}

impl<const N: usize, T: Scalar> From<Vector<N, T, VecPacked>> for Vector<N, T, VecAligned>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn from(value: Vector<N, T, VecPacked>) -> Self {
        value.align()
    }
}

// Layout

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Converts the vector into the specified memory-layout generics.
    /// In the case of a vector, the only memory-layout generics it has is its alignment.
    #[inline(always)]
    pub const fn to_layout<A2: VecAlignment>(self) -> Vector<N, T, A2> {
        Vector::from_array(self.to_array())
    }
}

// Array

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Creates a vector from an array.
    ///
    /// Cost:
    /// If `A` is `VecAligned`, this will perform a copy instruction to align the vector.
    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        Self {
            array,
            _align: <A::Alignment<N, T> as AlignTrait>::VALUE,
        }
    }

    /// Converts the vector into an array.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        self.array
    }

    /// Referecnes the vector as an array.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn as_array_ref(&self) -> &[T; N] {
        &self.array
    }

    /// Mutably referecnes the vector as an array.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        &mut self.array
    }

    /// Returns a pointer to the vector's buffer.
    ///
    /// Cost: nothing.
    pub const fn as_ptr(&self) -> *const T {
        self.as_array_ref().as_ptr()
    }

    /// Returns a mutable pointer to the vector's buffer.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.as_array_mut().as_mut_ptr()
    }
}

impl<const N: usize, T: Scalar> Vector<N, T, VecPacked>
where
    Usize<N>: VecLen,
{
    /// Casts an array reference to a vector reference.
    ///
    /// This requires `VecPacked` alignment,
    /// because it guarentees the same type-layout as an array,
    /// where as a `VecAligned` vector might have a larger size than an array.
    #[inline(always)]
    pub const fn from_array_ref(array: &[T; N]) -> &Self {
        unsafe { transmute::<&[T; N], &Self>(array) }
    }

    /// Casts an array mutably reference to a vector mutably reference.
    ///
    /// This requires `VecPacked` alignment,
    /// because it guarentees the same type-layout as an array,
    /// where as a `VecAligned` vector might have a larger size than an array.
    #[inline(always)]
    pub const fn from_array_mut(array: &mut [T; N]) -> &mut Self {
        unsafe { transmute::<&mut [T; N], &mut Self>(array) }
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> From<[T; N]> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn from(array: [T; N]) -> Self {
        Self::from_array(array)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> From<Vector<N, T, A>> for [T; N]
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn from(vector: Vector<N, T, A>) -> Self {
        vector.to_array()
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> AsRef<[T; N]> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn as_ref(&self) -> &[T; N] {
        self.as_array_ref()
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> AsRef<[T]> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.as_array_ref()
    }
}

// Bytes

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Referecnes the vector as a byte array, without padding.
    pub const fn as_bytes_ref(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;

        unsafe { from_raw_parts(ptr, size_of::<T>() * N) }
    }

    /// Mutably referecnes the vector as a byte array, without padding.
    ///
    /// SAFETY: `T` might not be a valid `[u8]`, which means the vector could become corrupted.
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self.as_mut_ptr() as *mut u8;

        unsafe { from_raw_parts_mut(ptr, size_of::<T>() * N) }
    }

    /// Referecnes the vector as a byte array, with padding.
    pub const fn as_bytes_ref_padded(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;

        unsafe { from_raw_parts(ptr, size_of::<Self>()) }
    }

    /// Mutably referecnes the vector as a byte array, with padding.
    ///
    /// SAFETY: `T` might not be a valid `[u8]`, which means the vector could become corrupted.
    pub const unsafe fn as_bytes_mut_padded(&mut self) -> &mut [u8] {
        let ptr = self.as_mut_ptr() as *mut u8;

        unsafe { from_raw_parts_mut(ptr, size_of::<Self>()) }
    }
}
