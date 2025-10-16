use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};

#[path = "../bench_operator.rs"]
mod bench_operator;

bench_unary_operator!(neg { - } for ggmath::i32::IVec2 { vec2!(1, 2) }, glam::IVec2 { ivec2(1, 2) });

bench_binary_operator!(add { + } for ggmath::i32::IVec2 { vec2!(1, 2), vec2!(3, 4) }, glam::IVec2 { ivec2(1, 2), ivec2(3, 4) });
bench_binary_operator!(sub { - } for ggmath::i32::IVec2 { vec2!(1, 2), vec2!(3, 4) }, glam::IVec2 { ivec2(1, 2), ivec2(3, 4) });
bench_binary_operator!(mul { * } for ggmath::i32::IVec2 { vec2!(1, 2), vec2!(3, 4) }, glam::IVec2 { ivec2(1, 2), ivec2(3, 4) });
bench_binary_operator!(div { / } for ggmath::i32::IVec2 { vec2!(21, 12), vec2!(3, 4) }, glam::IVec2 { ivec2(21, 12), ivec2(3, 4) });
bench_binary_operator!(rem { % } for ggmath::i32::IVec2 { vec2!(21, 12), vec2!(3, 4) }, glam::IVec2 { ivec2(21, 12), ivec2(3, 4) });

main!(library_benchmark_groups = neg, add, sub, mul, div, rem);
