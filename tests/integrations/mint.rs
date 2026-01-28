use assert_impl_trait::assert_impl;
use ggmath::{Alignment, Scalar, Vec2, Vec2U, Vec3, Vec3U, Vec4, Vec4U, Vector, vec2, vec3, vec4};
use mint::IntoMint;

assert_impl!(
    for<T, A: Alignment> where T: Scalar {
        Vector<2, T, A>: IntoMint<MintType = mint::Vector2<T>>,
        Vector<2, T, A>: From<mint::Point2<T>>,
        Vector<2, T, A>: Into<mint::Point2<T>>,
        Vector<2, T, A>: From<mint::Vector2<T>>,
        Vector<2, T, A>: Into<mint::Vector2<T>>,

        Vector<3, T, A>: IntoMint<MintType = mint::Vector3<T>>,
        Vector<3, T, A>: From<mint::Point3<T>>,
        Vector<3, T, A>: Into<mint::Point3<T>>,
        Vector<3, T, A>: From<mint::Vector3<T>>,
        Vector<3, T, A>: Into<mint::Vector3<T>>,

        Vector<4, T, A>: IntoMint<MintType = mint::Vector4<T>>,
        Vector<4, T, A>: From<mint::Vector4<T>>,
        Vector<4, T, A>: Into<mint::Vector4<T>>,
    }
);

#[test]
fn vector() {
    let val: Vec2<i32> = vec2!(1, 2);
    assert_eq!(val, mint::Point2::from(val).into());
    assert_eq!(val, mint::Vector2::from(val).into());

    let val: Vec3<i32> = vec3!(1, 2, 3);
    assert_eq!(val, mint::Point3::from(val).into());
    assert_eq!(val, mint::Vector3::from(val).into());

    let val: Vec4<i32> = vec4!(1, 2, 3, 4);
    assert_eq!(val, mint::Vector4::from(val).into());

    let val: Vec2U<i32> = vec2!(1, 2);
    assert_eq!(val, mint::Point2::from(val).into());
    assert_eq!(val, mint::Vector2::from(val).into());

    let val: Vec3U<i32> = vec3!(1, 2, 3);
    assert_eq!(val, mint::Point3::from(val).into());
    assert_eq!(val, mint::Vector3::from(val).into());

    let val: Vec4U<i32> = vec4!(1, 2, 3, 4);
    assert_eq!(val, mint::Vector4::from(val).into());
}
