use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};

main!(library_benchmark_groups = benches);
library_benchmark_group!(
    name = benches;
    benchmarks = ggmath_mat2a, glam_mat2a, ggmath_mat3u, glam_mat3u, ggmath_mat3a, glam_mat3a,
        ggmath_mat4a, glam_mat4a,
);

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_mat2a(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Mat2, mat2, vec2};

    let mut m2a: Mat2<f32> = mat2!(vec2!(x, y), vec2!(z, x));

    for _ in 0..2 {
        m2a += Mat2::from_diagonal(vec2!(x + 1.0, y));
        m2a += Mat2::from_col_array(&[m2a.to_col_array()[0]; 2]);
        m2a -= Mat2::from_diagonal(m2a.col(1) + m2a.row(1));
        m2a.set_row(1, vec2!(x + m2a.y_axis.x, m2a.x_axis.y));
        m2a += mat2!(m2a.diagonal(), m2a.diagonal() * vec2!(1.5));
    }

    black_box(m2a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_mat2a(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Mat2, Vec2, mat2, vec2};

    let mut m2a: Mat2 = mat2(vec2(x, y), vec2(z, x));

    for _ in 0..2 {
        m2a += Mat2::from_diagonal(vec2(x + 1.0, y));
        m2a += mat2(m2a.x_axis, m2a.x_axis);
        m2a -= Mat2::from_diagonal(m2a.col(1) + m2a.row(1));
        let row = vec2(x + m2a.y_axis.x, m2a.x_axis.y);
        m2a.x_axis.y = row.x;
        m2a.y_axis.y = row.y;
        m2a += mat2(m2a.diagonal(), m2a.diagonal() * Vec2::splat(1.5));
    }

    black_box(m2a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_mat3u(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Mat3U, mat3, vec3};

    let mut m3u: Mat3U<f32> = mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y));

    for _ in 0..2 {
        m3u += Mat3U::from_diagonal(vec3!(x + 1.0, y, z + 2.0));
        m3u += Mat3U::from_col_array(&[m3u.to_col_array()[0]; 3]);
        m3u -= Mat3U::from_diagonal(m3u.col(1) + m3u.row(1));
        m3u.set_row(1, vec3!(x + m3u.y_axis.x, m3u.x_axis.y, m3u.z_axis.y));
        m3u += mat3!(
            m3u.diagonal(),
            m3u.diagonal() * vec3!(1.5),
            m3u.diagonal() + vec3!(1.3),
        );
    }

    black_box(m3u)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_mat3u(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Mat3, Vec3, mat3, vec3};

    let mut m3u: Mat3 = mat3(vec3(x, y, z), vec3(y, z, x), vec3(z, x, y));

    for _ in 0..2 {
        m3u += Mat3::from_diagonal(vec3(x + 1.0, y, z + 2.0));
        m3u += mat3(m3u.x_axis, m3u.x_axis, m3u.x_axis);
        m3u -= Mat3::from_diagonal(m3u.col(1) + m3u.row(1));
        let row = vec3(x + m3u.y_axis.x, m3u.x_axis.y, m3u.z_axis.y);
        m3u.x_axis.y = row.x;
        m3u.y_axis.y = row.y;
        m3u.z_axis.y = row.z;
        m3u += mat3(
            m3u.diagonal(),
            m3u.diagonal() * Vec3::splat(1.5),
            m3u.diagonal() + Vec3::splat(1.3),
        );
    }

    black_box(m3u)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_mat3a(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Mat3, mat3, vec3};

    let mut m3a: Mat3<f32> = mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y));

    for _ in 0..2 {
        m3a += Mat3::from_diagonal(vec3!(x + 1.0, y, z + 2.0));
        m3a += Mat3::from_col_array(&[m3a.to_col_array()[0]; 3]);
        m3a -= Mat3::from_diagonal(m3a.col(1) + m3a.row(1));
        m3a.set_row(1, vec3!(x + m3a.y_axis.x, m3a.x_axis.y, m3a.z_axis.y));
        m3a += mat3!(
            m3a.diagonal(),
            m3a.diagonal() * vec3!(1.5),
            m3a.diagonal() + vec3!(1.3),
        );
    }

    black_box(m3a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_mat3a(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Mat3A, Vec3A, mat3a, vec3, vec3a};

    let mut m3a: Mat3A = mat3a(vec3a(x, y, z), vec3a(y, z, x), vec3a(z, x, y));

    for _ in 0..2 {
        m3a += Mat3A::from_diagonal(vec3(x + 1.0, y, z + 2.0));
        m3a += mat3a(m3a.x_axis, m3a.x_axis, m3a.x_axis);
        m3a -= Mat3A::from_diagonal((m3a.col(1) + m3a.row(1)).to_vec3());
        let row = vec3a(x + m3a.y_axis.x, m3a.x_axis.y, m3a.z_axis.y);
        m3a.x_axis.y = row.x;
        m3a.y_axis.y = row.y;
        m3a.z_axis.y = row.z;
        m3a += mat3a(
            m3a.diagonal(),
            m3a.diagonal() * Vec3A::splat(1.5),
            m3a.diagonal() + Vec3A::splat(1.3),
        );
    }

    black_box(m3a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn ggmath_mat4a(x: f32, y: f32, z: f32) -> impl Copy {
    use ggmath::{Mat4, mat4, vec4};

    let mut m4a: Mat4<f32> = mat4!(
        vec4!(x, y, z, x),
        vec4!(y, z, x, y),
        vec4!(z, x, y, z),
        vec4!(z, y, x, z),
    );

    for _ in 0..2 {
        m4a += Mat4::from_diagonal(vec4!(x + 1.0, y, z + 2.0, x));
        m4a += Mat4::from_col_array(&[m4a.to_col_array()[0]; 4]);
        m4a -= Mat4::from_diagonal(m4a.col(1) + m4a.row(1));
        m4a.set_row(
            1,
            vec4!(x + m4a.y_axis.x, m4a.x_axis.y, m4a.z_axis.y, m4a.w_axis.x),
        );
        m4a += mat4!(
            m4a.diagonal(),
            m4a.diagonal() * vec4!(1.5),
            m4a.diagonal() + vec4!(1.3),
            m4a.diagonal() + vec4!(5.0),
        );
    }

    black_box(m4a)
}

#[library_benchmark]
#[bench::arbitrary(4.0, 5.0, 6.0)]
pub fn glam_mat4a(x: f32, y: f32, z: f32) -> impl Copy {
    use glam::{Mat4, Vec4, mat4, vec4};

    let mut m4a: Mat4 = mat4(
        vec4(x, y, z, x),
        vec4(y, z, x, y),
        vec4(z, x, y, z),
        vec4(z, y, x, z),
    );

    for _ in 0..2 {
        m4a += Mat4::from_diagonal(vec4(x + 1.0, y, z + 2.0, x));
        m4a += mat4(m4a.x_axis, m4a.x_axis, m4a.x_axis, m4a.x_axis);
        m4a -= Mat4::from_diagonal(m4a.col(1) + m4a.row(1));
        let row = vec4(x + m4a.y_axis.x, m4a.x_axis.y, m4a.z_axis.y, m4a.w_axis.x);
        m4a.x_axis.y = row.x;
        m4a.y_axis.y = row.y;
        m4a.z_axis.y = row.z;
        m4a.w_axis.y = row.w;
        m4a += mat4(
            m4a.diagonal(),
            m4a.diagonal() * Vec4::splat(1.5),
            m4a.diagonal() + Vec4::splat(1.3),
            m4a.diagonal() + Vec4::splat(5.0),
        );
    }

    black_box(m4a)
}
