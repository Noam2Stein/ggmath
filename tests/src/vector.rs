use std::fmt::Debug;

use ggmath::vector::{vec2, vec2p, vec3, vec3p, vec4, vec4p, VecAligned, Vector};
use ggmath_testing::TestableScalar;

pub fn test_vector() {
    test_builder::<f32>();
}

fn test_builder<T: TestableScalar>() {
    let values = T::n_normal_values::<4>(0);

    fn assert_eq<T: PartialEq<O> + Debug, O: Debug>(a: T, b: O) {
        assert_eq!(a, b)
    }

    macro_rules! test_builder {
        ($macro_:ident, $macro_p:ident: $n:tt $([$($field:tt), *]), *) => {$(
            assert_eq($macro_!($(builder_fields!($field)), *), Vector::<$n, T, VecAligned>::from_fn(|index| values[index]));
            assert_eq($macro_p!($(builder_fields!($field)), *), Vector::<$n, T, VecAligned>::from_fn(|index| values[index]));
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
}
