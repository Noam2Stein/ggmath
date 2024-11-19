use super::*;

macro_rules! index_wrappers {
    (for $c:literal, $r:literal: $(
        $fn_ident:ident($self:tt) -> $fn_output:ty { $fn_self_expr:expr }
    )*) => {
        impl<T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Matrix<$c, $r, T, A, M>
        {$(
            #[inline(always)]
            pub fn $fn_ident($self) -> $fn_output {
                #[allow(unused_unsafe)] unsafe { $fn_self_expr }
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
                #[allow(unused_unsafe)] unsafe { $fn_self_expr }
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
                #[allow(unused_unsafe)] unsafe { $fn_self_expr }
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
                #[allow(unused_unsafe)] unsafe { $fn_self_expr }
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

index_wrappers!(
    for 2, 2:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }
);
index_wrappers!(
    for 2, 3:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }
    m02(self) -> T { self[(0, 2)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }
    m12(self) -> T { self[(1, 2)] }
);
index_wrappers!(
    for 2, 4:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }
    m02(self) -> T { self[(0, 2)] }
    m03(self) -> T { self[(0, 3)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }
    m12(self) -> T { self[(1, 2)] }
    m13(self) -> T { self[(1, 3)] }
);

index_wrappers!(
    for 3, 2:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }

    m20(self) -> T { self[(2, 0)] }
    m21(self) -> T { self[(2, 1)] }
);
index_wrappers!(
    for 3, 3:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }
    m02(self) -> T { self[(0, 2)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }
    m12(self) -> T { self[(1, 2)] }

    m20(self) -> T { self[(2, 0)] }
    m21(self) -> T { self[(2, 1)] }
    m22(self) -> T { self[(2, 2)] }
);
index_wrappers!(
    for 3, 4:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }
    m02(self) -> T { self[(0, 2)] }
    m03(self) -> T { self[(0, 3)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }
    m12(self) -> T { self[(1, 2)] }
    m13(self) -> T { self[(1, 3)] }

    m20(self) -> T { self[(2, 0)] }
    m21(self) -> T { self[(2, 1)] }
    m22(self) -> T { self[(2, 2)] }
    m23(self) -> T { self[(2, 3)] }
);

index_wrappers!(
    for 4, 2:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }

    m20(self) -> T { self[(2, 0)] }
    m21(self) -> T { self[(2, 1)] }

    m30(self) -> T { self[(3, 0)] }
    m31(self) -> T { self[(3, 1)] }
);
index_wrappers!(
    for 4, 3:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }
    m02(self) -> T { self[(0, 2)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }
    m12(self) -> T { self[(1, 2)] }

    m20(self) -> T { self[(2, 0)] }
    m21(self) -> T { self[(2, 1)] }
    m22(self) -> T { self[(2, 2)] }

    m30(self) -> T { self[(3, 0)] }
    m31(self) -> T { self[(3, 1)] }
    m32(self) -> T { self[(3, 2)] }
);
index_wrappers!(
    for 4, 4:

    m00(self) -> T { self[(0, 0)] }
    m01(self) -> T { self[(0, 1)] }
    m02(self) -> T { self[(0, 2)] }
    m03(self) -> T { self[(0, 3)] }

    m10(self) -> T { self[(1, 0)] }
    m11(self) -> T { self[(1, 1)] }
    m12(self) -> T { self[(1, 2)] }
    m13(self) -> T { self[(1, 3)] }

    m20(self) -> T { self[(2, 0)] }
    m21(self) -> T { self[(2, 1)] }
    m22(self) -> T { self[(2, 2)] }
    m23(self) -> T { self[(2, 3)] }

    m30(self) -> T { self[(3, 0)] }
    m31(self) -> T { self[(3, 1)] }
    m32(self) -> T { self[(3, 2)] }
    m33(self) -> T { self[(3, 3)] }
);
