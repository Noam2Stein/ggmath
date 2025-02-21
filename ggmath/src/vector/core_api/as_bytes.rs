use std::slice::{from_raw_parts, from_raw_parts_mut};

use super::{Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    /// referecnes ```self``` as a byte array without the padding.
    ///
    /// * Can still contain garbage because ```T``` might have its own padding.
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self as *const Self as *const _, size_of::<T>() * N) }
    }

    /// mutably referecnes ```self``` as a byte array without the padding.
    ///
    /// * Can still contain garbage because ```T``` might have its own padding.
    ///
    /// SAFETY: Getting the reference is completely safe but mutating ```T``` as bytes can cause undefined behaviour.
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self as *mut Self as *mut _, size_of::<T>() * N) }
    }

    /// referecnes ```self``` as a byte array with the padding.
    ///
    /// * May contain garbage if ```A == VecAligned``` or if ```T``` has its own padding.
    pub const fn as_bytes_padded(&self) -> &[u8] {
        unsafe { from_raw_parts(self as *const Self as *const _, size_of::<T>() * N) }
    }

    /// mutably referecnes ```self``` as a byte array with the padding.
    ///
    /// * May contain garbage if ```A == VecAligned``` or if ```T``` has its own padding.
    ///
    /// SAFETY: Getting the reference is completely safe but mutating ```T``` as bytes can cause undefined behaviour.
    pub const unsafe fn as_bytes_padded_mut(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self as *mut Self as *mut _, size_of::<T>() * N) }
    }
}
