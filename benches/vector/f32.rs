use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn bench(c: &mut Criterion) {
    ////////////////////////////////////////////////////////////////////////////////
    // Simple
    ////////////////////////////////////////////////////////////////////////////////

    c.bench_function("ggmath_vector_f32_simple", |b| {
        b.iter(|| {
            use ggmath::f32::{Vec2f, Vec3f, Vec3fA, Vec4fA};

            let mut v2 = black_box(Vec2f::ZERO);
            let mut v3 = black_box(Vec3f::ZERO);
            let mut v3a = black_box(Vec3fA::ZERO);
            let mut v4a = black_box(Vec4fA::ZERO);

            v2 = -v2;
            v2 = v2 + v2;
            v2 = v2 - v2;
            v2 = v2 * v2;
            v2 = v2 / v2;
            v2 = v2.abs();
            v2 = v2.recip();
            v2 = v2.max(black_box(v2));
            v2 = v2.min(black_box(v2));
            v2 = v2.clamp(black_box(v2), black_box(v2));
            v2 = v2.signum();
            v2 = v2.copysign(black_box(v2));

            v3 = -v3;
            v3 = v3 + v3;
            v3 = v3 - v3;
            v3 = v3 * v3;
            v3 = v3 / v3;
            v3 = v3.abs();
            v3 = v3.recip();
            v3 = v3.max(black_box(v3));
            v3 = v3.min(black_box(v3));
            v3 = v3.clamp(black_box(v3), black_box(v3));
            v3 = v3.signum();
            v3 = v3.copysign(black_box(v3));

            v3a = -v3a;
            v3a = v3a + v3a;
            v3a = v3a - v3a;
            v3a = v3a * v3a;
            v3a = v3a / v3a;
            v3a = v3a.abs();
            v3a = v3a.recip();
            v3a = v3a.max(black_box(v3a));
            v3a = v3a.min(black_box(v3a));
            v3a = v3a.clamp(black_box(v3a), black_box(v3a));
            v3a = v3a.signum();
            v3a = v3a.copysign(black_box(v3a));

            v4a = -v4a;
            v4a = v4a + v4a;
            v4a = v4a - v4a;
            v4a = v4a * v4a;
            v4a = v4a / v4a;
            v4a = v4a.abs();
            v4a = v4a.recip();
            v4a = v4a.max(black_box(v4a));
            v4a = v4a.min(black_box(v4a));
            v4a = v4a.clamp(black_box(v4a), black_box(v4a));
            v4a = v4a.signum();
            v4a = v4a.copysign(black_box(v4a));

            (v2, v3, v3a, v4a)
        })
    });

    c.bench_function("glam_vector_f32_simple", |b| {
        b.iter(|| {
            use glam::{Vec2, Vec3, Vec3A, Vec4};

            let mut v2 = black_box(Vec2::ZERO);
            let mut v3 = black_box(Vec3::ZERO);
            let mut v3a = black_box(Vec3A::ZERO);
            let mut v4a = black_box(Vec4::ZERO);

            v2 = -v2;
            v2 = v2 + v2;
            v2 = v2 - v2;
            v2 = v2 * v2;
            v2 = v2 / v2;
            v2 = v2.abs();
            v2 = v2.recip();
            v2 = v2.max(black_box(v2));
            v2 = v2.min(black_box(v2));
            v2 = v2.clamp(black_box(v2), black_box(v2));
            v2 = v2.signum();
            v2 = v2.copysign(black_box(v2));

            v3 = -v3;
            v3 = v3 + v3;
            v3 = v3 - v3;
            v3 = v3 * v3;
            v3 = v3 / v3;
            v3 = v3.abs();
            v3 = v3.recip();
            v3 = v3.max(black_box(v3));
            v3 = v3.min(black_box(v3));
            v3 = v3.clamp(black_box(v3), black_box(v3));
            v3 = v3.signum();
            v3 = v3.copysign(black_box(v3));

            v3a = -v3a;
            v3a = v3a + v3a;
            v3a = v3a - v3a;
            v3a = v3a * v3a;
            v3a = v3a / v3a;
            v3a = v3a.abs();
            v3a = v3a.recip();
            v3a = v3a.max(black_box(v3a));
            v3a = v3a.min(black_box(v3a));
            v3a = v3a.clamp(black_box(v3a), black_box(v3a));
            v3a = v3a.signum();
            v3a = v3a.copysign(black_box(v3a));

            v4a = -v4a;
            v4a = v4a + v4a;
            v4a = v4a - v4a;
            v4a = v4a * v4a;
            v4a = v4a / v4a;
            v4a = v4a.abs();
            v4a = v4a.recip();
            v4a = v4a.max(black_box(v4a));
            v4a = v4a.min(black_box(v4a));
            v4a = v4a.clamp(black_box(v4a), black_box(v4a));
            v4a = v4a.signum();
            v4a = v4a.copysign(black_box(v4a));

            (v2, v3, v3a, v4a)
        })
    });

    c.bench_function("ggmath_vector_f32_simple2", |b| {
        b.iter(|| {
            use ggmath::f32::{Vec3fA, Vec4fA};

            let mut v3a = black_box(Vec3fA::ZERO);
            let mut v4a = black_box(Vec4fA::ZERO);

            v3a = v3a + v3a;
            v3a = v3a.sqrt();
            v3a = v3a - v3a;

            v4a = v4a + v4a;
            v4a = v4a.sqrt();
            v4a = v4a - v4a;

            (v3a, v4a)
        })
    });

    c.bench_function("wide_vector_f32_simple2", |b| {
        b.iter(|| {
            use wide::f32x4;

            let mut v3a = black_box(f32x4::ZERO);
            let mut v4a = black_box(f32x4::ZERO);

            v3a = v3a + v3a;
            v3a = v3a.sqrt();
            v3a = v3a - v3a;

            v4a = v4a + v4a;
            v4a = v4a.sqrt();
            v4a = v4a - v4a;

            (v3a, v4a)
        })
    });

    ////////////////////////////////////////////////////////////////////////////////
    // Horizontal
    ////////////////////////////////////////////////////////////////////////////////

    c.bench_function("ggmath_vector_f32_horizontal", |b| {
        b.iter(|| {
            use ggmath::f32::{Vec2f, Vec3f, Vec3fA, Vec4fA};

            let mut v2 = black_box(Vec2f::ZERO);
            let mut v3 = black_box(Vec3f::ZERO);
            let mut v3a = black_box(Vec3fA::ZERO);
            let mut v4a = black_box(Vec4fA::ZERO);

            v2 = v2.with_x((black_box(v2) == black_box(v2)) as u8 as f32);
            v2 = v2.with_y((black_box(v2) != black_box(v2)) as u8 as f32);
            v2 = v2.with_x(v2.element_sum());
            v2 = v2.with_y(v2.element_product());
            v2 = v2.with_x(v2.max_element());
            v2 = v2.with_y(v2.min_element());
            v2 = v2.with_x(v2.dot(black_box(v2)));
            v2 = v2.with_y(v2.length());
            v2 = v2.with_x(v2.length_squared());
            v2 = v2.normalize();
            v2 = v2.try_normalize().unwrap_or(Vec2f::ZERO);
            v2 = v2.normalize_or(Vec2f::ZERO);

            v3 = v3.with_x((black_box(v3) == black_box(v3)) as u8 as f32);
            v3 = v3.with_y((black_box(v3) != black_box(v3)) as u8 as f32);
            v3 = v3.with_x(v3.element_sum());
            v3 = v3.with_y(v3.element_product());
            v3 = v3.with_x(v3.max_element());
            v3 = v3.with_y(v3.min_element());
            v3 = v3.with_x(v3.dot(black_box(v3)));
            v3 = v3.with_y(v3.length());
            v3 = v3.with_x(v3.length_squared());
            v3 = v3.normalize();
            v3 = v3.try_normalize().unwrap_or(Vec3f::ZERO);
            v3 = v3.normalize_or(Vec3f::ZERO);

            v3a = v3a.with_x((black_box(v3a) == black_box(v3a)) as u8 as f32);
            v3a = v3a.with_y((black_box(v3a) != black_box(v3a)) as u8 as f32);
            v3a = v3a.with_x(v3a.element_sum());
            v3a = v3a.with_y(v3a.element_product());
            v3a = v3a.with_x(v3a.max_element());
            v3a = v3a.with_y(v3a.min_element());
            v3a = v3a.with_x(v3a.dot(black_box(v3a)));
            v3a = v3a.with_y(v3a.length());
            v3a = v3a.with_x(v3a.length_squared());
            v3a = v3a.normalize();
            v3a = v3a.try_normalize().unwrap_or(Vec3fA::ZERO);
            v3a = v3a.normalize_or(Vec3fA::ZERO);

            v4a = v4a.with_x((black_box(v4a) == black_box(v4a)) as u8 as f32);
            v4a = v4a.with_y((black_box(v4a) != black_box(v4a)) as u8 as f32);
            v4a = v4a.with_x(v4a.element_sum());
            v4a = v4a.with_y(v4a.element_product());
            v4a = v4a.with_x(v4a.max_element());
            v4a = v4a.with_y(v4a.min_element());
            v4a = v4a.with_x(v4a.dot(black_box(v4a)));
            v4a = v4a.with_y(v4a.length());
            v4a = v4a.with_x(v4a.length_squared());
            v4a = v4a.normalize();
            v4a = v4a.try_normalize().unwrap_or(Vec4fA::ZERO);
            v4a = v4a.normalize_or(Vec4fA::ZERO);

            (v2, v3, v3a, v4a)
        })
    });

    c.bench_function("glam_vector_f32_horizontal", |b| {
        b.iter(|| {
            use glam::{Vec2, Vec3, Vec3A, Vec4};

            let mut v2 = black_box(Vec2::ZERO);
            let mut v3 = black_box(Vec3::ZERO);
            let mut v3a = black_box(Vec3A::ZERO);
            let mut v4a = black_box(Vec4::ZERO);

            v2 = v2.with_x((black_box(v2) == black_box(v2)) as u8 as f32);
            v2 = v2.with_y((black_box(v2) != black_box(v2)) as u8 as f32);
            v2 = v2.with_x(v2.element_sum());
            v2 = v2.with_y(v2.element_product());
            v2 = v2.with_x(v2.max_element());
            v2 = v2.with_y(v2.min_element());
            v2 = v2.with_x(v2.dot(black_box(v2)));
            v2 = v2.with_y(v2.length());
            v2 = v2.with_x(v2.length_squared());
            v2 = v2.normalize();
            v2 = v2.try_normalize().unwrap_or(Vec2::ZERO);
            v2 = v2.normalize_or(Vec2::ZERO);

            v3 = v3.with_x((black_box(v3) == black_box(v3)) as u8 as f32);
            v3 = v3.with_y((black_box(v3) != black_box(v3)) as u8 as f32);
            v3 = v3.with_x(v3.element_sum());
            v3 = v3.with_y(v3.element_product());
            v3 = v3.with_x(v3.max_element());
            v3 = v3.with_y(v3.min_element());
            v3 = v3.with_x(v3.dot(black_box(v3)));
            v3 = v3.with_y(v3.length());
            v3 = v3.with_x(v3.length_squared());
            v3 = v3.normalize();
            v3 = v3.try_normalize().unwrap_or(Vec3::ZERO);
            v3 = v3.normalize_or(Vec3::ZERO);

            v3a = v3a.with_x((black_box(v3a) == black_box(v3a)) as u8 as f32);
            v3a = v3a.with_y((black_box(v3a) != black_box(v3a)) as u8 as f32);
            v3a = v3a.with_x(v3a.element_sum());
            v3a = v3a.with_y(v3a.element_product());
            v3a = v3a.with_x(v3a.max_element());
            v3a = v3a.with_y(v3a.min_element());
            v3a = v3a.with_x(v3a.dot(black_box(v3a)));
            v3a = v3a.with_y(v3a.length());
            v3a = v3a.with_x(v3a.length_squared());
            v3a = v3a.normalize();
            v3a = v3a.try_normalize().unwrap_or(Vec3A::ZERO);
            v3a = v3a.normalize_or(Vec3A::ZERO);

            v4a = v4a.with_x((black_box(v4a) == black_box(v4a)) as u8 as f32);
            v4a = v4a.with_y((black_box(v4a) != black_box(v4a)) as u8 as f32);
            v4a = v4a.with_x(v4a.element_sum());
            v4a = v4a.with_y(v4a.element_product());
            v4a = v4a.with_x(v4a.max_element());
            v4a = v4a.with_y(v4a.min_element());
            v4a = v4a.with_x(v4a.dot(black_box(v4a)));
            v4a = v4a.with_y(v4a.length());
            v4a = v4a.with_x(v4a.length_squared());
            v4a = v4a.normalize();
            v4a = v4a.try_normalize().unwrap_or(Vec4::ZERO);
            v4a = v4a.normalize_or(Vec4::ZERO);

            (v2, v3, v3a, v4a)
        })
    });

    ////////////////////////////////////////////////////////////////////////////////
    // Rounding
    ////////////////////////////////////////////////////////////////////////////////

    c.bench_function("ggmath_vector_f32_rounding", |b| {
        b.iter(|| {
            use ggmath::f32::{Vec2f, Vec3f, Vec3fA, Vec4fA};

            let mut v2 = black_box(Vec2f::ZERO);
            let mut v3 = black_box(Vec3f::ZERO);
            let mut v3a = black_box(Vec3fA::ZERO);
            let mut v4a = black_box(Vec4fA::ZERO);

            v2 = v2.floor() + v2.ceil() - v2.round() * v2.trunc() / v2.fract();
            v3 = v3.floor() + v3.ceil() - v3.round() * v3.trunc() / v3.fract();
            v3a = v3a.floor() + v3a.ceil() - v3a.round() * v3a.trunc() / v3a.fract();
            v4a = v4a.floor() + v4a.ceil() - v4a.round() * v4a.trunc() / v4a.fract();

            (v2, v3, v3a, v4a)
        })
    });

    c.bench_function("glam_vector_f32_rounding", |b| {
        b.iter(|| {
            use glam::{Vec2, Vec3, Vec3A, Vec4};

            let mut v2 = black_box(Vec2::ZERO);
            let mut v3 = black_box(Vec3::ZERO);
            let mut v3a = black_box(Vec3A::ZERO);
            let mut v4a = black_box(Vec4::ZERO);

            v2 = v2.floor() + v2.ceil() - v2.round() * v2.trunc() / v2.fract();
            v3 = v3.floor() + v3.ceil() - v3.round() * v3.trunc() / v3.fract();
            v3a = v3a.floor() + v3a.ceil() - v3a.round() * v3a.trunc() / v3a.fract();
            v4a = v4a.floor() + v4a.ceil() - v4a.round() * v4a.trunc() / v4a.fract();

            (v2, v3, v3a, v4a)
        })
    });

    ////////////////////////////////////////////////////////////////////////////////
    // Trigonometry
    ////////////////////////////////////////////////////////////////////////////////

    c.bench_function("ggmath_vector_f32_trigonometry", |b| {
        b.iter(|| {
            use ggmath::f32::{Vec3fA, Vec4fA};

            let mut v3a = black_box(Vec3fA::ZERO);
            let mut v4a = black_box(Vec4fA::ZERO);

            v3a = v3a.sin() + v3a.cos() - v3a.tan() * v3a.asin() / v3a.acos() - v3a.atan();
            v4a = v4a.sin() + v4a.cos() - v4a.tan() * v4a.asin() / v4a.acos() - v4a.atan();

            (v3a, v4a)
        })
    });

    c.bench_function("wide_vector_f32_trigonometry", |b| {
        b.iter(|| {
            use wide::f32x4;

            let mut v3a = black_box(f32x4::ZERO);
            let mut v4a = black_box(f32x4::ZERO);

            v3a = v3a.sin() + v3a.cos() - v3a.tan() * v3a.asin() / v3a.acos() - v3a.atan();
            v4a = v4a.sin() + v4a.cos() - v4a.tan() * v4a.asin() / v4a.acos() - v4a.atan();

            (v3a, v4a)
        })
    });

    ////////////////////////////////////////////////////////////////////////////////
    // Division
    ////////////////////////////////////////////////////////////////////////////////

    c.bench_function("ggmath_vector_f32_division", |b| {
        b.iter(|| {
            use ggmath::f32::{Vec2f, Vec3f, Vec3fA, Vec4fA};

            let mut v2 = black_box(Vec2f::ZERO);
            let mut v3 = black_box(Vec3f::ZERO);
            let mut v3a = black_box(Vec3fA::ZERO);
            let mut v4a = black_box(Vec4fA::ZERO);

            v2 = v2 % black_box(v2);
            v2 = v2.div_euclid(black_box(v2));
            v2 = v2.rem_euclid(black_box(v2));

            v3 = v3 % black_box(v3);
            v3 = v3.div_euclid(black_box(v3));
            v3 = v3.rem_euclid(black_box(v3));

            v3a = v3a % black_box(v3a);
            v3a = v3a.div_euclid(black_box(v3a));
            v3a = v3a.rem_euclid(black_box(v3a));

            v4a = v4a % black_box(v4a);
            v4a = v4a.div_euclid(black_box(v4a));
            v4a = v4a.rem_euclid(black_box(v4a));

            (v2, v3, v3a, v4a)
        })
    });

    c.bench_function("glam_vector_f32_division", |b| {
        b.iter(|| {
            use glam::{Vec2, Vec3, Vec3A, Vec4};

            let mut v2 = black_box(Vec2::ZERO);
            let mut v3 = black_box(Vec3::ZERO);
            let mut v3a = black_box(Vec3A::ZERO);
            let mut v4a = black_box(Vec4::ZERO);

            v2 = v2 % black_box(v2);
            v2 = v2.div_euclid(black_box(v2));
            v2 = v2.rem_euclid(black_box(v2));

            v3 = v3 % black_box(v3);
            v3 = v3.div_euclid(black_box(v3));
            v3 = v3.rem_euclid(black_box(v3));

            v3a = v3a % black_box(v3a);
            v3a = v3a.div_euclid(black_box(v3a));
            v3a = v3a.rem_euclid(black_box(v3a));

            v4a = v4a % black_box(v4a);
            v4a = v4a.div_euclid(black_box(v4a));
            v4a = v4a.rem_euclid(black_box(v4a));

            (v2, v3, v3a, v4a)
        })
    });
}

criterion_group!(name = benches; config = Criterion::default().sample_size(1000); targets = bench);

criterion_main!(benches);
