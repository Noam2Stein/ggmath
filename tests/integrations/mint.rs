use assert_impl_trait::assert_impl;
use ggmath::{
    Alignment, Mask, Mask2, Mask2U, Mask3, Mask3U, Mask4, Mask4U, Mat2, Mat2U, Mat3, Mat3U, Mat4,
    Mat4U, Matrix, Quat, QuatU, Quaternion, Scalar, Vec2, Vec2U, Vec3, Vec3U, Vec4, Vec4U, Vector,
    mat2, mat3, mat4, vec2, vec3, vec4,
};
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

        Matrix<2, T, A>: IntoMint<MintType = mint::ColumnMatrix2<T>>,
        Matrix<2, T, A>: From<mint::ColumnMatrix2<T>>,
        Matrix<2, T, A>: Into<mint::ColumnMatrix2<T>>,
        Matrix<2, T, A>: From<mint::RowMatrix2<T>>,
        Matrix<2, T, A>: Into<mint::RowMatrix2<T>>,

        Matrix<3, T, A>: IntoMint<MintType = mint::ColumnMatrix3<T>>,
        Matrix<3, T, A>: From<mint::ColumnMatrix3<T>>,
        Matrix<3, T, A>: Into<mint::ColumnMatrix3<T>>,
        Matrix<3, T, A>: From<mint::RowMatrix3<T>>,
        Matrix<3, T, A>: Into<mint::RowMatrix3<T>>,

        Matrix<4, T, A>: IntoMint<MintType = mint::ColumnMatrix4<T>>,
        Matrix<4, T, A>: From<mint::ColumnMatrix4<T>>,
        Matrix<4, T, A>: Into<mint::ColumnMatrix4<T>>,
        Matrix<4, T, A>: From<mint::RowMatrix4<T>>,
        Matrix<4, T, A>: Into<mint::RowMatrix4<T>>,

        Quaternion<T, A>: IntoMint<MintType = mint::Quaternion<T>>,
        Quaternion<T, A>: From<mint::Quaternion<T>>,
        Quaternion<T, A>: Into<mint::Quaternion<T>>,

        Mask<2, T, A>: IntoMint<MintType = mint::Vector2<bool>>,
        Mask<2, T, A>: From<mint::Vector2<bool>>,
        Mask<2, T, A>: Into<mint::Vector2<bool>>,

        Mask<3, T, A>: IntoMint<MintType = mint::Vector3<bool>>,
        Mask<3, T, A>: From<mint::Vector3<bool>>,
        Mask<3, T, A>: Into<mint::Vector3<bool>>,

        Mask<4, T, A>: IntoMint<MintType = mint::Vector4<bool>>,
        Mask<4, T, A>: From<mint::Vector4<bool>>,
        Mask<4, T, A>: Into<mint::Vector4<bool>>,
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

    let val: Mat2<i32> = mat2!(vec2!(1, 2), vec2!(3, 4));
    assert_eq!(val, mint::ColumnMatrix2::from(val).into());
    assert_eq!(val, mint::RowMatrix2::from(val).into());

    let val: Mat3<i32> = mat3!(vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(7, 8, 9));
    assert_eq!(val, mint::ColumnMatrix3::from(val).into());
    assert_eq!(val, mint::RowMatrix3::from(val).into());

    let val: Mat4<i32> = mat4!(
        vec4!(1, 2, 3, 4),
        vec4!(5, 6, 7, 8),
        vec4!(9, 10, 11, 12),
        vec4!(13, 14, 15, 16),
    );
    assert_eq!(val, mint::ColumnMatrix4::from(val).into());
    assert_eq!(val, mint::RowMatrix4::from(val).into());

    let val: Mat2U<i32> = mat2!(vec2!(1, 2), vec2!(3, 4));
    assert_eq!(val, mint::ColumnMatrix2::from(val).into());
    assert_eq!(val, mint::RowMatrix2::from(val).into());

    let val: Mat3U<i32> = mat3!(vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(7, 8, 9));
    assert_eq!(val, mint::ColumnMatrix3::from(val).into());
    assert_eq!(val, mint::RowMatrix3::from(val).into());

    let val: Mat4U<i32> = mat4!(
        vec4!(1, 2, 3, 4),
        vec4!(5, 6, 7, 8),
        vec4!(9, 10, 11, 12),
        vec4!(13, 14, 15, 16),
    );
    assert_eq!(val, mint::ColumnMatrix4::from(val).into());
    assert_eq!(val, mint::RowMatrix4::from(val).into());

    let val: Quat<i32> = Quat::new(1, 2, 3, 4);
    assert_eq!(val, mint::Quaternion::from(val).into());

    let val: QuatU<i32> = QuatU::new(1, 2, 3, 4);
    assert_eq!(val, mint::Quaternion::from(val).into());

    let val: Mask2<i32> = Mask2::new(false, true);
    assert_eq!(val, mint::Vector2::<bool>::from(val).into());

    let val: Mask3<i32> = Mask3::new(false, true, false);
    assert_eq!(val, mint::Vector3::<bool>::from(val).into());

    let val: Mask4<i32> = Mask4::new(false, true, false, true);
    assert_eq!(val, mint::Vector4::<bool>::from(val).into());

    let val: Mask2U<i32> = Mask2U::new(false, true);
    assert_eq!(val, mint::Vector2::<bool>::from(val).into());

    let val: Mask3U<i32> = Mask3U::new(false, true, false);
    assert_eq!(val, mint::Vector3::<bool>::from(val).into());

    let val: Mask4U<i32> = Mask4U::new(false, true, false, true);
    assert_eq!(val, mint::Vector4::<bool>::from(val).into());
}
