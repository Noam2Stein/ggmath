use std::error::Error;

use ggmath::{Vec2, Vec2U, Vec3, Vec3U, Vec4, Vec4U, vec2, vec3, vec4};
use serde_json::{from_str, to_string};

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

    Ok(())
}
