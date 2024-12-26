use ggmath::vector::{
    vec2, vec2p, vec3, vec3p, vec4, vec4p, ScalarCount, VecAligned, VecAlignment, VecLen,
    VecPacked, Vector,
};
use ggmath_testing::{test_assert, vector_test_assert, TestFnDesc, TestResult, TestableScalar};

pub fn test_vector() -> TestResult {
    test_array::<2, f64, VecAligned>()?;
    test_array::<3, f64, VecAligned>()?;
    test_array::<4, f64, VecAligned>()?;
    test_array::<2, f64, VecPacked>()?;
    test_array::<3, f64, VecPacked>()?;
    test_array::<4, f64, VecPacked>()?;

    test_builder::<f32>()?;

    Ok(())
}

fn test_array<const N: usize, T: TestableScalar, A: VecAlignment>() -> TestResult
where
    ScalarCount<N>: VecLen,
{
    let array = T::n_values::<N>(0);
    let vector = Vector::<N, T, A>::from_array(array);

    vector_test_assert!(into_from_array: vector.into_array(), array, array);
    vector_test_assert!(as_array: vector.as_array(), array, vector);
    vector_test_assert!(as_array: vector.clone().as_array_mut(), array, vector);

    Ok(())
}

fn test_builder<T: TestableScalar>() -> TestResult {
    let values = T::n_values::<4>(0);

    macro_rules! test_builder {
        ($macro_:ident, $macro_p:ident: $n:tt $([$($field:tt), *]), *) => {$(
            test_assert!(FailedFn(format!(stringify!($macro_))), $macro_!($(builder_fields!($field)), *), Vector::<$n, T, VecAligned>::from_fn(|index| values[index]));
            test_assert!(FailedFn(format!(stringify!($macro_p))), $macro_p!($(builder_fields!($field)), *), Vector::<$n, T, VecAligned>::from_fn(|index| values[index]));
        )*};
    }
    macro_rules! builder_fields {
        ($index:literal) => {
            values[$index]
        };
        (($($index:literal), *)) => {
            Vector::<{ tt_count!($($index)*) }, T, VecAligned>::from_array([$(values[$index]), *])
        };
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

    test_builder!(vec2, vec2p: 2 [0, 1], [(0, 1)]);
    test_builder!(vec3, vec3p: 3 [0, 1, 2], [0, (1, 2)], [(0, 1), 2], [(0, 1, 2)]);
    test_builder!(vec4, vec4p: 4 [0, 1, 2, 3], [0, 1, (2, 3)], [0, (1, 2), 3], [(0, 1), 2, 3], [(0, 1), (2, 3)], [0, (1, 2, 3)], [(0, 1, 2), 3], [(0, 1, 2, 3)]);

    Ok(())
}
