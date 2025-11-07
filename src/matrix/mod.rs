#![expect(missing_docs)]

use crate::{Construct, NonSimd, Scalar, Simd, Simdness, SupportedVecLen, VecLen, Vector};

#[repr(transparent)]
pub struct Matrix<const N: usize, T: Scalar, S: Simdness>(<Self as MatrixReprExt>::Repr)
where
    VecLen<N>: SupportedVecLen;

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
