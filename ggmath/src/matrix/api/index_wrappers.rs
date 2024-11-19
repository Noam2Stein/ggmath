use super::*;

macro_rules! index_wrappers {
    (for $c:literal, $r:literal: $(
        $fn_ident:ident($self:tt) -> $fn_output:ty { $fn_self_expr:expr }
    )*) => {
        impl<T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<$c, $r, T, A, M>
        {$(
            #[inline(always)]
            pub fn $fn_ident($self) -> $fn_output {
                unsafe { $fn_self_expr }
            }
        )*}
    };
    (for $c:literal, $r:ident: $(
        $fn_ident:ident($self:tt) -> $fn_output:ty { $fn_self_expr:expr }
    )*) => {
        impl<const $r: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<$c, $r, T, A, M>
        where
            ScalarCount<$r>: VecLen<$r>,
        {$(
            #[inline(always)]
            pub fn $fn_ident($self) -> $fn_output {
                unsafe { $fn_self_expr }
            }
        )*}
    };
    (for $c:ident, $r:literal: $(
        $fn_ident:ident($self:tt) -> $fn_output:ty { $fn_self_expr:expr }
    )*) => {
        impl<const $c: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<$c, $r, T, A, M>
        where
            ScalarCount<$c>: VecLen<$c>,
        {$(
            #[inline(always)]
            pub fn $fn_ident($self) -> $fn_output {
                unsafe { $fn_self_expr }
            }
        )*}
    };
    (for $c:ident, $r:ident: $(
        $fn_ident:ident($self:tt) -> $fn_output:ty { $fn_self_expr:expr }
    )*) => {
        impl<const $c: usize, const $r: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<$c, $r, T, A, M>
        where
            ScalarCount<$c>: VecLen<$c>,
            ScalarCount<$r>: VecLen<$r>,
        {$(
            #[inline(always)]
            pub fn $fn_ident($self) -> $fn_output {
                unsafe { $fn_self_expr }
            }
        )*}
    };
}

index_wrappers!(
    for 2, R:

    column0(self) -> Vector<R, T, A> { self.get_column_unchecked(0) }
    column1(self) -> Vector<R, T, A> { self.get_column_unchecked(1) }
);
index_wrappers!(
    for 3, R:

    column0(self) -> Vector<R, T, A> { self.get_column_unchecked(0) }
    column1(self) -> Vector<R, T, A> { self.get_column_unchecked(1) }
    column2(self) -> Vector<R, T, A> { self.get_column_unchecked(2) }
);
index_wrappers!(
    for 4, R:

    column0(self) -> Vector<R, T, A> { self.get_column_unchecked(0) }
    column1(self) -> Vector<R, T, A> { self.get_column_unchecked(1) }
    column2(self) -> Vector<R, T, A> { self.get_column_unchecked(2) }
    column3(self) -> Vector<R, T, A> { self.get_column_unchecked(3) }
);

index_wrappers!(
    for C, 2:

    row0(self) -> Vector<C, T, A> { self.get_row_unchecked(0) }
    row1(self) -> Vector<C, T, A> { self.get_row_unchecked(1) }
);
index_wrappers!(
    for C, 3:

    row0(self) -> Vector<C, T, A> { self.get_row_unchecked(0) }
    row1(self) -> Vector<C, T, A> { self.get_row_unchecked(1) }
    row2(self) -> Vector<C, T, A> { self.get_row_unchecked(2) }
);
index_wrappers!(
    for C, 4:

    row0(self) -> Vector<C, T, A> { self.get_row_unchecked(0) }
    row1(self) -> Vector<C, T, A> { self.get_row_unchecked(1) }
    row2(self) -> Vector<C, T, A> { self.get_row_unchecked(2) }
    row3(self) -> Vector<C, T, A> { self.get_row_unchecked(3) }
);
