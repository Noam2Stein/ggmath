use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn random_math(c: &mut Criterion) {
    let mut group = c.benchmark_group("f32_random_math");

    let input = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
    ];

    group.bench_function("ggmath", |b| {
        b.iter(|| -> ggmath::f32_aliases::FVec4 {
            use ggmath::*;

            let input = black_box(input);

            let a = vec2!(input[0], input[1]);
            let b = vec3!(input[2], input[3], input[4]);
            let c = vec4!(input[5], a, input[6]);
            let d = vec3s!(input[7], input[8], input[9]);

            let e = a.xxy() + b + b + b + b + b;
            let f = e.xyzy() * c.yzwx() + d.as_simd().xxxy();
            let g = (-f).reverse() / c % c.xxxw();
            let h = g.xz() * b.zy();

            let i = h.xyxy() + c + f + g + e.xxyz();

            black_box(i)
        })
    });

    group.bench_function("glam", |b| {
        b.iter(|| -> glam::Vec4 {
            use glam::*;

            let input = black_box(input);

            let a = Vec2::new(input[0], input[1]);
            let b = Vec3A::new(input[2], input[3], input[4]);
            let c = Vec4::new(input[5], a.x, a.y, input[6]);
            let d = Vec3::new(input[7], input[8], input[9]);

            let e = a.xxy().to_vec3a() + b + b + b + b + b;
            let f = e.xyzy() * c.yzwx() + d.to_vec3a().xxxy();
            let g = (-f).wzyx() / c % c.xxxw();
            let h = g.xz() * b.zy();

            let i = h.xyxy() + c + f + g + e.xxyz();

            black_box(i)
        })
    });
}

criterion_group!(name = benches; config = Criterion::default(); targets = random_math);
criterion_main!(benches);
