use super::*;

repetitive! {
    // Get Column

    @for C in 2..=4, c in 0..C {
        @let c_name = ["first", "second", "third", "fourth"][c];

        impl<const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<@C, R, T, A, M>
        where
            Usize<R>: VecLen,
        {
            #[doc = @str["Returns the " c_name " column of the matrix."]]
            #[inline(always)]
            pub const fn @['c c](self) -> Vector<R, T, A> {
                self.get_column(@c).unwrap()
            }
        }
    }

    @for C in 2..=4, c0 in 0..C, c1 in 0..C {
        @let c0_name = ["first", "second", "third", "fourth"][c0];
        @let c1_name = ["first", "second", "third", "fourth"][c1];

        impl<const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<@C, R, T, A, M>
        where
            Usize<R>: VecLen,
        {
            #[doc = @str["Returns the " c0_name " and " c1_name " columns of the matrix."]]
            #[inline(always)]
            pub const fn @['c c0 c1](self) -> Matrix<2, R, T, A, M> {
                Matrix::<2, R, T, A, M>::from_columns([
                    self.get_column(@c0).unwrap(),
                    self.get_column(@c1).unwrap(),
                ])
            }
        }
    }

    @for C in 2..=4, c0 in 0..C, c1 in 0..C, c2 in 0..C {
        @let c0_name = ["first", "second", "third", "fourth"][c0];
        @let c1_name = ["first", "second", "third", "fourth"][c1];
        @let c2_name = ["first", "second", "third", "fourth"][c2];

        impl<const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<@C, R, T, A, M>
        where
            Usize<R>: VecLen,
        {
            #[doc = @str["Returns the " c0_name ", " c1_name ", and " c2_name " columns of the matrix."]]
            #[inline(always)]
            pub const fn @['c c0 c1 c2](self) -> Matrix<3, R, T, A, M> {
                Matrix::<3, R, T, A, M>::from_columns([
                    self.get_column(@c0).unwrap(),
                    self.get_column(@c1).unwrap(),
                    self.get_column(@c2).unwrap(),
                ])
            }
        }
    }

    @for C in 2..=4, c0 in 0..C, c1 in 0..C, c2 in 0..C, c3 in 0..C {
        @let c0_name = ["first", "second", "third", "fourth"][c0];
        @let c1_name = ["first", "second", "third", "fourth"][c1];
        @let c2_name = ["first", "second", "third", "fourth"][c2];
        @let c3_name = ["first", "second", "third", "fourth"][c3];

        impl<const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<@C, R, T, A, M>
        where
            Usize<R>: VecLen,
        {
            #[doc = @str["Returns the " c0_name ", " c1_name ", " c2_name ", and " c3_name " columns of the matrix."]]
            #[inline(always)]
            pub const fn @['c c0 c1 c2 c3](self) -> Matrix<4, R, T, A, M> {
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

    @for R in 2..=4, r in 0..R {
        @let r_name = ["first", "second", "third", "fourth"][r];

        impl<const C: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<C, @R, T, A, M>
        where
            Usize<C>: VecLen,
        {
            #[doc = @str["Returns the " r_name " row of the matrix."]]
            #[inline(always)]
            pub const fn @['r r](self) -> Vector<C, T, A> {
                self.get_row(@r).unwrap()
            }
        }
    }

    @for R in 2..=4, r0 in 0..R, r1 in 0..R {
        @let r0_name = ["first", "second", "third", "fourth"][r0];
        @let r1_name = ["first", "second", "third", "fourth"][r1];

        impl<const C: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<C, @R, T, A, M>
        where
            Usize<C>: VecLen,
        {
            #[doc = @str["Returns the " r0_name " and " r1_name " rows of the matrix."]]
            #[inline(always)]
            pub const fn @['r r0 r1](self) -> Matrix<C, 2, T, A, M> {
                Matrix::<C, 2, T, A, M>::from_rows([
                    self.get_row(@r0).unwrap(),
                    self.get_row(@r1).unwrap(),
                ])
            }
        }
    }

    @for R in 2..=4, r0 in 0..R, r1 in 0..R, r2 in 0..R {
        @let r0_name = ["first", "second", "third", "fourth"][r0];
        @let r1_name = ["first", "second", "third", "fourth"][r1];
        @let r2_name = ["first", "second", "third", "fourth"][r2];

        impl<const C: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<C, @R, T, A, M>
        where
            Usize<C>: VecLen,
        {
            #[doc = @str["Returns the " r0_name ", " r1_name ", and " r2_name " rows of the matrix."]]
            #[inline(always)]
            pub const fn @['r r0 r1 r2](self) -> Matrix<C, 3, T, A, M> {
                Matrix::<C, 3, T, A, M>::from_rows([
                    self.get_row(@r0).unwrap(),
                    self.get_row(@r1).unwrap(),
                    self.get_row(@r2).unwrap(),
                ])
            }
        }
    }

    @for R in 2..=4, r0 in 0..R, r1 in 0..R, r2 in 0..R, r3 in 0..R {
        @let r0_name = ["first", "second", "third", "fourth"][r0];
        @let r1_name = ["first", "second", "third", "fourth"][r1];
        @let r2_name = ["first", "second", "third", "fourth"][r2];
        @let r3_name = ["first", "second", "third", "fourth"][r3];

        impl<const C: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<C, @R, T, A, M>
        where
            Usize<C>: VecLen,
        {
            #[doc = @str["Returns the " r0_name ", " r1_name ", " r2_name ", and " r3_name " rows of the matrix."]]
            #[inline(always)]
            pub const fn @['r r0 r1 r2 r3](self) -> Matrix<C, 4, T, A, M> {
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

    @for C in 2..=4, R in 2..=4, c in 0..C, r in 0..R {
        @let c_name = ["first", "second", "third", "fourth"][c];
        @let r_name = ["first", "second", "third", "fourth"][r];

        impl<T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<@C, @R, T, A, M> {
            #[doc = @str["Returns the value at the " c_name " column and " r_name " row of the matrix."]]
            #[inline(always)]
            pub const fn @['m c r](self) -> T {
                self.get_cell(@c, @r).unwrap()
            }
        }
    }

    // Get Column Ref

    @for C in 2..=4, c in 0..C {
        @let c_name = ["first", "second", "third", "fourth"][c];

        impl<const R: usize, T: Scalar, A: VecAlignment> Matrix<@C, R, T, A, ColMajor>
        where
            Usize<R>: VecLen,
        {
            #[doc = @str["Returns a reference to the " c_name " column of the matrix."]]
            #[inline(always)]
            pub const fn @['c c '_ref](&self) -> &Vector<R, T, A> {
                self.get_column_ref(@c).unwrap()
            }
        }
    }

    @for C in 2..=4, len in 2..=C, c in 0..=C - len {
        @let c_name = ["first", "second", "third", "fourth"][c];
        @let len_name = ["two", "three", "four"][len - 2];

        impl<const R: usize, T: Scalar, A: VecAlignment> Matrix<@C, R, T, A, ColMajor>
        where
            Usize<R>: VecLen,
        {
            #[doc = @str["Returns a reference to " len_name " consecutive columns starting at the " c_name " column of the matrix."]]
            #[inline(always)]
            pub const fn @['c (c..c + len).concat_ident() '_ref](&self) -> &Matrix<@len, R, T, A, ColMajor> {
                self.get_matc_ref::<@len>(@c).unwrap()
            }
        }
    }

    // Get Row Ref

    @for R in 2..=4, r in 0..R {
        @let r_name = ["first", "second", "third", "fourth"][r];

        impl<const C: usize, T: Scalar, A: VecAlignment> Matrix<C, @R, T, A, RowMajor>
        where
            Usize<C>: VecLen,
        {
            #[doc = @str["Returns a reference to the " r_name " row of the matrix."]]
            #[inline(always)]
            pub const fn @['r r '_ref](&self) -> &Vector<C, T, A> {
                self.get_row_ref(@r).unwrap()
            }
        }
    }

    @for R in 2..=4, len in 2..=R, r in 0..=R - len {
        @let r_name = ["first", "second", "third", "fourth"][r];
        @let len_name = ["two", "three", "four"][len - 2];

        impl<const C: usize, T: Scalar, A: VecAlignment> Matrix<C, @R, T, A, RowMajor>
        where
            Usize<C>: VecLen,
        {
            #[doc = @str["Returns a reference to " len_name " consecutive rows starting at the " r_name " row of the matrix."]]
            #[inline(always)]
            pub const fn @['r (r..r + len).concat_ident() '_ref](&self) -> &Matrix<C, @len, T, A, RowMajor> {
                self.get_matr_ref::<@len>(@r).unwrap()
            }
        }
    }

    // Get Cell Ref

    @for C in 2..=4, R in 2..=4, c in 0..C, r in 0..R {
        @let c_name = ["first", "second", "third", "fourth"][c];
        @let r_name = ["first", "second", "third", "fourth"][r];

        impl<T: Scalar, A: VecAlignment, M: MatMajorAxis> Matrix<@C, @R, T, A, M> {
            #[doc = @str["Returns a reference to the value at the " c_name " column and " r_name " row of the matrix."]]
            #[inline(always)]
            pub const fn @['m c r '_ref](&self) -> &T {
                self.get_cell_ref(@c, @r).unwrap()
            }
        }
    }
}
