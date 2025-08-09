use std::{ops::*, time::Duration};

use criterion::*;
use ggmath::{VecAligned, VecPacked};
use repetitive::repetitive;

fn benchmark(c: &mut Criterion) {
    repetitive! {
        @for [
            prim,
            prim_fn_prefix,
            prim_glam_prefix,

            prim_values,

            prim_has_bitwise,
            prim_has_bitshift,
            prim_has_arithmetic,
            prim_has_signed_arithmetic,

            prim_has_glam_vec3a,
        ] in [
            ['f32, 'f, "", [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], false, false, true, true, true],
            ['f64, 'd, 'D, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], false, false, true, true, false],

            ['u8, 'u8, 'U8, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],
            ['u16, 'u16, 'U16, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],
            ['u32, 'u32, 'U, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],
            ['u64, 'u64, 'U64, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],
            ['usize, 'usize, 'USize, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],

            ['i8, 'i8, 'I8, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],
            ['i16, 'i16, 'I16, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],
            ['i32, 'i32, 'I, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],
            ['i64, 'i64, 'I64, [1, 2, 3, 4, 5, 6, 7, 8], true, false, true, false, false],

            ['bool, 'b, 'B, [true, false, true, false, true, false, true, false], true, false, false, false, true],
        ] {
            let ggmath_left2 = ggmath::Vector::<2, @prim, VecPacked>::from_array([@(prim_values[0]), @(prim_values[1])]);
            let ggmath_right2 = ggmath::Vector::<2, @prim, VecPacked>::from_array([@(prim_values[2]), @(prim_values[3])]);
            let ggmath_left3 = ggmath::Vector::<3, @prim, VecPacked>::from_array([@(prim_values[0]), @(prim_values[1]), @(prim_values[2])]);
            let ggmath_right3 = ggmath::Vector::<3, @prim, VecPacked>::from_array([@(prim_values[3]), @(prim_values[4]), @(prim_values[5])]);
            let ggmath_left4 = ggmath::Vector::<4, @prim, VecAligned>::from_array([@(prim_values[0]), @(prim_values[1]), @(prim_values[2]), @(prim_values[3])]);
            let ggmath_right4 = ggmath::Vector::<4, @prim, VecAligned>::from_array([@(prim_values[4]), @(prim_values[5]), @(prim_values[6]), @(prim_values[7])]);

            let glam_left2 = glam::@[prim_glam_prefix 'Vec2]::new(@(prim_values[0]), @(prim_values[1]));
            let glam_right2 = glam::@[prim_glam_prefix 'Vec2]::new(@(prim_values[2]), @(prim_values[3]));
            let glam_left3 = glam::@[prim_glam_prefix 'Vec3]::new(@(prim_values[0]), @(prim_values[1]), @(prim_values[2]));
            let glam_right3 = glam::@[prim_glam_prefix 'Vec3]::new(@(prim_values[3]), @(prim_values[4]), @(prim_values[5]));
            let glam_left4 = glam::@[prim_glam_prefix 'Vec4]::new(@(prim_values[0]), @(prim_values[1]), @(prim_values[2]), @(prim_values[3]));
            let glam_right4 = glam::@[prim_glam_prefix 'Vec4]::new(@(prim_values[4]), @(prim_values[5]), @(prim_values[6]), @(prim_values[7]));

            @if prim_has_glam_vec3a {
                let ggmath_left3a = ggmath::Vector::<3, @prim, VecAligned>::from_array([@(prim_values[0]), @(prim_values[1]), @(prim_values[2])]);
                let ggmath_right3a = ggmath::Vector::<3, @prim, VecAligned>::from_array([@(prim_values[3]), @(prim_values[4]), @(prim_values[5])]);

                let glam_left3a = glam::@[prim_glam_prefix 'Vec3A]::new(@(prim_values[0]), @(prim_values[1]), @(prim_values[2]));
                let glam_right3a = glam::@[prim_glam_prefix 'Vec3A]::new(@(prim_values[3]), @(prim_values[4]), @(prim_values[5]));
            }

            @if prim_has_bitwise {
                @for un_bitwise_op in ['not] {
                    @if prim_has_glam_vec3a {
                        benchmark_fn(
                            @str[prim_fn_prefix "vec3a_" un_bitwise_op],
                            || ggmath_left3a.@un_bitwise_op(),
                            || glam_left3a.@un_bitwise_op(),
                            c,
                        );
                    }

                    benchmark_fn(
                        @str[prim_fn_prefix "vec4a_" un_bitwise_op],
                        || ggmath_left4.@un_bitwise_op(),
                        || glam_left4.@un_bitwise_op(),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec2a_" un_bitwise_op],
                        || ggmath_left2.@un_bitwise_op(),
                        || glam_left2.@un_bitwise_op(),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec3p_" un_bitwise_op],
                        || ggmath_left3.@un_bitwise_op(),
                        || glam_left3.@un_bitwise_op(),
                        c,
                    );
                }
            }

            @if prim_has_signed_arithmetic {
                @for un_arithmetic_op in ['neg] {
                    @if prim_has_glam_vec3a {
                        benchmark_fn(
                            @str[prim_fn_prefix "vec3a_" un_arithmetic_op],
                            || ggmath_left3a.@un_arithmetic_op(),
                            || glam_left3a.@un_arithmetic_op(),
                            c,
                        );
                    }

                    benchmark_fn(
                        @str[prim_fn_prefix "vec4a_" un_arithmetic_op],
                        || ggmath_left4.@un_arithmetic_op(),
                        || glam_left4.@un_arithmetic_op(),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec2a_" un_arithmetic_op],
                        || ggmath_left2.@un_arithmetic_op(),
                        || glam_left2.@un_arithmetic_op(),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec3p_" un_arithmetic_op],
                        || ggmath_left3.@un_arithmetic_op(),
                        || glam_left3.@un_arithmetic_op(),
                        c,
                    );
                }
            }

            @if prim_has_bitwise {
                @for bin_bitwise_op in ['bitand, 'bitor, 'bitxor] {
                    @if prim_has_glam_vec3a {
                        benchmark_fn(
                            @str[prim_fn_prefix "vec3a_" bin_bitwise_op],
                            || ggmath_left3a.@bin_bitwise_op(ggmath_right3a),
                            || glam_left3a.@bin_bitwise_op(glam_right3a),
                            c,
                        );
                    }

                    benchmark_fn(
                        @str[prim_fn_prefix "vec4a_" bin_bitwise_op],
                        || ggmath_left4.@bin_bitwise_op(ggmath_right4),
                        || glam_left4.@bin_bitwise_op(glam_right4),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec2a_" bin_bitwise_op],
                        || ggmath_left2.@bin_bitwise_op(ggmath_right2),
                        || glam_left2.@bin_bitwise_op(glam_right2),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec3p_" bin_bitwise_op],
                        || ggmath_left3.@bin_bitwise_op(ggmath_right3),
                        || glam_left3.@bin_bitwise_op(glam_right3),
                        c,
                    );
                }
            }

            @if prim_has_bitshift {
                @for bin_bitshift_op in ['shl, 'shr] {
                    @if prim_has_glam_vec3a {
                        benchmark_fn(
                            @str[prim_fn_prefix "vec3a_" bin_bitshift_op],
                            || ggmath_left3a.@bin_bitshift_op(ggmath_right3a),
                            || glam_left3a.@bin_bitshift_op(glam_right3a),
                            c,
                        );
                    }

                    benchmark_fn(
                        @str[prim_fn_prefix "vec4a_" bin_bitshift_op],
                        || ggmath_left4.@bin_bitshift_op(ggmath_right4),
                        || glam_left4.@bin_bitshift_op(glam_right4),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec2a_" bin_bitshift_op],
                        || ggmath_left2.@bin_bitshift_op(ggmath_right2),
                        || glam_left2.@bin_bitshift_op(glam_right2),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec3p_" bin_bitshift_op],
                        || ggmath_left3.@bin_bitshift_op(ggmath_right3),
                        || glam_left3.@bin_bitshift_op(glam_right3),
                        c,
                    );
                }
            }

            @if prim_has_arithmetic {
                @for bin_arithmetic_op in ['add, 'sub, 'mul, 'div, 'rem] {
                    @if prim_has_glam_vec3a {
                        benchmark_fn(
                            @str[prim_fn_prefix "vec3a_" bin_arithmetic_op],
                            || ggmath_left3a.@bin_arithmetic_op(ggmath_right3a),
                            || glam_left3a.@bin_arithmetic_op(glam_right3a),
                            c,
                        );
                    }

                    benchmark_fn(
                        @str[prim_fn_prefix "vec4a_" bin_arithmetic_op],
                        || ggmath_left4.@bin_arithmetic_op(ggmath_right4),
                        || glam_left4.@bin_arithmetic_op(glam_right4),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec2a_" bin_arithmetic_op],
                        || ggmath_left2.@bin_arithmetic_op(ggmath_right2),
                        || glam_left2.@bin_arithmetic_op(glam_right2),
                        c,
                    );

                    benchmark_fn(
                        @str[prim_fn_prefix "vec3p_" bin_arithmetic_op],
                        || ggmath_left3.@bin_arithmetic_op(ggmath_right3),
                        || glam_left3.@bin_arithmetic_op(glam_right3),
                        c,
                    );
                }
            }
        }
    }
}

fn benchmark_fn<O1, O2, R1, R2>(name: &str, ggmath: R1, glam: R2, c: &mut Criterion)
where
    R1: FnMut() -> O1 + Copy,
    R2: FnMut() -> O2 + Copy,
{
    let mut group = c.benchmark_group(name);

    group.bench_function("ggmath", |b| b.iter(ggmath));
    group.bench_function("glam", |b| b.iter(glam));
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(300)
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(100)).confidence_level(0.8);
    targets = benchmark
);
criterion_main!(benches);
