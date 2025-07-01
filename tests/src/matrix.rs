use std::array;

use ggmath::{
    matrix::*,
    testing::{mat_test_assert, test_assert, TestFnDesc, TestResult, TestableScalar},
    vector::*,
};

pub fn test_matrix() -> TestResult {
    test_builder::<f32>()?;

    test_t::<u32>()?;

    Ok(())
}

fn test_builder<T: TestableScalar>() -> TestResult {
    let vectors = T::VALUES.map(Vector::<4, T, VecAligned>::from_array);

    macro_rules! test_builder {
        (
            $r:literal,
            $macro2:ident, $macro2p:ident, $macro2c:ident, $macro2cp:ident,
            $macro3:ident, $macro3p:ident, $macro3c:ident, $macro3cp:ident,
            $macro4:ident, $macro4p:ident, $macro4c:ident, $macro4cp:ident:
            $([$($field:tt), *]), *
        ) => {
            let rhs2 = Matrix::<2, $r, T, VecAligned, RowMajor>::from_rows_fn(|row_index| vectors[row_index].xy());
            let rhs3 = Matrix::<3, $r, T, VecAligned, RowMajor>::from_rows_fn(|row_index| vectors[row_index].xyz());
            let rhs4 = Matrix::<4, $r, T, VecAligned, RowMajor>::from_rows_fn(|row_index| vectors[row_index].xyzw());

            $(
                test_assert!(TestFnDesc(concat!(stringify!($macro2), "!").to_string()), $macro2!($(builder_field!(2 $field)); *), rhs2);
                test_assert!(TestFnDesc(concat!(stringify!($macro2p), "!").to_string()), $macro2p!($(builder_field!(2 $field)); *), rhs2);
                test_assert!(TestFnDesc(concat!(stringify!($macro2c), "!").to_string()), $macro2c!($(builder_field!(2 $field)); *), rhs2);
                test_assert!(TestFnDesc(concat!(stringify!($macro2cp), "!").to_string()), $macro2cp!($(builder_field!(2 $field)); *), rhs2);

                test_assert!(TestFnDesc(concat!(stringify!($macro2), "!").to_string()), $macro3!($(builder_field!(3 $field)); *), rhs3);
                test_assert!(TestFnDesc(concat!(stringify!($macro2p), "!").to_string()), $macro3p!($(builder_field!(3 $field)); *), rhs3);
                test_assert!(TestFnDesc(concat!(stringify!($macroc2), "!").to_string()), $macro3c!($(builder_field!(3 $field)); *), rhs3);
                test_assert!(TestFnDesc(concat!(stringify!($macro2cp), "!").to_string()), $macro3cp!($(builder_field!(3 $field)); *), rhs3);

                test_assert!(TestFnDesc(concat!(stringify!($macro2), "!").to_string()), $macro4!($(builder_field!(4 $field)); *), rhs4);
                test_assert!(TestFnDesc(concat!(stringify!($macro2p), "!").to_string()), $macro4p!($(builder_field!(4 $field)); *), rhs4);
                test_assert!(TestFnDesc(concat!(stringify!($macro2c), "!").to_string()), $macro4c!($(builder_field!(4 $field)); *), rhs4);
                test_assert!(TestFnDesc(concat!(stringify!($macro2cp), "!").to_string()), $macro4cp!($(builder_field!(4 $field)); *), rhs4);
            )*
        };
    }
    macro_rules! builder_field {
        ($c:tt $index:literal) => {
            first_n_components!($c(vectors[$index]))
        };
        ($c:tt ($($index:literal), *)) => {
            Matrix::<$c, { tt_count!($($index)*) }, T, VecAligned, RowMajor>::from_rows([$(first_n_components!($c(vectors[$index]))), *])
        }
    }
    macro_rules! tt_count {
        ($tt0:tt $tt1:tt) => {
            2
        };
        ($tt0:tt $tt1:tt $tt2:tt) => {
            3
        };
        ($tt0:tt $tt1:tt $tt2:tt $tt3:tt) => {
            4
        };
    }
    macro_rules! first_n_components {
        (2($expr:expr)) => {
            Vector::<4, _, _>::xy($expr)
        };
        (3($expr:expr)) => {
            Vector::<4, _, _>::xyz($expr)
        };
        (4($expr:expr)) => {
            Vector::<4, _, _>::xyzw($expr)
        };
    }

    test_builder!(
        2, mat2, mat2p, mat2c, mat2cp, mat3x2, mat3x2p, mat3x2c, mat3x2cp, mat4x2, mat4x2p, mat4x2c, mat4x2cp:
        [0, 1], [(0, 1)]
    );
    test_builder!(
        3, mat2x3, mat2x3p, mat2x3c, mat2x3cp, mat3, mat3p, mat3c, mat3cp, mat4x3, mat4x3p, mat4x3c, mat4x3cp:
        [0, 1, 2], [0, (1, 2)], [(0, 1), 2], [(0, 1, 2)]
    );
    test_builder!(
        4, mat2x4, mat2x4p, mat2x4c, mat2x4cp, mat3x4, mat3x4p, mat3x4c, mat3x4cp, mat4, mat4p, mat4c, mat4cp:
        [0, 1, 2, 3], [0, 1, (2, 3)], [0, (1, 2), 3], [(0, 1), 2, 3], [(0, 1), (2, 3)], [0, (1, 2, 3)], [(0, 1, 2), 3], [(0, 1, 2, 3)]
    );

    Ok(())
}

fn test_t<T: TestableScalar>() -> TestResult {
    test_c_r_t::<2, 4, T>()?;
    test_c_r_t::<3, 2, T>()?;
    test_c_r_t::<4, 4, T>()?;

    Ok(())
}
fn test_c_r_t<const C: usize, const R: usize, T: TestableScalar>() -> TestResult
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    test_c_r_t_a_m::<C, R, T, VecAligned, ColumnMajor>()?;
    test_c_r_t_a_m::<C, R, T, VecAligned, RowMajor>()?;

    test_c_r_t_a_m::<C, R, T, VecPacked, ColumnMajor>()?;
    test_c_r_t_a_m::<C, R, T, VecPacked, RowMajor>()?;

    Ok(())
}

fn test_c_r_t_a_m<
    const C: usize,
    const R: usize,
    T: TestableScalar,
    A: VecAlignment,
    M: MatrixMajorAxis,
>() -> TestResult
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    for values_r in T::get_4_c_r::<C, R>() {
        let values_c = array::from_fn::<_, C, _>(|column_index| {
            array::from_fn::<_, R, _>(|row_index| values_r[row_index][column_index])
        });

        test_conversions::<C, R, T, A, M>(values_r, values_c)?;
    }

    Ok(())
}

fn test_conversions<
    const C: usize,
    const R: usize,
    T: TestableScalar,
    A: VecAlignment,
    M: MatrixMajorAxis,
>(
    values_r: [[T; C]; R],
    values_c: [[T; R]; C],
) -> TestResult
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    let rows = values_r.map(|row_array| Vector::<C, T, A>::from_array(row_array));
    let columns = values_c.map(|column_array| Vector::<R, T, A>::from_array(column_array));

    let matrix = Matrix::<C, R, T, A, M>::from_rows(rows);

    mat_test_assert!(from_columns: Matrix::<C, R, T, A, M>::from_columns(columns), matrix; matrix);

    mat_test_assert!(into_rows: matrix.into_rows(), rows; matrix);
    mat_test_assert!(into_columns: matrix.into_columns(), columns; matrix);
    mat_test_assert!(into_rows_array: matrix.into_rows_array(), values_r; matrix);
    mat_test_assert!(into_columns_array: matrix.into_columns_array(), values_c; matrix);

    mat_test_assert!(into_aligned: matrix.into_aligned(), matrix; matrix);
    mat_test_assert!(into_packed: matrix.into_packed(), matrix; matrix);
    mat_test_assert!(into_column_major: matrix.into_column_major(), matrix; matrix);
    mat_test_assert!(into_row_major: matrix.into_row_major(), matrix; matrix);

    Ok(())
}
