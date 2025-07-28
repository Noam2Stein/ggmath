use macro_loop::macro_loop;

use super::*;

macro_loop! {
    // Get Column

    @for C in 2..=4, c in 0..@C {
        impl<const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<@C, R, T, A, M>
        where
            Usize<R>: VecLen,
        {
            #[inline(always)]
            pub const fn @[c @c](self) -> Vector<R, T, A> {
                self.get_column(@c).unwrap()
            }
        }
    }

    @for C in 2..=4, c0 in 0..@C, c1 in 0..@C {
        impl<const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<@C, R, T, A, M>
        where
            Usize<R>: VecLen,
        {
            #[inline(always)]
            pub const fn @[c @c0 @c1](self) -> Matrix<2, R, T, A, M> {
                Matrix::<2, R, T, A, M>::from_columns([
                    self.get_column(@c0).unwrap(),
                    self.get_column(@c1).unwrap(),
                ])
            }
        }
    }

    @for C in 2..=4, c0 in 0..@C, c1 in 0..@C, c2 in 0..@C {
        impl<const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<@C, R, T, A, M>
        where
            Usize<R>: VecLen,
        {
            #[inline(always)]
            pub const fn @[c @c0 @c1 @c2](self) -> Matrix<3, R, T, A, M> {
                Matrix::<3, R, T, A, M>::from_columns([
                    self.get_column(@c0).unwrap(),
                    self.get_column(@c1).unwrap(),
                    self.get_column(@c2).unwrap(),
                ])
            }
        }
    }

    @for C in 2..=4, c0 in 0..@C, c1 in 0..@C, c2 in 0..@C, c3 in 0..@C {
        impl<const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<@C, R, T, A, M>
        where
            Usize<R>: VecLen,
        {
            #[inline(always)]
            pub const fn @[c @c0 @c1 @c2 @c3](self) -> Matrix<4, R, T, A, M> {
                Matrix::<4, R, T, A, M>::from_columns([
                    self.get_column(@c0).unwrap(),
                    self.get_column(@c1).unwrap(),
                    self.get_column(@c2).unwrap(),
                    self.get_column(@c3).unwrap(),
                ])
            }
        }
    }

    // Get Row

    @for R in 2..=4, r in 0..@R {
        impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<C, @R, T, A, M>
        where
            Usize<C>: VecLen,
        {
            #[inline(always)]
            pub const fn @[r @r](self) -> Vector<C, T, A> {
                self.get_row(@r).unwrap()
            }
        }
    }

    @for R in 2..=4, r0 in 0..@R, r1 in 0..@R {
        impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<C, @R, T, A, M>
        where
            Usize<C>: VecLen,
        {
            #[inline(always)]
            pub const fn @[r @r0 @r1](self) -> Matrix<C, 2, T, A, M> {
                Matrix::<C, 2, T, A, M>::from_rows([
                    self.get_row(@r0).unwrap(),
                    self.get_row(@r1).unwrap(),
                ])
            }
        }
    }

    @for R in 2..=4, r0 in 0..@R, r1 in 0..@R, r2 in 0..@R {
        impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<C, @R, T, A, M>
        where
            Usize<C>: VecLen,
        {
            #[inline(always)]
            pub const fn @[r @r0 @r1 @r2](self) -> Matrix<C, 3, T, A, M> {
                Matrix::<C, 3, T, A, M>::from_rows([
                    self.get_row(@r0).unwrap(),
                    self.get_row(@r1).unwrap(),
                    self.get_row(@r2).unwrap(),
                ])
            }
        }
    }

    @for R in 2..=4, r0 in 0..@R, r1 in 0..@R, r2 in 0..@R, r3 in 0..@R {
        impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<C, @R, T, A, M>
        where
            Usize<C>: VecLen,
        {
            #[inline(always)]
            pub const fn @[r @r0 @r1 @r2 @r3](self) -> Matrix<C, 4, T, A, M> {
                Matrix::<C, 4, T, A, M>::from_rows([
                    self.get_row(@r0).unwrap(),
                    self.get_row(@r1).unwrap(),
                    self.get_row(@r2).unwrap(),
                    self.get_row(@r3).unwrap(),
                ])
            }
        }
    }

    // Get Cell

    @for C in 2..=4, R in 2..=4, c in 0..@C, r in 0..@R {
        impl<T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<@C, @R, T, A, M> {
            #[inline(always)]
            pub const fn @[m @c @r](self) -> T {
                self.get_cell(@c, @r).unwrap()
            }
        }
    }

    // Get Column Ref

    @for C in 2..=4, c in 0..@C {
        impl<const R: usize, T: Scalar, A: VecAlignment> Matrix<@C, R, T, A, ColumnMajor>
        where
            Usize<R>: VecLen,
        {
            #[inline(always)]
            pub const fn @[c @c _ref](&self) -> &Vector<R, T, A> {
                self.get_column_ref(@c).unwrap()
            }
        }
    }

    @for C in 2..=4, len in 2..=@C, c in 0..=@C - @len {
        impl<const R: usize, T: Scalar, A: VecAlignment> Matrix<@C, R, T, A, ColumnMajor>
        where
            Usize<R>: VecLen,
        {
            #[inline(always)]
            pub const fn @[c @(@c..@c + @len) _ref](&self) -> &Matrix<@len, R, T, A, ColumnMajor> {
                self.get_matc_ref::<@len>(@c).unwrap()
            }
        }
    }

    // Get Row Ref

    @for R in 2..=4, r in 0..@R {
        impl<const C: usize, T: Scalar, A: VecAlignment> Matrix<C, @R, T, A, RowMajor>
        where
            Usize<C>: VecLen,
        {
            #[inline(always)]
            pub const fn @[r @r _ref](&self) -> &Vector<C, T, A> {
                self.get_row_ref(@r).unwrap()
            }
        }
    }

    @for R in 2..=4, len in 2..=@R, r in 0..=@R - @len {
        impl<const C: usize, T: Scalar, A: VecAlignment> Matrix<C, @R, T, A, RowMajor>
        where
            Usize<C>: VecLen,
        {
            #[inline(always)]
            pub const fn @[r @(@r..@r + @len) _ref](&self) -> &Matrix<C, @len, T, A, RowMajor> {
                self.get_matr_ref::<@len>(@r).unwrap()
            }
        }
    }

    // Get Cell Ref

    @for C in 2..=4, R in 2..=4, c in 0..@C, r in 0..@R {
        impl<T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<@C, @R, T, A, M> {
            #[inline(always)]
            pub const fn @[m @c @r _ref](&self) -> &T {
                self.get_cell_ref(@c, @r).unwrap()
            }
        }
    }
}
