use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use glam::Vec3Swizzles;

criterion_main!(benches);

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_millis(500));
    targets = fvec3
}

fn fvec3(c: &mut Criterion) {
    let ggmath1: ggmath::Vector<3, f32, ggmath::VecAligned> = ggmath::vec3!(1.0, 40.5, 499.0);
    let ggmath2: ggmath::Vector<3, f32, ggmath::VecAligned> = ggmath::vec3!(2.0, 21.5, 534.0);

    let glam1: glam::Vec3A = glam::Vec3A::new(1.0, 40.5, 499.0);
    let glam2: glam::Vec3A = glam::Vec3A::new(2.0, 21.5, 534.0);

    let wide1 = wide::f32x4::new([1.0, 40.5, 499.0, 0.0]);
    #[expect(unused)]
    let wide2 = wide::f32x4::new([2.0, 21.5, 534.0, 0.0]);

    let mut group = c.benchmark_group("fvec3_splat");
    group.bench_function("ggmath", |b| {
        b.iter(|| ggmath::Vector::<3, f32, ggmath::VecAligned>::splat(1.0))
    });
    group.bench_function("glam", |b| b.iter(|| glam::Vec3A::splat(1.0)));
    group.finish();

    // swizzle

    let mut group = c.benchmark_group("fvec3_swizzle2");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.xz()));
    group.bench_function("glam", |b| b.iter(|| glam1.xz()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_swizzle3");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.xzy()));
    group.bench_function("glam", |b| b.iter(|| glam1.xzy()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_swizzle4");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.xyzy()));
    group.bench_function("glam", |b| b.iter(|| glam1.xyzy()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_constructor");
    group.bench_function("ggmath", |b| {
        b.iter(|| ggmath::vec3!(ggmath1.xz() + ggmath2.xy(), 499.0))
    });
    group.bench_function("glam", |b| {
        b.iter(|| (glam1.xz() + glam2.xy()).extend(499.0))
    });
    group.finish();

    // operators

    let mut group = c.benchmark_group("fvec3_neg");
    group.bench_function("ggmath", |b| b.iter(|| -ggmath1));
    group.bench_function("glam", |b| b.iter(|| -glam1));
    group.finish();

    let mut group = c.benchmark_group("fvec3_add");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 + ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 + glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec3_sub");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 - ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 - glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec3_mul");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 * ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 * glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec3_div");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 / ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 / glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec3_rem");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 % ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 % glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec3_eq");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 == ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 == glam2));
    group.finish();

    let mut group = c.benchmark_group("fvec3_ne");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1 != ggmath2));
    group.bench_function("glam", |b| b.iter(|| glam1 != glam2));
    group.finish();

    // cmp masks

    let mut group = c.benchmark_group("fvec3_eq_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.eq_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpeq(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_ne_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.ne_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpne(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_lt_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.lt_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmplt(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_gt_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.gt_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpgt(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_le_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.le_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmple(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_ge_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.ge_mask(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpge(glam2)));
    group.finish();

    // min/max/clamp

    let mut group = c.benchmark_group("fvec3_min");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.min(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.min(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_max");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.max(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.max(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_clamp");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.clamp(ggmath1, ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.clamp(glam1, glam2)));
    group.finish();

    // generic functions

    let mut group = c.benchmark_group("fvec3_sum");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.sum()));
    group.bench_function("glam", |b| b.iter(|| glam1.element_sum()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_product");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.product()));
    group.bench_function("glam", |b| b.iter(|| glam1.element_product()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_mag_sq");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.mag_sq()));
    group.bench_function("glam", |b| b.iter(|| glam1.length_squared()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_dot");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.dot(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.dot(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_abs_diff");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.abs_diff(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| (glam1 - glam2).abs()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_distance_sq");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.distance_sq(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.distance_squared(glam2)));
    group.finish();

    // float functions

    let mut group = c.benchmark_group("fvec3_abs");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.abs()));
    group.bench_function("glam", |b| b.iter(|| glam1.abs()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_signum");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.signum()));
    group.bench_function("glam", |b| b.iter(|| glam1.signum()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_positive_sign_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.positive_sign_mask()));
    group.bench_function("glam", |b| b.iter(|| glam1.cmpge(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_negative_sign_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.negative_sign_mask()));
    group.bench_function("glam", |b| b.iter(|| glam1.cmple(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_sqrt");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.sqrt()));
    group.bench_function("wide", |b| b.iter(|| wide1.sqrt()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_mag");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.mag()));
    group.bench_function("glam", |b| b.iter(|| glam1.length()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_distance");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.distance(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.distance(glam2)));
    group.finish();

    let mut group = c.benchmark_group("fvec3_is_nan");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.is_nan()));
    group.bench_function("glam", |b| b.iter(|| glam1.is_nan()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_is_finite");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.is_finite()));
    group.bench_function("glam", |b| b.iter(|| glam1.is_finite()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_nan_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.nan_mask()));
    group.bench_function("glam", |b| b.iter(|| glam1.is_nan_mask()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_finite_mask");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.finite_mask()));
    group.bench_function("glam", |b| b.iter(|| glam1.is_finite_mask()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_cross");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.cross(ggmath2)));
    group.bench_function("glam", |b| b.iter(|| glam1.cross(glam2)));
    group.finish();

    // rounding functions

    let mut group = c.benchmark_group("fvec3_round");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.round()));
    group.bench_function("glam", |b| b.iter(|| glam1.round()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_floor");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.floor()));
    group.bench_function("glam", |b| b.iter(|| glam1.floor()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_ceil");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.ceil()));
    group.bench_function("glam", |b| b.iter(|| glam1.ceil()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_trunc");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.trunc()));
    group.bench_function("glam", |b| b.iter(|| glam1.trunc()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_fract");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.fract()));
    group.bench_function("glam", |b| b.iter(|| glam1.fract()));
    group.finish();

    // trigonometric functions

    let mut group = c.benchmark_group("fvec3_sin");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.sin()));
    group.bench_function("wide", |b| b.iter(|| wide1.sin()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_cos");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.cos()));
    group.bench_function("wide", |b| b.iter(|| wide1.cos()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_tan");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.tan()));
    group.bench_function("wide", |b| b.iter(|| wide1.tan()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_asin");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.asin()));
    group.bench_function("wide", |b| b.iter(|| wide1.asin()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_acos");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.acos()));
    group.bench_function("wide", |b| b.iter(|| wide1.acos()));
    group.finish();

    let mut group = c.benchmark_group("fvec3_atan");
    group.bench_function("ggmath", |b| b.iter(|| ggmath1.atan()));
    group.bench_function("wide", |b| b.iter(|| wide1.atan()));
    group.finish();
}
