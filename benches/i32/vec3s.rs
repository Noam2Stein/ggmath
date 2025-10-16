use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};

#[path = "../bench_operator.rs"]
mod bench_operator;

bench_unary_operator!(neg { - } for ggmath::i32::IVec3S { vec3s!(1, 2, 3) }, glam::IVec3 { ivec3(1, 2, 3) });

bench_binary_operator!(add { + } for ggmath::i32::IVec3S { vec3s!(1, 2, 3), vec3s!(4, 5, 6) }, glam::IVec3 { ivec3(1, 2, 3), ivec3(4, 5, 6) });
bench_binary_operator!(sub { - } for ggmath::i32::IVec3S { vec3s!(1, 2, 3), vec3s!(4, 5, 6) }, glam::IVec3 { ivec3(1, 2, 3), ivec3(4, 5, 6) });
bench_binary_operator!(mul { * } for ggmath::i32::IVec3S { vec3s!(1, 2, 3), vec3s!(4, 5, 6) }, glam::IVec3 { ivec3(1, 2, 3), ivec3(4, 5, 6) });
bench_binary_operator!(div { / } for ggmath::i32::IVec3S { vec3s!(21, 12, 13), vec3s!(4, 5, 6) }, glam::IVec3 { ivec3(21, 12, 13), ivec3(4, 5, 6) });
bench_binary_operator!(rem { % } for ggmath::i32::IVec3S { vec3s!(21, 12, 13), vec3s!(4, 5, 6) }, glam::IVec3 { ivec3(21, 12, 13), ivec3(4, 5, 6) });

main!(library_benchmark_groups = neg, add, sub, mul, div, rem);
