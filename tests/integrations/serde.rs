use std::error::Error;

use assert_impl_trait::assert_impl;
use ggmath::{
    Alignment, Length, Mask, Mask2, Mask2U, Mask3, Mask3U, Mask4, Mask4U, Scalar, SupportedLength,
    Vec2, Vec2U, Vec3, Vec3U, Vec4, Vec4U, Vector, vec2, vec3, vec4,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        where T: Serialize {
            Vector<N, T, A>: Serialize,
        }
        for<'de> where T: Deserialize<'de> {
            Vector<N, T, A>: Deserialize<'de>,
        }

        Mask<N, T, A>: Serialize,
        for<'de> {
            Mask<N, T, A>: Deserialize<'de>,
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
