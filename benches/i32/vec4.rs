use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};

#[path = "../bench_operator.rs"]
mod bench_operator;

bench_unary_operator!(neg { - } for ggmath::i32::IVec4 { vec4!(1, 2, 3, 4) }, wide::i32x4 { i32x4::new([1, 2, 3, 4]) });

bench_binary_operator!(add { + } for ggmath::i32::IVec4 { vec4!(1, 2, 3, 4), vec4!(5, 6, 7, 8) }, wide::i32x4 { i32x4::new([1, 2, 3, 4]), i32x4::new([5, 6, 7, 8]) });
bench_binary_operator!(sub { - } for ggmath::i32::IVec4 { vec4!(1, 2, 3, 4), vec4!(5, 6, 7, 8) }, wide::i32x4 { i32x4::new([1, 2, 3, 4]), i32x4::new([5, 6, 7, 8]) });
bench_binary_operator!(mul { * } for ggmath::i32::IVec4 { vec4!(1, 2, 3, 4), vec4!(5, 6, 7, 8) }, wide::i32x4 { i32x4::new([1, 2, 3, 4]), i32x4::new([5, 6, 7, 8]) });
bench_binary_operator!(div { / } for ggmath::i32::IVec4 { vec4!(21, 12, 13, 14), vec4!(5, 6, 7, 8) }, glam::IVec4 { ivec4(21, 12, 13, 14), ivec4(5, 6, 7, 8) });
bench_binary_operator!(rem { % } for ggmath::i32::IVec4 { vec4!(21, 12, 13, 14), vec4!(5, 6, 7, 8) }, glam::IVec4 { ivec4(21, 12, 13, 14), ivec4(5, 6, 7, 8) });

main!(library_benchmark_groups = neg, add, sub, mul, div, rem);
