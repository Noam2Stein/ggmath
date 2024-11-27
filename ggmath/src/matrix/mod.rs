use crate::{construct::*, ggmath, scalar::*, vector::*};

mod api;
mod generics;
mod impl_std;
pub use api::*;
pub use generics::*;

pub struct Matrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    inner: M::InnerMatrix<C, R, T, A>,
}

pub type Mat2C<T> = Matrix<2, 2, T, VecAligned, ColumnMajor>;
pub type Mat2x3C<T> = Matrix<2, 3, T, VecAligned, ColumnMajor>;
pub type Mat2x4C<T> = Matrix<2, 4, T, VecAligned, ColumnMajor>;
pub type Mat3x2C<T> = Matrix<3, 2, T, VecAligned, ColumnMajor>;
pub type Mat3C<T> = Matrix<3, 3, T, VecAligned, ColumnMajor>;
pub type Mat3x4C<T> = Matrix<3, 4, T, VecAligned, ColumnMajor>;
pub type Mat4x2C<T> = Matrix<4, 2, T, VecAligned, ColumnMajor>;
pub type Mat4x3C<T> = Matrix<4, 3, T, VecAligned, ColumnMajor>;
pub type Mat4C<T> = Matrix<4, 4, T, VecAligned, ColumnMajor>;

pub type Mat2CP<T> = Matrix<2, 2, T, VecPacked, ColumnMajor>;
pub type Mat2x3CP<T> = Matrix<2, 3, T, VecPacked, ColumnMajor>;
pub type Mat2x4CP<T> = Matrix<2, 4, T, VecPacked, ColumnMajor>;
pub type Mat3x2CP<T> = Matrix<3, 2, T, VecPacked, ColumnMajor>;
pub type Mat3CP<T> = Matrix<3, 3, T, VecPacked, ColumnMajor>;
pub type Mat3x4CP<T> = Matrix<3, 4, T, VecPacked, ColumnMajor>;
pub type Mat4x2CP<T> = Matrix<4, 2, T, VecPacked, ColumnMajor>;
pub type Mat4x3CP<T> = Matrix<4, 3, T, VecPacked, ColumnMajor>;
pub type Mat4CP<T> = Matrix<4, 4, T, VecPacked, ColumnMajor>;

pub type Mat2R<T> = Matrix<2, 2, T, VecAligned, RowMajor>;
pub type Mat2x3R<T> = Matrix<2, 3, T, VecAligned, RowMajor>;
pub type Mat2x4R<T> = Matrix<2, 4, T, VecAligned, RowMajor>;
pub type Mat3x2R<T> = Matrix<3, 2, T, VecAligned, RowMajor>;
pub type Mat3R<T> = Matrix<3, 3, T, VecAligned, RowMajor>;
pub type Mat3x4R<T> = Matrix<3, 4, T, VecAligned, RowMajor>;
pub type Mat4x2R<T> = Matrix<4, 2, T, VecAligned, RowMajor>;
pub type Mat4x3R<T> = Matrix<4, 3, T, VecAligned, RowMajor>;
pub type Mat4R<T> = Matrix<4, 4, T, VecAligned, RowMajor>;

pub type Mat2RP<T> = Matrix<2, 2, T, VecPacked, RowMajor>;
pub type Mat2x3RP<T> = Matrix<2, 3, T, VecPacked, RowMajor>;
pub type Mat2x4RP<T> = Matrix<2, 4, T, VecPacked, RowMajor>;
pub type Mat3x2RP<T> = Matrix<3, 2, T, VecPacked, RowMajor>;
pub type Mat3RP<T> = Matrix<3, 3, T, VecPacked, RowMajor>;
pub type Mat3x4RP<T> = Matrix<3, 4, T, VecPacked, RowMajor>;
pub type Mat4x2RP<T> = Matrix<4, 2, T, VecPacked, RowMajor>;
pub type Mat4x3RP<T> = Matrix<4, 3, T, VecPacked, RowMajor>;
pub type Mat4RP<T> = Matrix<4, 4, T, VecPacked, RowMajor>;

pub use ggmath_proc_macros::{
    mat2, mat2c, mat2cp, mat2p, mat2x3, mat2x3c, mat2x3cp, mat2x3p, mat2x4, mat2x4c, mat2x4cp,
    mat2x4p, mat3, mat3c, mat3cp, mat3p, mat3x2, mat3x2c, mat3x2cp, mat3x2p, mat3x4, mat3x4c,
    mat3x4cp, mat3x4p, mat4, mat4c, mat4cp, mat4p, mat4x2, mat4x2c, mat4x2cp, mat4x2p, mat4x3,
    mat4x3c, mat4x3cp, mat4x3p, matrix_aliases,
};

#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod f32_aliases for f32(F));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod f64_aliases for f64(D));

#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod u8_aliases for u8(U8));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod u16_aliases for u16(U16));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod u32_aliases for u32(U));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod u64_aliases for u64(U64));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod u128_aliases for u128(U128));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod usize_aliases for usize(USize));

#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod i8_aliases for i8(I8));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod i16_aliases for i16(I16));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod i32_aliases for i32(I));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod i64_aliases for i64(I64));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod i128_aliases for i128(I128));
#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod isize_aliases for isize(ISize));

#[cfg(feature = "primitive_aliases")]
matrix_aliases!(pub mod bool_aliases for bool(B));
