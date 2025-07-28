use std::{
    mem::transmute,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use super::*;

// Array

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Creates a `Vector<N, T, A>` from a `[T; N]` by copying it.
    ///
    /// Cost: a copy if the vector type is aligned.
    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        Self {
            array,
            _align: <<A as VecAlignment>::Alignment<N, T> as AlignTrait>::VALUE,
        }
    }

    /// Creates a `[T; N]` from a `Vector<N, T, A>` by copying it.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        self.array
    }

    /// referecnes `self` as an array.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn as_array_ref(&self) -> &[T; N] {
        &self.array
    }

    /// mutably referecnes `self` as an array.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        &mut self.array
    }

    /// Returns a raw pointer to the vector's buffer.
    ///
    /// Cost: nothing.
    pub const fn as_ptr(&self) -> *const T {
        self.as_array_ref().as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    ///
    /// Cost: nothing.
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.as_array_mut().as_mut_ptr()
    }
}

impl<const N: usize, T: Scalar> Vector<N, T, VecPacked>
where
    Usize<N>: VecLen,
{
    /// Converts an array reference to a vector reference.
    ///
    /// This is only possible with `VecPacked` alignment,
    /// because it guarentees the same type-layout as an array,
    /// where as a `VecAligned` vector might have a larger size than an array.
    #[inline(always)]
    pub const fn from_array_ref(array: &[T; N]) -> &Self {
        unsafe { transmute(array) }
    }

    /// Converts a mutable array reference to a mutable vector reference.
    ///
    /// This is only possible with `VecPacked` alignment,
    /// because it guarentees the same type-layout as an array,
    /// where as a `VecAligned` vector might have a larger size than an array.
    #[inline(always)]
    pub const fn from_array_mut(array: &mut [T; N]) -> &mut Self {
        unsafe { transmute(array) }
    }
}

// Bytes

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// referecnes `self` as a byte array without the padding.
    ///
    /// * Can still contain garbage because `T` might have its own padding.
    pub const fn as_bytes_ref(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;

        unsafe { from_raw_parts(ptr, size_of::<T>() * N) }
    }

    /// mutably referecnes `self` as a byte array without the padding.
    ///
    /// * Can still contain garbage because `T` might have its own padding.
    ///
    /// SAFETY: This has the same safety as `transmute` would.
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self.as_mut_ptr() as *mut u8;

        unsafe { from_raw_parts_mut(ptr, size_of::<T>() * N) }
    }

    /// referecnes `self` as a byte array with the padding.
    ///
    /// * May contain garbage if `A == VecAligned` or if `T` has its own padding.
    pub const fn as_bytes_ref_padded(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;

        unsafe { from_raw_parts(ptr, size_of::<Self>()) }
    }

    /// mutably referecnes `self` as a byte array with the padding.
    ///
    /// * May contain garbage if `A == VecAligned` or if `T` has its own padding.
    ///
    /// SAFETY: This has the same safety as `transmute` would.
    pub const unsafe fn as_bytes_mut_padded(&mut self) -> &mut [u8] {
        let ptr = self.as_mut_ptr() as *mut u8;

        unsafe { from_raw_parts_mut(ptr, size_of::<Self>()) }
    }
}

// Convert T

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Converts the vector to a new vector with a different scalar type using the `From` trait.
    pub fn to_t<TOutput: Scalar + From<T>>(self) -> Vector<N, TOutput, A> {
        self.map(TOutput::from)
    }
}

// Convert A

impl<const N: usize, T: Scalar> From<Vector<N, T, VecAligned>> for Vector<N, T, VecPacked>
where
    Usize<N>: VecLen,
{
    fn from(value: Vector<N, T, VecAligned>) -> Self {
        value.unalign()
    }
}

impl<const N: usize, T: Scalar> From<Vector<N, T, VecPacked>> for Vector<N, T, VecAligned>
where
    Usize<N>: VecLen,
{
    fn from(value: Vector<N, T, VecPacked>) -> Self {
        value.align()
    }
}
