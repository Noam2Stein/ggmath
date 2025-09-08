use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use glam::Vec2Swizzles;

criterion_main!(benches);

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_millis(500));
    targets = fvec2
}

fn fvec2(c: &mut Criterion) {
    let ggmath1: ggmath::Vector<2, f32, ggmath::VecAligned> = ggmath::vec2!(1.0, 40.5);
    let ggmath2: ggmath::Vector<2, f32, ggmath::VecAligned> = ggmath::vec2!(2.0, 21.5);

    let glam1: glam::Vec2 = glam::Vec2::new(1.0, 40.5);
    let glam2: glam::Vec2 = glam::Vec2::new(2.0, 21.5);

    let wide1 = wide::f32x4::new([1.0, 40.5, 0.0, 0.0]);
    #[expect(unused)]
    let wide2 = wide::f32x4::new([2.0, 21.5, 0.0, 0.0]);

    let mut group = c.benchmark_group("fvec2_splat");
    group.bench_function("ggmath", |b| {
        b.iter(|| ggmath::Vector::<2, f32, ggmath::VecAligned>::splat(1.0))
    });
    group.bench_function("glam", |b| b.iter(|| glam::Vec2::splat(1.0)));
    group.finish();

    // swizzle

    let mut group = c.benchmark_group("fvec2_swizzle2");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.xx()));
    group.bench_function("glam", |b| b.iter(|| glam1.xx()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_swizzle3");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.xyy()));
    group.bench_function("glam", |b| b.iter(|| glam1.xyy()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_swizzle4");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.xyxy()));
    group.bench_function("glam", |b| b.iter(|| glam1.xyxy()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_constructor");
    group.bench_function("ggmath", |b| {
        b.iter(|| ggmath::vec2!((ggmath1.xx() + ggmath2.xy())[0], 1.0))
    });
    group.bench_function("glam", |b| {
        b.iter(|| glam::Vec2::new((glam1.xx() + glam2.xy()).x, 1.0))
    });
    group.finish();

    // operators

    let mut group = c.benchmark_group("fvec2_neg");
    group.bench_function("ggmath", |b| b.iter(|| -ggmath1));
    group.bench_function("glam", |b| b.iter(|| -glam1));
    group.finish();

    let mut group = c.benchmark_group("fvec2_add");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 + ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 + glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec2_sub");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 - ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 - glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec2_mul");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 * ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 * glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec2_div");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 / ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 / glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec2_rem");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 % ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 % glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec2_eq");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 == ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 == glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec2_ne");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 != ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 != glam2));
    group.finish();

    // cmp masks

    let mut group = c.benchmark_group("fvec2_eq_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.eq_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpeq(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_ne_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.ne_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpne(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_lt_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.lt_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmplt(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_gt_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.gt_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpgt(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_le_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.le_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmple(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_ge_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.ge_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpge(glam2)));
    group.finish();

    // generic functions

    let mut group = c.benchmark_group("fvec2_sum");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.sum()));
    group.bench_function("glam", |b| b.iter(|| glam1.element_sum()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_product");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.product()));
    group.bench_function("glam", |b| b.iter(|| glam1.element_product()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_mag_sq");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.mag_sq()));
    group.bench_function("glam", |b| b.iter(|| glam1.length_squared()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_dot");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.dot(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.dot(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_abs_diff");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.abs_diff(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| (glam1 - glam2).abs()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_distance_sq");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.distance_sq(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.distance_squared(glam2)));
    group.finish();

    // float functions

    let mut group = c.benchmark_group("fvec2_abs");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.abs()));
    group.bench_function("glam", |b| b.iter(|| glam1.abs()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_signum");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.signum()));
    group.bench_function("glam", |b| b.iter(|| glam1.signum()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_positive_sign_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.positive_sign_mask()));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpge(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_negative_sign_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.negative_sign_mask()));
    group.bench_function("glam", |b| b.iter(|| glam1.cmple(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_sqrt");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.sqrt()));
    group.bench_function("wide", |b| b.iter(|| wide1.sqrt()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_mag");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.mag()));
    group.bench_function("glam", |b| b.iter(|| glam1.length()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_distance");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.distance(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.distance(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec2_is_nan");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.is_nan()));
    group.bench_function("glam", |b| b.iter(|| glam1.is_nan()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_is_finite");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.is_finite()));
    group.bench_function("glam", |b| b.iter(|| glam1.is_finite()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_nan_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.nan_mask()));
    group.bench_function("glam", |b| b.iter(|| glam1.is_nan_mask()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_finite_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.finite_mask()));
    group.bench_function("glam", |b| b.iter(|| glam1.is_finite_mask()));
    group.finish();

    // rounding functions

    let mut group = c.benchmark_group("fvec2_round");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.round()));
    group.bench_function("glam", |b| b.iter(|| glam1.round()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_floor");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.floor()));
    group.bench_function("glam", |b| b.iter(|| glam1.floor()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_ceil");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.ceil()));
    group.bench_function("glam", |b| b.iter(|| glam1.ceil()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_trunc");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.trunc()));
    group.bench_function("glam", |b| b.iter(|| glam1.trunc()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_fract");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.fract()));
    group.bench_function("glam", |b| b.iter(|| glam1.fract()));
    group.finish();

    // trigonometric functions

    let mut group = c.benchmark_group("fvec2_sin");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.sin()));
    group.bench_function("wide", |b| b.iter(|| wide1.sin()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_cos");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.cos()));
    group.bench_function("wide", |b| b.iter(|| wide1.cos()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_tan");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.tan()));
    group.bench_function("wide", |b| b.iter(|| wide1.tan()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_asin");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.asin()));
    group.bench_function("wide", |b| b.iter(|| wide1.asin()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_acos");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.acos()));
    group.bench_function("wide", |b| b.iter(|| wide1.acos()));
    group.finish();

    let mut group = c.benchmark_group("fvec2_atan");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.atan()));
    group.bench_function("wide", |b| b.iter(|| wide1.atan()));
    group.finish();
}
