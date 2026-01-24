use gungraun::{library_benchmark, library_benchmark_group, main};

main!(library_benchmark_groups = benches);
library_benchmark_group!(
    name = benches;
    benchmarks = ggmath_arithmetic, glam_arithmetic, ggmath_arithmetic_ext, glam_arithmetic_ext,
        ggmath_bitops, glam_bitops, ggmath_swizzle, glam_swizzle, ggmath_horizontal,
        glam_horizontal, ggmath_round, glam_round, ggmath_trigo, glam_trigo, ggmath_exp, glam_exp,
        ggmath_euclid_div, glam_euclid_div,
);

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_arithmetic(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u += vec2!(x);
        v2u -= v2u * vec2!(y);
        v2u = -v2u / vec2!(z);
        v2u = v2u.recip() + v2u.sqrt();

        v3u += vec3!(x);
        v3u -= v3u * vec3!(y);
        v3u = -v3u / vec3!(z);
        v3u = v3u.recip() + v3u.sqrt();

        v3a += vec3!(x);
        v3a -= v3a * vec3!(y);
        v3a = -v3a / vec3!(z);
        v3a = v3a.recip() + v3a.sqrt();

        v4a += vec4!(x);
        v4a -= v4a * vec4!(y);
        v4a = -v4a / vec4!(z);
        v4a = v4a.recip() + v4a.sqrt();
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_arithmetic(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u += Vec2::splat(x);
        v2u -= v2u * Vec2::splat(y);
        v2u = -v2u / Vec2::splat(z);
        v2u = v2u.recip() + v2u.sqrt();

        v3u += Vec3::splat(x);
        v3u -= v3u * Vec3::splat(y);
        v3u = -v3u / Vec3::splat(z);
        v3u = v3u.recip() + v3u.sqrt();

        v3a += Vec3A::splat(x);
        v3a -= v3a * Vec3A::splat(y);
        v3a = -v3a / Vec3A::splat(z);
        v3a = v3a.recip() + v3a.sqrt();

        v4a += Vec4::splat(x);
        v4a -= v4a * Vec4::splat(y);
        v4a = -v4a / Vec4::splat(z);
        v4a = v4a.recip() + v4a.sqrt();
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_arithmetic_ext(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.lerp(v2u.move_towards(vec2!(4.0, 5.0), v2u.y), v2u.x);
        v2u = v2u.midpoint(v2u.sqrt());

        v3u = v3u.lerp(v3u.move_towards(vec3!(4.0, 5.0, 6.0), v3u.y), v3u.x);
        v3u = v3u.midpoint(v3u.sqrt()) + v3u.cross(vec3!(4.0, 5.0, 6.0));

        v3a = v3a.lerp(v3a.move_towards(vec3!(4.0, 5.0, 6.0), v3a.y), v3a.x);
        v3a = v3a.midpoint(v3a.sqrt()) + v3a.cross(vec3!(4.0, 5.0, 6.0));

        v4a = v4a.lerp(v4a.move_towards(vec4!(4.0, 5.0, 6.0, 7.0), v4a.y), v4a.x);
        v4a = v4a.midpoint(v4a.sqrt());
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_arithmetic_ext(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.lerp(v2u.move_towards(vec2(4.0, 5.0), v2u.y), v2u.x);
        v2u = v2u.midpoint(v2u.sqrt());

        v3u = v3u.lerp(v3u.move_towards(vec3(4.0, 5.0, 6.0), v3u.y), v3u.x);
        v3u = v3u.midpoint(v3u.sqrt()) + v3u.cross(vec3(4.0, 5.0, 6.0));

        v3a = v3a.lerp(v3a.move_towards(vec3a(4.0, 5.0, 6.0), v3a.y), v3a.x);
        v3a = v3a.midpoint(v3a.sqrt()) + v3a.cross(vec3a(4.0, 5.0, 6.0));

        v4a = v4a.lerp(v4a.move_towards(vec4(4.0, 5.0, 6.0, 7.0), v4a.y), v4a.x);
        v4a = v4a.midpoint(v4a.sqrt());
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_bitops(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u / v2u.abs();
        v2u = v2u.recip().max(v2u + vec2!(1.0));
        v2u = v2u.min(v2u + vec2!(1.0));
        v2u = v2u.clamp(v2u - vec2!(1.0), v2u + vec2!(1.0)).signum();
        v2u = v2u.copysign(v2u + vec2!(1.0));

        v3u = v3u / v3u.abs();
        v3u = v3u.recip().max(v3u + vec3!(1.0));
        v3u = v3u.min(v3u + vec3!(1.0));
        v3u = v3u.clamp(v3u - vec3!(1.0), v3u + vec3!(1.0)).signum();
        v3u = v3u.copysign(v3u + vec3!(1.0));

        v3a = v3a / v3a.abs();
        v3a = v3a.recip().max(v3a + vec3!(1.0));
        v3a = v3a.min(v3a + vec3!(1.0));
        v3a = v3a.clamp(v3a - vec3!(1.0), v3a + vec3!(1.0)).signum();
        v3a = v3a.copysign(v3a + vec3!(1.0));

        v4a = v4a / v4a.abs();
        v4a = v4a.recip().max(v4a + vec4!(1.0));
        v4a = v4a.min(v4a + vec4!(1.0));
        v4a = v4a.clamp(v4a - vec4!(1.0), v4a + vec4!(1.0)).signum();
        v4a = v4a.copysign(v4a + vec4!(1.0));
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_bitops(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u / v2u.abs();
        v2u = v2u.recip().max(v2u + Vec2::ONE);
        v2u = v2u.min(v2u + Vec2::ONE);
        v2u = v2u.clamp(v2u - Vec2::ONE, v2u + Vec2::ONE).signum();
        v2u = v2u.copysign(v2u + Vec2::ONE);

        v3u = v3u / v3u.abs();
        v3u = v3u.recip().max(v3u + Vec3::ONE);
        v3u = v3u.min(v3u + Vec3::ONE);
        v3u = v3u.clamp(v3u - Vec3::ONE, v3u + Vec3::ONE).signum();
        v3u = v3u.copysign(v3u + Vec3::ONE);

        v3a = v3a / v3a.abs();
        v3a = v3a.recip().max(v3a + Vec3A::ONE);
        v3a = v3a.min(v3a + Vec3A::ONE);
        v3a = v3a.clamp(v3a - Vec3A::ONE, v3a + Vec3A::ONE).signum();
        v3a = v3a.copysign(v3a + Vec3A::ONE);

        v4a = v4a / v4a.abs();
        v4a = v4a.recip().max(v4a + Vec4::ONE);
        v4a = v4a.min(v4a + Vec4::ONE);
        v4a = v4a.clamp(v4a - Vec4::ONE, v4a + Vec4::ONE).signum();
        v4a = v4a.copysign(v4a + Vec4::ONE);
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_swizzle(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.yx() + v3u.zx() + v3a.zx().unalign() + v4a.wy().unalign();
        v3u = v2u.yyx() + v3u.zxy() + v3a.zxy().unalign() + v4a.wyz().unalign();
        v3a = v2u.align().yyx() + v3u.zxy().align() + v3a.zxy() + v4a.wyz();
        v4a = v2u.align().xyyx() + v3u.zxyy().align() + v3a.zxyx() + v4a.wyzz();
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_swizzle(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{
        Vec2, Vec2Swizzles, Vec3, Vec3A, Vec3Swizzles, Vec4, Vec4Swizzles, vec2, vec3, vec3a, vec4,
    };

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.yx() + v3u.zx() + v3a.zx() + v4a.wy();
        v3u = v2u.yyx() + v3u.zxy() + v3a.zxy().to_vec3() + v4a.wyz();
        v3a = v2u.yyx().to_vec3a() + v3u.zxy().to_vec3a() + v3a.zxy() + v4a.wyz().to_vec3a();
        v4a = v2u.xyyx() + v3u.zxyy() + v3a.zxyx() + v4a.wyzz();
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_horizontal(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.with_x((v2u + 1.0 == v2u - 1.0) as u8 as f32);
        v2u = v2u.with_y((v2u + 1.0 != v2u - 1.0) as u8 as f32);
        v2u = v2u.with_x((v2u + 1.0).is_nan() as u8 as f32);
        v2u = v2u.with_y((v2u + 1.0).is_finite() as u8 as f32);
        v2u = v2u.with_x(v2u.element_sum());
        v2u = v2u.with_y(v2u.element_product());
        v2u = v2u.with_x(v2u.max_element());
        v2u = v2u.with_y(v2u.min_element());
        v2u = v2u.normalize();
        v2u = v2u.with_x(v2u.dot(v2u + 1.0));
        v2u = v2u.try_normalize().unwrap_or(vec2!(0.0));
        v2u = v2u.with_y(v2u.length());
        v2u = v2u.normalize_or(vec2!(0.0));
        v2u = v2u.with_y(v2u.is_normalized() as u8 as f32);
        v2u = v2u.with_x(v2u.length_squared() + v2u.distance(v2u * 1.5));
        v2u = v2u.with_y(v2u.distance_squared(v2u * 1.5));
        v2u = v2u.normalize_or_zero()
            + v2u.with_max_length(10.0)
            + v2u.with_min_length(5.0)
            + v2u.clamp_length(3.0, 4.0);

        v3u = v3u.with_x((v3u + 1.0 == v3u - 1.0) as u8 as f32);
        v3u = v3u.with_y((v3u + 1.0 != v3u - 1.0) as u8 as f32);
        v3u = v3u.with_x((v3u + 1.0).is_nan() as u8 as f32);
        v3u = v3u.with_y((v3u + 1.0).is_finite() as u8 as f32);
        v3u = v3u.with_x(v3u.element_sum());
        v3u = v3u.with_y(v3u.element_product());
        v3u = v3u.with_x(v3u.max_element());
        v3u = v3u.with_y(v3u.min_element());
        v3u = v3u.normalize();
        v3u = v3u.with_x(v3u.dot(v3u + 1.0));
        v3u = v3u.try_normalize().unwrap_or(vec3!(0.0));
        v3u = v3u.with_y(v3u.length());
        v3u = v3u.normalize_or(vec3!(0.0));
        v3u = v3u.with_y(v3u.is_normalized() as u8 as f32);
        v3u = v3u.with_x(v3u.length_squared() + v3u.distance(v3u * 1.5));
        v3u = v3u.with_y(v3u.distance_squared(v3u * 1.5));
        v3u = v3u.normalize_or_zero()
            + v3u.with_max_length(10.0)
            + v3u.with_min_length(5.0)
            + v3u.clamp_length(3.0, 4.0);

        v3a = v3a.with_x((v3a + 1.0 == v3a - 1.0) as u8 as f32);
        v3a = v3a.with_y((v3a + 1.0 != v3a - 1.0) as u8 as f32);
        v3a = v3a.with_x((v3a + 1.0).is_nan() as u8 as f32);
        v3a = v3a.with_y((v3a + 1.0).is_finite() as u8 as f32);
        v3a = v3a.with_x(v3a.element_sum());
        v3a = v3a.with_y(v3a.element_product());
        v3a = v3a.with_x(v3a.max_element());
        v3a = v3a.with_y(v3a.min_element());
        v3a = v3a.normalize();
        v3a = v3a.with_x(v3a.dot(v3a + 1.0));
        v3a = v3a.try_normalize().unwrap_or(vec3!(0.0));
        v3a = v3a.with_y(v3a.length());
        v3a = v3a.normalize_or(vec3!(0.0));
        v3a = v3a.with_y(v3a.is_normalized() as u8 as f32);
        v3a = v3a.with_x(v3a.length_squared() + v3a.distance(v3a * 1.5));
        v3a = v3a.with_y(v3a.distance_squared(v3a * 1.5));
        v3a = v3a.normalize_or_zero()
            + v3a.with_max_length(10.0)
            + v3a.with_min_length(5.0)
            + v3a.clamp_length(3.0, 4.0);

        v4a = v4a.with_x((v4a + 1.0 == v4a - 1.0) as u8 as f32);
        v4a = v4a.with_y((v4a + 1.0 != v4a - 1.0) as u8 as f32);
        v4a = v4a.with_x((v4a + 1.0).is_nan() as u8 as f32);
        v4a = v4a.with_y((v4a + 1.0).is_finite() as u8 as f32);
        v4a = v4a.with_x(v4a.element_sum());
        v4a = v4a.with_y(v4a.element_product());
        v4a = v4a.with_x(v4a.max_element());
        v4a = v4a.with_y(v4a.min_element());
        v4a = v4a.normalize();
        v4a = v4a.with_x(v4a.dot(v4a + 1.0));
        v4a = v4a.try_normalize().unwrap_or(vec4!(0.0));
        v4a = v4a.with_y(v4a.length());
        v4a = v4a.normalize_or(vec4!(0.0));
        v4a = v4a.with_y(v4a.is_normalized() as u8 as f32);
        v4a = v4a.with_x(v4a.length_squared() + v4a.distance(v4a * 1.5));
        v4a = v4a.with_y(v4a.distance_squared(v4a * 1.5));
        v4a = v4a.normalize_or_zero()
            + v4a.with_max_length(10.0)
            + v4a.with_min_length(5.0)
            + v4a.clamp_length(3.0, 4.0);
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_horizontal(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.with_x((v2u + 1.0 == v2u - 1.0) as u8 as f32);
        v2u = v2u.with_y((v2u + 1.0 != v2u - 1.0) as u8 as f32);
        v2u = v2u.with_x((v2u + 1.0).is_nan() as u8 as f32);
        v2u = v2u.with_y((v2u + 1.0).is_finite() as u8 as f32);
        v2u = v2u.with_x(v2u.element_sum());
        v2u = v2u.with_y(v2u.element_product());
        v2u = v2u.with_x(v2u.max_element());
        v2u = v2u.with_y(v2u.min_element());
        v2u = v2u.normalize();
        v2u = v2u.with_x(v2u.dot(v2u + 1.0));
        v2u = v2u.try_normalize().unwrap_or(Vec2::ZERO);
        v2u = v2u.with_y(v2u.length());
        v2u = v2u.normalize_or(Vec2::ZERO);
        v2u = v2u.with_y(v2u.is_normalized() as u8 as f32);
        v2u = v2u.with_x(v2u.length_squared() + v2u.distance(v2u * 1.5));
        v2u = v2u.with_y(v2u.distance_squared(v2u * 1.5));
        v2u = v2u.normalize_or_zero()
            + v2u.clamp_length_max(10.0)
            + v2u.clamp_length_min(5.0)
            + v2u.clamp_length(3.0, 4.0);

        v3u = v3u.with_x((v3u + 1.0 == v3u - 1.0) as u8 as f32);
        v3u = v3u.with_y((v3u + 1.0 != v3u - 1.0) as u8 as f32);
        v3u = v3u.with_x((v3u + 1.0).is_nan() as u8 as f32);
        v3u = v3u.with_y((v3u + 1.0).is_finite() as u8 as f32);
        v3u = v3u.with_x(v3u.element_sum());
        v3u = v3u.with_y(v3u.element_product());
        v3u = v3u.with_x(v3u.max_element());
        v3u = v3u.with_y(v3u.min_element());
        v3u = v3u.normalize();
        v3u = v3u.with_x(v3u.dot(v3u + 1.0));
        v3u = v3u.try_normalize().unwrap_or(Vec3::ZERO);
        v3u = v3u.with_y(v3u.length());
        v3u = v3u.normalize_or(Vec3::ZERO);
        v3u = v3u.with_y(v3u.is_normalized() as u8 as f32);
        v3u = v3u.with_x(v3u.length_squared() + v3u.distance(v3u * 1.5));
        v3u = v3u.with_y(v3u.distance_squared(v3u * 1.5));
        v3u = v3u.normalize_or_zero()
            + v3u.clamp_length_max(10.0)
            + v3u.clamp_length_min(5.0)
            + v3u.clamp_length(3.0, 4.0);

        v3a = v3a.with_x((v3a + 1.0 == v3a - 1.0) as u8 as f32);
        v3a = v3a.with_y((v3a + 1.0 != v3a - 1.0) as u8 as f32);
        v3a = v3a.with_x((v3a + 1.0).is_nan() as u8 as f32);
        v3a = v3a.with_y((v3a + 1.0).is_finite() as u8 as f32);
        v3a = v3a.with_x(v3a.element_sum());
        v3a = v3a.with_y(v3a.element_product());
        v3a = v3a.with_x(v3a.max_element());
        v3a = v3a.with_y(v3a.min_element());
        v3a = v3a.normalize();
        v3a = v3a.with_x(v3a.dot(v3a + 1.0));
        v3a = v3a.try_normalize().unwrap_or(Vec3A::ZERO);
        v3a = v3a.with_y(v3a.length());
        v3a = v3a.normalize_or(Vec3A::ZERO);
        v3a = v3a.with_y(v3a.is_normalized() as u8 as f32);
        v3a = v3a.with_x(v3a.length_squared() + v3a.distance(v3a * 1.5));
        v3a = v3a.with_y(v3a.distance_squared(v3a * 1.5));
        v3a = v3a.normalize_or_zero()
            + v3a.clamp_length_max(10.0)
            + v3a.clamp_length_min(5.0)
            + v3a.clamp_length(3.0, 4.0);

        v4a = v4a.with_x((v4a + 1.0 == v4a - 1.0) as u8 as f32);
        v4a = v4a.with_y((v4a + 1.0 != v4a - 1.0) as u8 as f32);
        v4a = v4a.with_x((v4a + 1.0).is_nan() as u8 as f32);
        v4a = v4a.with_y((v4a + 1.0).is_finite() as u8 as f32);
        v4a = v4a.with_x(v4a.element_sum());
        v4a = v4a.with_y(v4a.element_product());
        v4a = v4a.with_x(v4a.max_element());
        v4a = v4a.with_y(v4a.min_element());
        v4a = v4a.normalize();
        v4a = v4a.with_x(v4a.dot(v4a + 1.0));
        v4a = v4a.try_normalize().unwrap_or(Vec4::ZERO);
        v4a = v4a.with_y(v4a.length());
        v4a = v4a.normalize_or(Vec4::ZERO);
        v4a = v4a.with_y(v4a.is_normalized() as u8 as f32);
        v4a = v4a.with_x(v4a.length_squared() + v4a.distance(v4a * 1.5));
        v4a = v4a.with_y(v4a.distance_squared(v4a * 1.5));
        v4a = v4a.normalize_or_zero()
            + v4a.clamp_length_max(10.0)
            + v4a.clamp_length_min(5.0)
            + v4a.clamp_length(3.0, 4.0);
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_round(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.floor() + v2u.ceil() - v2u.round() * v2u.trunc() / v2u.fract();
        v3u = v3u.floor() + v3u.ceil() - v3u.round() * v3u.trunc() / v3u.fract();
        v3a = v3a.floor() + v3a.ceil() - v3a.round() * v3a.trunc() / v3a.fract();
        v4a = v4a.floor() + v4a.ceil() - v4a.round() * v4a.trunc() / v4a.fract();
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_round(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.floor() + v2u.ceil() - v2u.round() * v2u.trunc() / v2u.fract();
        v3u = v3u.floor() + v3u.ceil() - v3u.round() * v3u.trunc() / v3u.fract();
        v3a = v3a.floor() + v3a.ceil() - v3a.round() * v3a.trunc() / v3a.fract();
        v4a = v4a.floor() + v4a.ceil() - v4a.round() * v4a.trunc() / v4a.fract();
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_trigo(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.sin() + v2u.cos() - v2u.tan() * v2u.asin() / v2u.acos() - v2u.atan();
        let (sin, cos) = v2u.sin_cos();
        v2u = sin / cos;
        v2u = v2u.rotate(5.0) + v2u.perp();

        v3u = v3u.sin() + v3u.cos() - v3u.tan() * v3u.asin() / v3u.acos() - v3u.atan();
        let (sin, cos) = v3u.sin_cos();
        v3u = sin / cos * sin.angle_between(cos);
        v3u = v3u.rotate_x(6.0) + v3u.rotate_y(7.0) + v3u.rotate_z(8.0);

        v3a = v3a.sin() + v3a.cos() - v3a.tan() * v3a.asin() / v3a.acos() - v3a.atan();
        let (sin, cos) = v3a.sin_cos();
        v3a = sin / cos * sin.angle_between(cos);
        v3a = v3a.rotate_x(6.0) + v3a.rotate_y(7.0) + v3a.rotate_z(8.0);

        v4a = v4a.sin() + v4a.cos() - v4a.tan() * v4a.asin() / v4a.acos() - v4a.atan();
        let (sin, cos) = v4a.sin_cos();
        v4a = sin / cos;
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_trigo(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.sin() + v2u.cos()
            - vec2(v2u.x.tan(), v2u.y.tan()) * vec2(v2u.x.asin(), v2u.y.asin())
                / vec2(v2u.x.acos(), v2u.y.acos())
            - vec2(v2u.x.atan(), v2u.y.atan());
        let (sin, cos) = v2u.sin_cos();
        v2u = sin / cos;
        v2u = Vec2::from_angle(5.0).rotate(v2u) + v2u.perp();

        v3u = v3u.sin() + v3u.cos()
            - vec3(v3u.x.tan(), v3u.y.tan(), v3u.z.tan())
                * vec3(v3u.x.asin(), v3u.y.asin(), v3u.z.asin())
                / vec3(v3u.x.acos(), v3u.y.acos(), v3u.z.acos())
            - vec3(v3u.x.atan(), v3u.y.atan(), v3u.z.atan());
        let (sin, cos) = v3u.sin_cos();
        v3u = sin / cos * sin.angle_between(cos);
        v3u = v3u.rotate_x(6.0) + v3u.rotate_y(7.0) + v3u.rotate_z(8.0);

        v3a = v3a.sin() + v3a.cos()
            - vec3a(v3a.x.tan(), v3a.y.tan(), v3a.z.tan())
                * vec3a(v3a.x.asin(), v3a.y.asin(), v3a.z.asin())
                / vec3a(v3a.x.acos(), v3a.y.acos(), v3a.z.acos())
            - vec3a(v3a.x.atan(), v3a.y.atan(), v3a.z.atan());
        let (sin, cos) = v3a.sin_cos();
        v3a = sin / cos * sin.angle_between(cos);
        v3a = v3a.rotate_x(6.0) + v3a.rotate_y(7.0) + v3a.rotate_z(8.0);

        v4a = v4a.sin() + v4a.cos()
            - vec4(v4a.x.tan(), v4a.y.tan(), v4a.z.tan(), v4a.w.tan())
                * vec4(v4a.x.asin(), v4a.y.asin(), v4a.z.asin(), v4a.w.asin())
                / vec4(v4a.x.acos(), v4a.y.acos(), v4a.z.acos(), v4a.w.acos())
            - vec4(v4a.x.atan(), v4a.y.atan(), v4a.z.atan(), v4a.w.atan());
        let (sin, cos) = v4a.sin_cos();
        v4a = sin / cos;
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_exp(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.exp() + v2u.exp2() + v2u.ln() + v2u.log2() + v2u.powf(5.0);
        v3u = v3u.exp() + v3u.exp2() + v3u.ln() + v3u.log2() + v3u.powf(5.0);
        v3a = v3a.exp() + v3a.exp2() + v3a.ln() + v3a.log2() + v3a.powf(5.0);
        v4a = v4a.exp() + v4a.exp2() + v4a.ln() + v4a.log2() + v4a.powf(5.0);
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_exp(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u.exp() + v2u.exp2() + v2u.ln() + v2u.log2() + v2u.powf(5.0);
        v3u = v3u.exp() + v3u.exp2() + v3u.ln() + v3u.log2() + v3u.powf(5.0);
        v3a = v3a.exp() + v3a.exp2() + v3a.ln() + v3a.log2() + v3a.powf(5.0);
        v4a = v4a.exp() + v4a.exp2() + v4a.ln() + v4a.log2() + v4a.powf(5.0);
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_euclid_div(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u % (v2u + 1.0);
        v2u = v2u.div_euclid(v2u + 1.0);
        v2u = v2u.rem_euclid(v2u + 1.0);

        v3u = v3u % (v3u + 1.0);
        v3u = v3u.div_euclid(v3u + 1.0);
        v3u = v3u.rem_euclid(v3u + 1.0);

        v3a = v3a % (v3a + 1.0);
        v3a = v3a.div_euclid(v3a + 1.0);
        v3a = v3a.rem_euclid(v3a + 1.0);

        v4a = v4a % (v4a + 1.0);
        v4a = v4a.div_euclid(v4a + 1.0);
        v4a = v4a.rem_euclid(v4a + 1.0);
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_euclid_div(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u = v2u % (v2u + 1.0);
        v2u = v2u.div_euclid(v2u + 1.0);
        v2u = v2u.rem_euclid(v2u + 1.0);

        v3u = v3u % (v3u + 1.0);
        v3u = v3u.div_euclid(v3u + 1.0);
        v3u = v3u.rem_euclid(v3u + 1.0);

        v3a = v3a % (v3a + 1.0);
        v3a = v3a.div_euclid(v3a + 1.0);
        v3a = v3a.rem_euclid(v3a + 1.0);

        v4a = v4a % (v4a + 1.0);
        v4a = v4a.div_euclid(v4a + 1.0);
        v4a = v4a.rem_euclid(v4a + 1.0);
    }

    (v2u, v3u, v3a, v4a)
}
