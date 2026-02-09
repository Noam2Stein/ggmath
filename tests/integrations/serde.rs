use std::error::Error;

use assert_impl_trait::assert_impl;
use ggmath::{
    Alignment, Length, Mask, Mask2, Mask2U, Mask3, Mask3U, Mask4, Mask4U, Mat2, Mat2U, Mat3, Mat3U,
    Mat4, Mat4U, Matrix, Quat, QuatU, Quaternion, Scalar, SupportedLength, Vec2, Vec2U, Vec3,
    Vec3U, Vec4, Vec4U, Vector, mat2, mat3, mat4, vec2, vec3, vec4,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        Mask<N, T, A>: Serialize,
        where T: Serialize {
            Vector<N, T, A>: Serialize,
            Matrix<N, T, A>: Serialize,
            Quaternion<T, A>: Serialize,
        }

        for<'de> {
            Mask<N, T, A>: Deserialize<'de>,
            where T: Deserialize<'de> {
                Vector<N, T, A>: Deserialize<'de>,
                Matrix<N, T, A>: Deserialize<'de>,
                Quaternion<T, A>: Deserialize<'de>,
            }
        }
    }
);

#[test]
fn vector() -> Result<(), Box<dyn Error>> {
    let val: Vec2<u32> = vec2!(1, 2);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Vec3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec3U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec4U<f32>>(&to_string(&val)?).is_err());

    let val: Vec3<u32> = vec3!(1, 2, 3);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Vec2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec4U<f32>>(&to_string(&val)?).is_err());

    let val: Vec4<u32> = vec4!(1, 2, 3, 4);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Vec2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec3U<f32>>(&to_string(&val)?).is_err());

    let val: Vec2U<u32> = vec2!(1, 2);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Vec3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec3U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec4U<f32>>(&to_string(&val)?).is_err());

    let val: Vec3U<u32> = vec3!(1, 2, 3);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Vec2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec4U<f32>>(&to_string(&val)?).is_err());

    let val: Vec4U<u32> = vec4!(1, 2, 3, 4);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Vec2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Vec3U<f32>>(&to_string(&val)?).is_err());

    let val: Mat2<u32> = mat2!(vec2!(1, 2), vec2!(3, 4));
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mat3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat3U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat4U<f32>>(&to_string(&val)?).is_err());

    let val: Mat3<u32> = mat3!(vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(7, 8, 9));
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mat2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat4U<f32>>(&to_string(&val)?).is_err());

    let val: Mat4<u32> = mat4!(
        vec4!(1, 2, 3, 4),
        vec4!(5, 6, 7, 8),
        vec4!(9, 10, 11, 12),
        vec4!(13, 14, 15, 16),
    );
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mat2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat3U<f32>>(&to_string(&val)?).is_err());

    let val: Mat2U<u32> = mat2!(vec2!(1, 2), vec2!(3, 4));
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mat3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat3U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat4U<f32>>(&to_string(&val)?).is_err());

    let val: Mat3U<u32> = mat3!(vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(7, 8, 9));
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mat2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat4U<f32>>(&to_string(&val)?).is_err());

    let val: Mat4U<u32> = mat4!(
        vec4!(1, 2, 3, 4),
        vec4!(5, 6, 7, 8),
        vec4!(9, 10, 11, 12),
        vec4!(13, 14, 15, 16),
    );
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mat2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mat3U<f32>>(&to_string(&val)?).is_err());

    let val: Quat<u32> = Quat::new(1, 2, 3, 4);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);

    let val: QuatU<u32> = QuatU::new(1, 2, 3, 4);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);

    let val: Mask2<u32> = Mask2::new(false, true);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mask3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask3U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask4U<f32>>(&to_string(&val)?).is_err());

    let val: Mask3<u32> = Mask3::new(false, true, false);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mask2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask4U<f32>>(&to_string(&val)?).is_err());

    let val: Mask4<u32> = Mask4::new(false, true, false, true);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.unalign(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mask2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask3U<f32>>(&to_string(&val)?).is_err());

    let val: Mask2U<u32> = Mask2U::new(false, true);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mask3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask3U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask4U<f32>>(&to_string(&val)?).is_err());

    let val: Mask3U<u32> = Mask3U::new(false, true, false);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mask2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask4<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask4U<f32>>(&to_string(&val)?).is_err());

    let val: Mask4U<u32> = Mask4U::new(false, true, false, true);
    assert_eq!(val, from_str(&to_string(&val)?)?);
    assert_eq!(val.align(), from_str(&to_string(&val)?)?);
    assert!(from_str::<Mask2<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask3<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask2U<f32>>(&to_string(&val)?).is_err());
    assert!(from_str::<Mask3U<f32>>(&to_string(&val)?).is_err());

    Ok(())
}
