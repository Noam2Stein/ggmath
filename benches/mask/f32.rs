use gungraun::{library_benchmark, library_benchmark_group, main};

main!(library_benchmark_groups = benches);
library_benchmark_group!(
    name = benches;
    benchmarks = ggmath_mask, glam_mask
);

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_mask(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Vec2U, Vec3, Vec3U, Vec4, vec2, vec3, vec4};

    let mut v2u: Vec2U<f32> = vec2!(x, y);
    let mut v3u: Vec3U<f32> = vec3!(x, y, z);
    let mut v3a: Vec3<f32> = vec3!(x, y, z);
    let mut v4a: Vec4<f32> = vec4!(x, y, z, x);

    for _ in 0..2 {
        v2u += ((v2u + Vec2U::ONE).finite_mask() & (v2u / vec2!(x)).nan_mask())
            .select(Vec2U::ONE, Vec2U::NEG_X);
        v2u += v2u.eq_mask(vec2!(x, y)).select(v2u, -v2u)
            + v2u.ne_mask(vec2!(x, y)).select(Vec2U::Y, Vec2U::X);
        v2u += (v2u.lt_mask(vec2!(x, y)) | (v2u * vec2!(1.3)).gt_mask(v2u)).select(v2u, -v2u);
        v2u += (v2u.le_mask(vec2!(x, y)) ^ (v2u * vec2!(1.3)).ge_mask(v2u)).select(v2u, -v2u);

        v3u += ((v3u + Vec3U::ONE).finite_mask() & (v3u / vec3!(x)).nan_mask())
            .select(Vec3U::ONE, Vec3U::NEG_X);
        v3u += v3u.eq_mask(vec3!(x, y, z)).select(v3u, -v3u)
            + v3u.ne_mask(vec3!(x, y, z)).select(Vec3U::Y, Vec3U::X);
        v3u += (v3u.lt_mask(vec3!(x, y, z)) | (v3u * vec3!(1.3)).gt_mask(v3u)).select(v3u, -v3u);
        v3u += (v3u.le_mask(vec3!(x, y, z)) ^ (v3u * vec3!(1.3)).ge_mask(v3u)).select(v3u, -v3u);

        v3a += ((v3a + Vec3::ONE).finite_mask() & (v3a / vec3!(x)).nan_mask())
            .select(Vec3::ONE, Vec3::NEG_X);
        v3a += v3a.eq_mask(vec3!(x, y, z)).select(v3a, -v3a)
            + v3a.ne_mask(vec3!(x, y, z)).select(Vec3::Y, Vec3::X);
        v3a += (v3a.lt_mask(vec3!(x, y, z)) | (v3a * vec3!(1.3)).gt_mask(v3a)).select(v3a, -v3a);
        v3a += (v3a.le_mask(vec3!(x, y, z)) ^ (v3a * vec3!(1.3)).ge_mask(v3a)).select(v3a, -v3a);

        v4a += ((v4a + Vec4::ONE).finite_mask() & (v4a / vec4!(x)).nan_mask())
            .select(Vec4::ONE, Vec4::NEG_X);
        v4a += v4a.eq_mask(vec4!(x, y, z, x)).select(v4a, -v4a)
            + v4a.ne_mask(vec4!(x, y, z, x)).select(Vec4::Y, Vec4::X);
        v4a += (v4a.lt_mask(vec4!(x, y, z, x)) | (v4a * vec4!(1.3)).gt_mask(v4a)).select(v4a, -v4a);
        v4a += (v4a.le_mask(vec4!(x, y, z, x)) ^ (v4a * vec4!(1.3)).ge_mask(v4a)).select(v4a, -v4a);
    }

    (v2u, v3u, v3a, v4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_mask(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Vec2, Vec3, Vec3A, Vec4, vec2, vec3, vec3a, vec4};

    let mut v2u: Vec2 = vec2(x, y);
    let mut v3u: Vec3 = vec3(x, y, z);
    let mut v3a: Vec3A = vec3a(x, y, z);
    let mut v4a: Vec4 = vec4(x, y, z, x);

    for _ in 0..2 {
        v2u += Vec2::select(
            (v2u + Vec2::ONE).is_finite_mask() & (v2u / Vec2::splat(x)).is_nan_mask(),
            Vec2::ONE,
            Vec2::NEG_X,
        );
        v2u += Vec2::select(v2u.cmpeq(vec2(x, y)), v2u, -v2u)
            + Vec2::select(v2u.cmpne(vec2(x, y)), Vec2::Y, Vec2::X);
        v2u += Vec2::select(
            v2u.cmplt(vec2(x, y)) | (v2u * Vec2::splat(1.3)).cmpgt(v2u),
            v2u,
            -v2u,
        );
        v2u += Vec2::select(
            v2u.cmple(vec2(x, y)) ^ (v2u * Vec2::splat(1.3)).cmpge(v2u),
            v2u,
            -v2u,
        );

        v3u += Vec3::select(
            (v3u + Vec3::ONE).is_finite_mask() & (v3u / Vec3::splat(x)).is_nan_mask(),
            Vec3::ONE,
            Vec3::NEG_X,
        );
        v3u += Vec3::select(v3u.cmpeq(vec3(x, y, z)), v3u, -v3u)
            + Vec3::select(v3u.cmpne(vec3(x, y, z)), Vec3::Y, Vec3::X);
        v3u += Vec3::select(
            v3u.cmplt(vec3(x, y, z)) | (v3u * Vec3::splat(1.3)).cmpgt(v3u),
            v3u,
            -v3u,
        );
        v3u += Vec3::select(
            v3u.cmple(vec3(x, y, z)) ^ (v3u * Vec3::splat(1.3)).cmpge(v3u),
            v3u,
            -v3u,
        );

        v3a += Vec3A::select(
            (v3a + Vec3A::ONE).is_finite_mask() & (v3a / Vec3A::splat(x)).is_nan_mask(),
            Vec3A::ONE,
            Vec3A::NEG_X,
        );
        v3a += Vec3A::select(v3a.cmpeq(vec3a(x, y, z)), v3a, -v3a)
            + Vec3A::select(v3a.cmpne(vec3a(x, y, z)), Vec3A::Y, Vec3A::X);
        v3a += Vec3A::select(
            v3a.cmplt(vec3a(x, y, z)) | (v3a * Vec3A::splat(1.3)).cmpgt(v3a),
            v3a,
            -v3a,
        );
        v3a += Vec3A::select(
            v3a.cmple(vec3a(x, y, z)) ^ (v3a * Vec3A::splat(1.3)).cmpge(v3a),
            v3a,
            -v3a,
        );

        v4a += Vec4::select(
            (v4a + Vec4::ONE).is_finite_mask() & (v4a / Vec4::splat(x)).is_nan_mask(),
            Vec4::ONE,
            Vec4::NEG_X,
        );
        v4a += Vec4::select(v4a.cmpeq(vec4(x, y, z, x)), v4a, -v4a)
            + Vec4::select(v4a.cmpne(vec4(x, y, z, x)), Vec4::Y, Vec4::X);
        v4a += Vec4::select(
            v4a.cmplt(vec4(x, y, z, x)) | (v4a * Vec4::splat(1.3)).cmpgt(v4a),
            v4a,
            -v4a,
        );
        v4a += Vec4::select(
            v4a.cmple(vec4(x, y, z, x)) ^ (v4a * Vec4::splat(1.3)).cmpge(v4a),
            v4a,
            -v4a,
        );
    }

    (v2u, v3u, v3a, v4a)
}
