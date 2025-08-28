use std::{
    mem::{transmute, transmute_copy},
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
        match A::IS_ALIGNED {
            true => {
                let mut output = match N {
                    2 => unsafe { transmute_copy::<T::InnerVec2A, Self>(&T::INNER_VEC2A_GARBAGE) },
			3 => unsafe { transmute_copy::<T::InnerVec3A, Self>(&T::INNER_VEC3A_GARBAGE) },
			4 => unsafe { transmute_copy::<T::InnerVec4A, Self>(&T::INNER_VEC4A_GARBAGE) },
                    _ => unreachable!(),
                };

                *output.as_array_mut() = array;

                output
            },
            false => unsafe { transmute_copy::<[T; N], Self>(&array) },
        }
    }

    /// Converts the vector into an array.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        unsafe { transmute_copy::<Self, [T; N]>(&self) }
    }

    /// Referecnes the vector as an array.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn as_array_ref(&self) -> &[T; N] {
        unsafe { transmute::<&Self, &[T; N]>(self) }
    }

    /// Mutably referecnes the vector as an array.
    ///
    /// Cost: nothing.
    #[inline(always)]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute::<&mut Self, &mut [T; N]>(self) }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_conversion() {
        assert_eq!(vec2!(1, 2).to_t::<f64>(), vec2!(1.0, 2.0));
        assert_eq!(vec3p!(1, 2, 3).to_t::<f64>(), vec3!(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_alignment_conversion() {
        assert_eq!(vec2!(1, 2).align(), vec2!(1, 2));
        assert_eq!(vec3p!(1, 2, 3).align(), vec3!(1, 2, 3));
        assert_eq!(vec4!(1, 2, 3, 4).unalign(), vec4!(1, 2, 3, 4));
        assert_eq!(vec2p!(1, 2).unalign(), vec2p!(1, 2));

        assert_eq!(Vec2::from(vec2p!(1, 2)), vec2!(1, 2));
        assert_eq!(Vec3P::from(vec3!(1, 2, 3)), vec3!(1, 2, 3));
    }

    #[test]
    fn test_layout_conversion() {
        assert_eq!(vec2!(1, 2).to_layout::<VecAligned>(), vec2!(1, 2));
        assert_eq!(vec3!(1, 2, 3).to_layout::<VecAligned>(), vec3!(1, 2, 3));
        assert_eq!(vec3!(1, 2, 3).to_layout::<VecPacked>(), vec3!(1, 2, 3));
    }

    #[test]
    fn test_array_conversion() {
        assert_eq!(vec2!(1, 2).to_array(), [1, 2]);
        assert_eq!(vec3p!(1, 2, 3).to_array(), [1, 2, 3]);

        assert_eq!(vec2!(1, 2).as_array_ref(), &[1, 2]);
        assert_eq!(vec3p!(1, 2, 3).as_array_ref(), &[1, 2, 3]);

        assert_eq!(vec2!(1, 2).as_array_mut(), &mut [1, 2]);
        assert_eq!(vec3p!(1, 2, 3).as_array_mut(), &mut [1, 2, 3]);

        unsafe {
            assert_eq!(vec2!(1, 2).as_ptr().as_ref(), [1, 2].as_ptr().as_ref());
            assert_eq!(
                vec3p!(1, 2, 3).as_ptr().as_ref(),
                [1, 2, 3].as_ptr().as_ref()
            );

            assert_eq!(
                vec2!(1, 2).as_mut_ptr().as_mut(),
                [1, 2].as_mut_ptr().as_mut()
            );
            assert_eq!(
                vec3p!(1, 2, 3).as_mut_ptr().as_mut(),
                [1, 2, 3].as_mut_ptr().as_mut()
            );
        }

        assert_eq!(Vec2P::from_array_ref(&[1, 2]), &vec2p!(1, 2));
        assert_eq!(Vec2P::from_array_mut(&mut [1, 2]), &mut vec2!(1, 2));

        assert_eq!(Vec2::from([1, 2]), vec2!(1, 2));
        assert_eq!(Vec3P::from([1, 2, 3]), vec3p!(1, 2, 3));
        assert_eq!(<Vec2<_> as Into<[_; 2]>>::into(vec2!(1, 2)), [1, 2]);

        assert_eq!(<Vec2<_> as AsRef<[_; 2]>>::as_ref(&vec2!(1, 2)), &[1, 2]);
    }

    #[test]
    fn test_bytes_conversion() {
        unsafe {
            assert_eq!(
                vec2!(1u16, 2u16).as_bytes_ref(),
                &transmute::<[u16; 2], [u8; 4]>([1, 2])
            );
            assert_eq!(
                vec3p!(1u16, 2u16, 3u16).as_bytes_ref(),
                &transmute::<[u16; 3], [u8; 6]>([1, 2, 3])
            );

            assert_eq!(
                vec2!(1u16, 2u16).as_bytes_mut(),
                &mut transmute::<[u16; 2], [u8; 4]>([1, 2])
            );
            assert_eq!(
                vec3p!(1u16, 2u16, 3u16).as_bytes_mut(),
                &mut transmute::<[u16; 3], [u8; 6]>([1, 2, 3])
            );

            assert_eq!(
                vec2!(1u16, 2u16).as_bytes_ref_padded().len(),
                size_of::<U16Vec2>()
            );
            assert_eq!(
                vec3!(1u16, 2u16, 3u16).as_bytes_ref_padded().len(),
                size_of::<U16Vec3>()
            );
            assert_eq!(
                vec4!(1u16, 2u16, 3u16, 4u16).as_bytes_ref_padded().len(),
                size_of::<U16Vec4>()
            );

            assert_eq!(
                vec2p!(1u16, 2u16).as_bytes_ref_padded(),
                vec2p!(1u16, 2u16).as_bytes_ref()
            );
            assert_eq!(
                vec3p!(1u16, 2u16, 3u16).as_bytes_ref_padded(),
                vec3p!(1u16, 2u16, 3u16).as_bytes_ref()
            );
            assert_eq!(
                vec4p!(1u16, 2u16, 3u16, 4u16).as_bytes_ref_padded(),
                vec4p!(1u16, 2u16, 3u16, 4u16).as_bytes_ref()
            );

            assert_eq!(
                vec2!(1u16, 2u16).as_bytes_mut_padded().len(),
                size_of::<U16Vec2>()
            );
            assert_eq!(
                vec3!(1u16, 2u16, 3u16).as_bytes_mut_padded().len(),
                size_of::<U16Vec3>()
            );
            assert_eq!(
                vec4!(1u16, 2u16, 3u16, 4u16).as_bytes_mut_padded().len(),
                size_of::<U16Vec4>()
            );

            assert_eq!(
                vec2p!(1u16, 2u16).as_bytes_mut_padded(),
                vec2p!(1u16, 2u16).as_bytes_mut()
            );
            assert_eq!(
                vec3p!(1u16, 2u16, 3u16).as_bytes_mut_padded(),
                vec3p!(1u16, 2u16, 3u16).as_bytes_mut()
            );
            assert_eq!(
                vec4p!(1u16, 2u16, 3u16, 4u16).as_bytes_mut_padded(),
                vec4p!(1u16, 2u16, 3u16, 4u16).as_bytes_mut()
            );
        }
    }
}
