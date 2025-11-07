#![expect(missing_docs)]

use crate::{
    Construct, NonSimd, Scalar, Simd, Simdness, SupportedVecLen, VecLen, Vector, vector::vec4g,
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
