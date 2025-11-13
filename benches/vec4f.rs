use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn arithmetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic");

    group.bench_function("ggmath", |b| {
        b.iter(|| {
            use ggmath::{f32::Vec4f, vec4};

            let v: Vec4f = black_box(vec4!(5.6, 1.7, 3.2, 11.11));

            let v = black_box(
                v + v.xzyw()
                    + vec4!(8.0, v.zz(), v.x)
                    + (v + v - v * v / v % v + 2.0 * 2.0 / 2.0 % 2.0)
                    + v.floor()
                    + v.ceil()
                    + v.round()
                    + v.trunc()
                    + v.fract()
                    + v.mul_add(v + v, v * v)
                    + v.div_euclid(v + v)
                    + v.rem_euclid(v * 2.0)
                    + v.recip().max(v + 1.3)
                    + v.min(v.midpoint(v + 111.4))
                    + v.clamp(v - 1.5, v.abs() - 0.5)
                    + v.signum().copysign(v - 0.22)
                    + v.element_sum()
                    + vec4!(v.element_product()).max_element()
                    + v.min_element()
                    + v.dot(v - 0.5)
                    + (v * v).length()
                    - (v + v).length_squared()
                    + v.normalize()
                    - v.try_normalize().unwrap_or(Vec4f::ZERO) * v.normalize_or(Vec4f::ONE),
            );

            let mut result = Vec4f::X;
            for _ in 0..(v.x % 4.0).round().abs() as usize {
                result += v;
            }

            black_box(result)
        })
    });

    group.bench_function("glam", |b| {
        b.iter(|| {
            use glam::{Vec4, Vec4Swizzles, vec4};

            let v: Vec4 = black_box(vec4(5.6, 1.7, 3.2, 11.11));

            let v = black_box(
                v + v.xzyw()
                    + vec4(8.0, v.z, v.z, v.x)
                    + (v + v - v * v / v % v + 2.0 * 2.0 / 2.0 % 2.0)
                    + v.floor()
                    + v.ceil()
                    + v.round()
                    + v.trunc()
                    + v.fract()
                    + v.mul_add(v + v, v * v)
                    + v.div_euclid(v + v)
                    + v.rem_euclid(v * 2.0)
                    + v.recip().max(v + 1.3)
                    + v.min(v.midpoint(v + 111.4))
                    + v.clamp(v - 1.5, v.abs() - 0.5)
                    + v.signum().copysign(v - 0.22)
                    + v.element_sum()
                    + Vec4::splat(v.element_product()).max_element()
                    + v.min_element()
                    + v.dot(v - 0.5)
                    + (v * v).length()
                    - (v + v).length_squared()
                    + v.normalize()
                    - v.try_normalize().unwrap_or(Vec4::ZERO) * v.normalize_or(Vec4::ONE),
            );

            let mut result = Vec4::X;
            for _ in 0..(v.x % 4.0).round().abs() as usize {
                result += v;
            }

            black_box(result)
        })
    });
}

criterion_group!(name = benches; config = Criterion::default(); targets = arithmetic);
criterion_main!(benches);
