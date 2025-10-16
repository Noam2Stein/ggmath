use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};

#[path = "../bench_operator.rs"]
mod bench_operator;

bench_unary_operator!(neg { - } for ggmath::f32::FVec2 { vec2!(1.0, 2.0) }, glam::Vec2 { vec2(1.0, 2.0) });

bench_binary_operator!(add { + } for ggmath::f32::FVec2 { vec2!(1.0, 2.0), vec2!(3.0, 4.0) }, glam::Vec2 { vec2(1.0, 2.0), vec2(3.0, 4.0) });
bench_binary_operator!(sub { - } for ggmath::f32::FVec2 { vec2!(1.0, 2.0), vec2!(3.0, 4.0) }, glam::Vec2 { vec2(1.0, 2.0), vec2(3.0, 4.0) });
bench_binary_operator!(mul { * } for ggmath::f32::FVec2 { vec2!(1.0, 2.0), vec2!(3.0, 4.0) }, glam::Vec2 { vec2(1.0, 2.0), vec2(3.0, 4.0) });
bench_binary_operator!(div { / } for ggmath::f32::FVec2 { vec2!(21.0, 12.0), vec2!(3.0, 4.0) }, glam::Vec2 { vec2(21.0, 12.0), vec2(3.0, 4.0) });
bench_binary_operator!(rem { % } for ggmath::f32::FVec2 { vec2!(21.0, 12.0), vec2!(3.0, 4.0) }, glam::Vec2 { vec2(21.0, 12.0), vec2(3.0, 4.0) });

main!(library_benchmark_groups = neg, add, sub, mul, div, rem);
