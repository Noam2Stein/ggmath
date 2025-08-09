use super::*;

// Splat

repetitive! {
    @for C in 2..=4, R in 2..=4, [M, m, major] in [['C, 'c, "column-major"], ['R, 'r, "row-major"]] {
        @let splat = @['splat C 'x R m];
        @let splatp = @['splat C 'x R m 'p];

        @if C == R {
            @let Mat = @['Mat C M];
            @let MatP = @['Mat C M 'P];
            @let matrixCxR = @['matrix C 'x R];

            #[doc = @str["Constructs a " major " aligned " matrixCxR " where all elements are the same value."]]
            #[inline(always)]
            pub const fn @splat<T: Scalar>(t: T) -> @Mat<T> {
                Matrix::splat(t)
            }

            #[doc = @str["Constructs a " major " unaligned " matrixCxR " where all elements are the same value."]]
            #[inline(always)]
            pub const fn @splatp<T: Scalar>(t: T) -> @MatP<T> {
                Matrix::splat(t)
            }
        }
        @if C != R {
            @let Mat = @['Mat C 'x R M];
            @let MatP = @['Mat C 'x R M 'P];
            @let matrixCxR = @['matrix C 'x R];

            #[doc = @str["Constructs a " major " aligned " matrixCxR " where all elements are the same value."]]
            #[inline(always)]
            pub const fn @splat<T: Scalar>(t: T) -> @Mat<T> {
                Matrix::splat(t)
            }

            #[doc = @str["Constructs a " major " unaligned " matrixCxR " where all elements are the same value."]]
            #[inline(always)]
            pub const fn @splatp<T: Scalar>(t: T) -> @MatP<T> {
                Matrix::splat(t)
            }
        }

    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Constructs a matrix where all elements are the same value.
    #[inline(always)]
    pub const fn splat(t: T) -> Self {
        Self::resolved(
            Matrix {
                inner: [Vector::splat(t); _],
            },
            Matrix {
                inner: [Vector::splat(t); _],
            },
        )
    }
}
