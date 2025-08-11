use std::time::Duration;

use criterion::*;
use ggmath::*;
use repetitive::repetitive;

fn benchmark(c: &mut Criterion) {
    repetitive! {
        // Swizzle isn't concerned about the scalar type, just its size.
        // This means it is only necessary to benchmark one type per size.
        @for prim in ['u8, 'u16, 'u32, 'u64, 'u128] {
            let ggmath2 = Vec2P::<@prim>::from_array([20, 1]);
            let ggmath3 = Vec3P::<@prim>::from_array([20, 1, 2]);
            let ggmath4 = Vec4P::<@prim>::from_array([20, 1, 2, 3]);
            let ggmath2p = Vec2P::<@prim>::from_array([20, 1]);
            let ggmath3p = Vec3P::<@prim>::from_array([20, 1, 2]);
            let ggmath4p = Vec4P::<@prim>::from_array([20, 1, 2, 3]);

            // Vec2
            c.bench_function(
                @str[prim " vec2_to_vec2_swizzle"],
                |b| b.iter(|| ggmath2.yx()),
            );
            c.bench_function(
                @str[prim " vec2_to_vec3_swizzle"],
                |b| b.iter(|| ggmath2.yyx()),
            );
            c.bench_function(
                @str[prim " vec2_to_vec4_swizzle"],
                |b| b.iter(|| ggmath2.yyxy()),
            );

            // Vec3
            c.bench_function(
                @str[prim " vec3_to_vec2_swizzle"],
                |b| b.iter(|| ggmath3.zy()),
            );
            c.bench_function(
                @str[prim " vec3_to_vec3_swizzle"],
                |b| b.iter(|| ggmath3.zyx()),
            );
            c.bench_function(
                @str[prim " vec3_to_vec4_swizzle"],
                |b| b.iter(|| ggmath3.zyxy()),
            );

            // Vec4
            c.bench_function(
                @str[prim " vec4_to_vec2_swizzle"],
                |b| b.iter(|| ggmath4.wz()),
            );
            c.bench_function(
                @str[prim " vec4_to_vec3_swizzle"],
                |b| b.iter(|| ggmath4.wyy()),
            );
            c.bench_function(
                @str[prim " vec4_to_vec4_swizzle"],
                |b| b.iter(|| ggmath4.wzyy()),
            );

            // Vec2P
            c.bench_function(
                @str[prim " vec2p_to_vec2p_swizzle"],
                |b| b.iter(|| ggmath2p.yx()),
            );
            c.bench_function(
                @str[prim " vec2p_to_vec3p_swizzle"],
                |b| b.iter(|| ggmath2p.yyx()),
            );
            c.bench_function(
                @str[prim " vec2p_to_vec4p_swizzle"],
                |b| b.iter(|| ggmath2p.yyxy()),
            );

            // Vec3P
            c.bench_function(
                @str[prim " vec3p_to_vec2p_swizzle"],
                |b| b.iter(|| ggmath3p.zy()),
            );
            c.bench_function(
                @str[prim " vec3p_to_vec3p_swizzle"],
                |b| b.iter(|| ggmath3p.zyx()),
            );
            c.bench_function(
                @str[prim " vec3p_to_vec4p_swizzle"],
                |b| b.iter(|| ggmath3p.zyxy()),
            );

            // Vec4P
            c.bench_function(
                @str[prim " vec4p_to_vec2p_swizzle"],
                |b| b.iter(|| ggmath4p.wz()),
            );
            c.bench_function(
                @str[prim " vec4p_to_vec3p_swizzle"],
                |b| b.iter(|| ggmath4p.wyy()),
            );
            c.bench_function(
                @str[prim " vec4p_to_vec4p_swizzle"],
                |b| b.iter(|| ggmath4p.wzyy()),
            );
        }
    }
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
