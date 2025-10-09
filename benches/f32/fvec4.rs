use std::hint::black_box;

use ggmath::{f32_aliases::FVec4, vec4};
use gungraun::{library_benchmark, library_benchmark_group, main};

macro_rules! unary_operator_benchmark {
    ($group:ident: $ggmath_fn:ident, $glam_fn:ident => $op_punct:tt) => {
        #[library_benchmark]
        fn $ggmath_fn() -> FVec4 {
            black_box($op_punct black_box(vec4!(1.0, 2.0, 3.0, 4.0)))
        }

        #[library_benchmark]
        fn $glam_fn() -> glam::Vec4 {
            black_box($op_punct black_box(glam::vec4(1.0, 2.0, 3.0, 4.0)))
        }

        library_benchmark_group!(name = $group; benchmarks = $ggmath_fn, $glam_fn);
    };
}

unary_operator_benchmark!(neg: ggmath_neg, glam_neg => -);

macro_rules! binary_operator_benchmark {
    ($group:ident: $ggmath_fn:ident, $glam_fn:ident => $op_punct:tt) => {
        #[library_benchmark]
        fn $ggmath_fn() -> FVec4 {
            black_box(black_box(vec4!(1.0, 2.0, 3.0, 4.0)) $op_punct black_box(vec4!(5.0, 6.0, 7.0, 8.0)))
        }

        #[library_benchmark]
        fn $glam_fn() -> glam::Vec4 {
            black_box(black_box(glam::vec4(1.0, 2.0, 3.0, 4.0)) $op_punct black_box(glam::vec4(5.0, 6.0, 7.0, 8.0)))
        }

        library_benchmark_group!(name = $group; benchmarks = $ggmath_fn, $glam_fn);
    };
}

binary_operator_benchmark!(add: ggmath_add, glam_add => +);
binary_operator_benchmark!(sub: ggmath_sub, glam_sub => -);
binary_operator_benchmark!(mul: ggmath_mul, glam_mul => *);
binary_operator_benchmark!(div: ggmath_div, glam_div => /);
binary_operator_benchmark!(rem: ggmath_rem, glam_rem => %);

main!(library_benchmark_groups = neg, add, sub, mul, div, rem);
