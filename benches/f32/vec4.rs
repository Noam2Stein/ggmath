use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};

bench_unary_operator!(neg: ggmath_neg, glam_neg => -);

bench_binary_operator!(add: ggmath_add, glam_add => +);
bench_binary_operator!(sub: ggmath_sub, glam_sub => -);
bench_binary_operator!(mul: ggmath_mul, glam_mul => *);
bench_binary_operator!(div: ggmath_div, glam_div => /);
bench_binary_operator!(rem: ggmath_rem, glam_rem => %);

main!(library_benchmark_groups = neg, add, sub, mul, div, rem);

macro_rules! bench_unary_operator {
    ($group:ident: $ggmath_fn:ident, $glam_fn:ident => $op_punct:tt) => {
        #[library_benchmark]
        fn $ggmath_fn() -> ggmath::f32::FVec4 {
            black_box($op_punct black_box(ggmath::vec4!(1.0, 2.0, 3.0, 4.0)))
        }

        #[library_benchmark]
        fn $glam_fn() -> glam::Vec4 {
            black_box($op_punct black_box(glam::vec4(1.0, 2.0, 3.0, 4.0)))
        }

        library_benchmark_group!(name = $group; benchmarks = $ggmath_fn, $glam_fn);
    };
}

use bench_unary_operator;

macro_rules! bench_binary_operator {
    ($group:ident: $ggmath_fn:ident, $glam_fn:ident => $op_punct:tt) => {
        #[library_benchmark]
        fn $ggmath_fn() -> ggmath::f32::FVec4 {
            black_box(black_box(ggmath::vec4!(1.0, 2.0, 3.0, 4.0)) $op_punct black_box(ggmath::vec4!(5.0, 6.0, 7.0, 8.0)))
        }

        #[library_benchmark]
        fn $glam_fn() -> glam::Vec4 {
            black_box(black_box(glam::vec4(1.0, 2.0, 3.0, 4.0)) $op_punct black_box(glam::vec4(5.0, 6.0, 7.0, 8.0)))
        }

        library_benchmark_group!(name = $group; benchmarks = $ggmath_fn, $glam_fn);
    };
}

use bench_binary_operator;
