#![expect(missing_docs)]

use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

use crate::{
    Construct, NonSimd, Scalar, Simd, Simdness, SupportedVecLen, Vec2, VecLen, Vector,
    vector::vec4g,
};

mod constructor;
mod deref;
pub use constructor::*;

#[repr(transparent)]
pub struct Matrix<const N: usize, T: Scalar, S: Simdness>(<Self as MatrixReprExt>::Repr)
where
    VecLen<N>: SupportedVecLen;

impl<const N: usize, T: Scalar, S: Simdness> Matrix<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    specialized_matrix_api! {
        MatrixApi for <N, T, S>:

        pub fn from_vectors(vectors: [Vector<N, T, S>; N]) -> Self;

        pub fn from_arrays(arrays: [[T; N]; N]) -> Self;
    }

    #[inline(always)]
    pub const fn const_from_vectors(vectors: [Vector<N, T, S>; N]) -> Self {
        let mut result = MaybeUninit::<Matrix<N, T, S>>::uninit();

        let mut i = 0;
        while i < N {
            unsafe {
                *result.as_mut_ptr().cast::<Vector<N, T, S>>().add(i) = vectors[i];
            }

            i += 1;
        }

        unsafe { result.assume_init() }
    }

    #[inline(always)]
    pub const fn is_simd(self) -> bool {
        S::IS_SIMD
    }

    #[inline(always)]
    pub fn as_simdness<S2: Simdness>(self) -> Matrix<N, T, S2> {
        if S::IS_SIMD == S2::IS_SIMD {
            // SAFETY: S and S2 are the same type, so we are transmuting a type to itself
            unsafe { core::mem::transmute_copy::<Matrix<N, T, S>, Matrix<N, T, S2>>(&self) }
        } else {
            Matrix::from_vectors(self.as_vectors().map(|vec| vec.as_simdness()))
        }
    }

    #[inline(always)]
    pub fn as_simd(self) -> Matrix<N, T, Simd> {
        self.as_simdness::<Simd>()
    }

    #[inline(always)]
    pub fn as_nonsimd(self) -> Matrix<N, T, NonSimd> {
        self.as_simdness::<NonSimd>()
    }

    #[inline(always)]
    pub const fn as_vectors(self) -> [Vector<N, T, S>; N] {
        *self.as_vectors_ref()
    }

    #[inline(always)]
    pub const fn as_vectors_ref(&self) -> &[Vector<N, T, S>; N] {
        const { assert!(size_of::<Matrix<N, T, S>>() == size_of::<[Vector<N, T, S>; N]>()) }
        const { assert!(align_of::<Matrix<N, T, S>>() >= size_of::<[Vector<N, T, S>; N]>()) }

        // SAFETY: Matrix<N, T, S> must be transmutable to [Vector<N, T, S>; N], even if it has higher alignment
        unsafe { core::mem::transmute::<&Matrix<N, T, S>, &[Vector<N, T, S>; N]>(self) }
    }

    #[inline(always)]
    pub const fn as_vectors_mut(&mut self) -> &mut [Vector<N, T, S>; N] {
        const { assert!(size_of::<Matrix<N, T, S>>() == size_of::<[Vector<N, T, S>; N]>()) }
        const { assert!(align_of::<Matrix<N, T, S>>() >= size_of::<[Vector<N, T, S>; N]>()) }

        // SAFETY: Matrix<N, T, S> must be transmutable to [Vector<N, T, S>; N], even if it has higher alignment
        unsafe { core::mem::transmute::<&mut Matrix<N, T, S>, &mut [Vector<N, T, S>; N]>(self) }
    }

    #[inline(always)]
    pub const fn as_arrays(self) -> [[T; N]; N] {
        let mut result = MaybeUninit::<[[T; N]; N]>::uninit();

        let mut i = 0;
        while i < N {
            unsafe {
                *result.as_mut_ptr().cast::<[T; N]>().add(i) = self.as_vectors_ref()[i].as_array();
            }

            i += 1;
        }

        unsafe { result.assume_init() }
    }
}

impl<const N: usize, T: Scalar, S: Simdness> Clone for Matrix<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T: Scalar, S: Simdness> Copy for Matrix<N, T, S> where
    VecLen<N>: SupportedVecLen
{
}

impl<const N: usize, T: Scalar, S: Simdness> Index<usize> for Matrix<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    type Output = Vector<N, T, S>;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_vectors_ref()[index]
    }
}

impl<const N: usize, T: Scalar, S: Simdness> IndexMut<usize> for Matrix<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_vectors_mut()[index]
    }
}

impl<const N: usize, T: Scalar + PartialEq, S: Simdness> PartialEq for Matrix<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        (0..N).all(|c| self[c] == other[c])
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        (0..N).any(|c| self[c] != other[c])
    }
}

////////////////////////////////////////////////////////////////////////////////
// Type Aliases
////////////////////////////////////////////////////////////////////////////////

pub type Mat2<T> = Matrix<2, T, Simd>;
pub type Mat3<T> = Matrix<3, T, Simd>;
pub type Mat4<T> = Matrix<4, T, Simd>;
pub type Mat2S<T> = Matrix<2, T, NonSimd>;
pub type Mat3S<T> = Matrix<3, T, NonSimd>;
pub type Mat4S<T> = Matrix<4, T, NonSimd>;

#[macro_export]
macro_rules! declare_matrix_aliases {
    ($vis:vis type $prefix:ident => $T:ty) => {
        $crate::hidden::paste! {
            #[expect(missing_docs)]
            $vis type [<$prefix Mat2>] = $crate::Matrix<2, $T, $crate::Simd>;
            #[expect(missing_docs)]
            $vis type [<$prefix Mat3>] = $crate::Matrix<3, $T, $crate::Simd>;
            #[expect(missing_docs)]
            $vis type [<$prefix Mat4>] = $crate::Matrix<4, $T, $crate::Simd>;
            #[expect(missing_docs)]
            $vis type [<$prefix Mat2S>] = $crate::Matrix<2, $T, $crate::NonSimd>;
            #[expect(missing_docs)]
            $vis type [<$prefix Mat3S>] = $crate::Matrix<3, $T, $crate::NonSimd>;
            #[expect(missing_docs)]
            $vis type [<$prefix Mat4S>] = $crate::Matrix<4, $T, $crate::NonSimd>;
        }
    };
}

pub use declare_matrix_aliases;

////////////////////////////////////////////////////////////////////////////////
// Representation
////////////////////////////////////////////////////////////////////////////////

trait MatrixReprExt {
    type Repr: Construct;
}

impl<const N: usize, T: Scalar, S: Simdness> MatrixReprExt for Matrix<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    type Repr = <VecLen<N> as SupportedVecLen>::Pick<
        Vector<4, T, S>,
        [Vector<3, T, S>; 3],
        [Vector<4, T, S>; 4],
    >;
}

////////////////////////////////////////////////////////////////////////////////
// Specialization
////////////////////////////////////////////////////////////////////////////////

trait MatrixApi<const N: usize, T: Scalar, S: Simdness>
where
    VecLen<N>: SupportedVecLen,
{
    fn mat_from_vectors(vectors: [Vector<N, T, S>; N]) -> Matrix<N, T, S>;

    fn mat_from_arrays(arrays: [[T; N]; N]) -> Matrix<N, T, S>;
}

impl<T: Scalar, S: Simdness> MatrixApi<2, T, S> for VecLen<2> {
    #[inline(always)]
    fn mat_from_vectors(vectors: [Vector<2, T, S>; 2]) -> Matrix<2, T, S> {
        Matrix(vec4g!(vectors[0], vectors[1]))
    }

    #[inline(always)]
    fn mat_from_arrays(arrays: [[T; 2]; 2]) -> Matrix<2, T, S> {
        Matrix(vec4g!(
            arrays[0][0],
            arrays[0][1],
            arrays[1][0],
            arrays[1][1],
        ))
    }
}

impl<const N: usize, T: Scalar, S: Simdness> MatrixApi<N, T, S> for VecLen<N>
where
    VecLen<N>: SupportedVecLen,
    Matrix<N, T, S>: MatrixReprExt<Repr = [Vector<N, T, S>; N]>,
{
    #[inline(always)]
    fn mat_from_vectors(vectors: [Vector<N, T, S>; N]) -> Matrix<N, T, S> {
        Matrix(vectors)
    }

    #[inline(always)]
    fn mat_from_arrays(arrays: [[T; N]; N]) -> Matrix<N, T, S> {
        Matrix(core::array::from_fn(|c| Vector::from_array(arrays[c])))
    }
}

macro_rules! specialized_matrix_api {
    {
        $Api:ident for <$N:expr, $T:ty, $S:ty>:

        $(
            $(#[$meta:meta])*
            $vis:vis fn $fn_name:ident$(<$(const $CONST_PARAM:ident: $ConstParam:ty),* $(,)?>)?(
                $($param:ident$(: $Param:ty)?),* $(,)?
            ) -> $return_tt:ty
            $(where
                $($BoundType:ty: $BoundTrait:path),*)?;
        )*
    } => {
        $(
            $(#[$meta])*
            $vis fn $fn_name$(<$(const $CONST_PARAM: $ConstParam),*>)?(
                $($param $(: $Param)?),*
            ) -> $return_tt
            $(where
                $($BoundType: $BoundTrait),*)?
            {
                (const {
                    match $N {
                        2 => specialized_matrix_api!(@transmute_fnptr($($param),*) <$crate::VecLen<2> as $Api<2, $T, $S>>::vec_$fn_name$(::<$($CONST_PARAM),*>)?),
                        3 => specialized_matrix_api!(@transmute_fnptr($($param),*) <$crate::VecLen<3> as $Api<3, $T, $S>>::vec_$fn_name$(::<$($CONST_PARAM),*>)?),
                        4 => specialized_matrix_api!(@transmute_fnptr($($param),*) <$crate::VecLen<4> as $Api<4, $T, $S>>::vec_$fn_name$(::<$($CONST_PARAM),*>)?),
                        ..2 | 5.. => panic!("N must be 2, 3, or 4"),
                    }
                })($($param),*)
            }
        )*
    };

    { @transmute_fnptr($($param:ident),*) <$VecLen:ty as $Api:ident<$N:expr, $T:ty, $S:ty>>::vec_$fn_name:ident$(::<$($CONST_PARAM:ident),*>)? } => {
        crate::hidden::paste! {
            unsafe {
                let fnptr: fn($(specialized_matrix_api!(@_ $param)),*) -> _ = <$VecLen as $Api<$N, $T, $S>>::[<mat_$fn_name>]$(::<$($CONST_PARAM),*>)?;

                core::mem::transmute_copy::<
                    fn($(specialized_matrix_api!(@_ $param)),*) -> _,
                    fn($(specialized_matrix_api!(@_ $param)),*) -> _,
                >(&fnptr)
            }
        }
    };

    { @_ $_:tt } => { _ }
}

use specialized_matrix_api;

////////////////////////////////////////////////////////////////////////////////
// Ensure Layout Safety
////////////////////////////////////////////////////////////////////////////////

const _: () = assert!(size_of::<Vec2<f32>>() == size_of::<f32>() * 2);
const _: () = assert!(size_of::<Vec2<f64>>() == size_of::<f64>() * 2);
const _: () = assert!(size_of::<Vec2<i8>>() == size_of::<i8>() * 2);
const _: () = assert!(size_of::<Vec2<i16>>() == size_of::<i16>() * 2);
const _: () = assert!(size_of::<Vec2<i32>>() == size_of::<i32>() * 2);
const _: () = assert!(size_of::<Vec2<i64>>() == size_of::<i64>() * 2);
const _: () = assert!(size_of::<Vec2<isize>>() == size_of::<isize>() * 2);
const _: () = assert!(size_of::<Vec2<u8>>() == size_of::<u8>() * 2);
const _: () = assert!(size_of::<Vec2<u16>>() == size_of::<u16>() * 2);
const _: () = assert!(size_of::<Vec2<u32>>() == size_of::<u32>() * 2);
const _: () = assert!(size_of::<Vec2<u64>>() == size_of::<u64>() * 2);
const _: () = assert!(size_of::<Vec2<usize>>() == size_of::<usize>() * 2);
const _: () = assert!(size_of::<Vec2<bool>>() == size_of::<bool>() * 2);
