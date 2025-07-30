use std::{ops::*, time::Duration};

use criterion::*;
use ggmath::{VecAligned, VecPacked};
use macro_loop::macro_loop;

fn benchmark(c: &mut Criterion) {
    macro_loop! {
        @for [float, prefix, glam_prefix, has_vec3a] in [
            [f32, f, "", true],
            [f64, d, D, false],
        ] {
            let ggmath_left2 = ggmath::Vector::<2, @float, VecPacked>::from_array([1.0, 2.0]);
            let ggmath_right2 = ggmath::Vector::<2, @float, VecPacked>::from_array([3.0, 4.0]);
            let ggmath_left3 = ggmath::Vector::<3, @float, VecPacked>::from_array([1.0, 2.0, 3.0]);
            let ggmath_right3 = ggmath::Vector::<3, @float, VecPacked>::from_array([4.0, 5.0, 6.0]);
            let ggmath_left4 = ggmath::Vector::<4, @float, VecAligned>::from_array([1.0, 2.0, 3.0, 4.0]);
            let ggmath_right4 = ggmath::Vector::<4, @float, VecAligned>::from_array([5.0, 6.0, 7.0, 8.0]);

            let glam_left2 = glam::@[@glam_prefix Vec2]::new(1.0, 2.0);
            let glam_right2 = glam::@[@glam_prefix Vec2]::new(3.0, 4.0);
            let glam_left3 = glam::@[@glam_prefix Vec3]::new(1.0, 2.0, 3.0);
            let glam_right3 = glam::@[@glam_prefix Vec3]::new(4.0, 5.0, 6.0);
            let glam_left4 = glam::@[@glam_prefix Vec4]::new(1.0, 2.0, 3.0, 4.0);
            let glam_right4 = glam::@[@glam_prefix Vec4]::new(5.0, 6.0, 7.0, 8.0);

            @if @has_vec3a {
                let ggmath_left3a = ggmath::Vector::<3, @float, VecAligned>::from_array([1.0, 2.0, 3.0]);
                let ggmath_right3a = ggmath::Vector::<3, @float, VecAligned>::from_array([4.0, 5.0, 6.0]);

                let glam_left3a = glam::@[@glam_prefix Vec3A]::new(1.0, 2.0, 3.0);
                let glam_right3a = glam::@[@glam_prefix Vec3A]::new(4.0, 5.0, 6.0);
            }

            @for bin_op in [add, sub, mul, div, rem] {
                @if @has_vec3a {
                    benchmark_fn(
                        @("" + @prefix + "vec3a_" + @bin_op),
                        || ggmath_left3a.@bin_op(ggmath_right3a),
                        || glam_left3a.@bin_op(glam_right3a),
                        c,
                    );
                }

                benchmark_fn(
                    @("" + @prefix + "vec4a_" + @bin_op),
                    || ggmath_left4.@bin_op(ggmath_right4),
                    || glam_left4.@bin_op(glam_right4),
                    c,
                );

                benchmark_fn(
                    @("" + @prefix + "vec2a_" + @bin_op),
                    || ggmath_left2.@bin_op(ggmath_right2),
                    || glam_left2.@bin_op(glam_right2),
                    c,
                );

                benchmark_fn(
                    @("" + @prefix + "vec3p_" + @bin_op),
                    || ggmath_left3.@bin_op(ggmath_right3),
                    || glam_left3.@bin_op(glam_right3),
                    c,
                );
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
        .measurement_time(Duration::from_millis(100));
    targets = benchmark
);
criterion_main!(benches);
