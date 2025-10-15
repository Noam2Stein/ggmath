use core::{
    fmt::{Debug, Display},
    panic::{RefUnwindSafe, UnwindSafe},
};

use ggmath::{
    NonSimd, Scalar, Simd, Simdness, vec2, vec2g, vec2s, vec3, vec3g, vec3s, vec4, vec4g, vec4s,
};

use crate::assert_panic;

/// In order for these tests to be correct, `b` must be greater than `a`, and `d` must be greater than `c`.
fn test_primitive_api<T: Scalar, S: Simdness>(a: T, b: T, c: T, d: T)
where
    T: Debug + Display + PartialEq + PartialOrd + UnwindSafe + RefUnwindSafe,
{
    macro_rules! Vec2T {
        () => {
            ggmath::Vector<2, T, S>
        };
    }
    macro_rules! Vec3T {
        () => {
            ggmath::Vector<3, T, S>
        };
    }
    macro_rules! Vec4T {
        () => {
            ggmath::Vector<4, T, S>
        };
    }

    macro_rules! vec2t {
        ($($tt:tt)*) => {{
            let v: ggmath::Vector<_, _, S> = vec2g!($($tt)*);
            v
        }};
    }
    macro_rules! vec3t {
        ($($tt:tt)*) => {{
            let v: ggmath::Vector<_, _, S> = vec3g!($($tt)*);
            v
        }};
    }
    macro_rules! vec4t {
        ($($tt:tt)*) => {{
            let v: ggmath::Vector<_, _, S> = vec4g!($($tt)*);
            v
        }};
    }

    assert_eq!(<Vec2T!()>::from_array([a, b]).as_array(), [a, b]);
    assert_eq!(<Vec3T!()>::from_array([a, b, c]).as_array(), [a, b, c]);
    assert_eq!(
        <Vec4T!()>::from_array([a, b, c, d]).as_array(),
        [a, b, c, d]
    );

    assert_eq!(<Vec2T!()>::splat(a).as_array(), [a, a]);
    assert_eq!(<Vec3T!()>::splat(a).as_array(), [a, a, a]);
    assert_eq!(<Vec4T!()>::splat(a).as_array(), [a, a, a, a]);

    assert_eq!(<Vec2T!()>::from_fn(|i| [a, b][i]).as_array(), [a, b]);
    assert_eq!(<Vec3T!()>::from_fn(|i| [a, b, c][i]).as_array(), [a, b, c]);
    assert_eq!(
        <Vec4T!()>::from_fn(|i| [a, b, c, d][i]).as_array(),
        [a, b, c, d]
    );

    assert_eq!(<Vec2T!()>::const_from_array([a, b]).as_array(), [a, b]);
    assert_eq!(
        <Vec3T!()>::const_from_array([a, b, c]).as_array(),
        [a, b, c]
    );
    assert_eq!(
        <Vec4T!()>::const_from_array([a, b, c, d]).as_array(),
        [a, b, c, d]
    );

    assert_eq!(vec2t!(a, b).len(), 2);
    assert_eq!(vec3t!(a, b, c).len(), 3);
    assert_eq!(vec4t!(a, b, c, d).len(), 4);

    assert_eq!(vec2t!(a, b).is_simd(), S::IS_SIMD);
    assert_eq!(vec3t!(a, b, c).is_simd(), S::IS_SIMD);
    assert_eq!(vec4t!(a, b, c, d).is_simd(), S::IS_SIMD);

    assert_eq!(vec2t!(a, b).as_array(), [a, b]);
    assert_eq!(vec2t!(vec2t!(a, b)).as_array(), [a, b]);
    assert_eq!(vec3t!(a, b, c).as_array(), [a, b, c]);
    assert_eq!(vec3t!(a, vec2t!(b, c)).as_array(), [a, b, c]);
    assert_eq!(vec4t!(a, b, c, d).as_array(), [a, b, c, d]);
    assert_eq!(vec4t!(a, vec2t!(b, c), d).as_array(), [a, b, c, d]);

    assert_eq!(vec2t!(a).as_array(), [a; 2]);
    assert_eq!(vec3t!(a).as_array(), [a; 3]);
    assert_eq!(vec4t!(a).as_array(), [a; 4]);

    assert_eq!(vec2t!(a, b).as_array_ref(), &[a, b]);
    assert_eq!(vec3t!(a, b, c).as_array_ref(), &[a, b, c]);
    assert_eq!(vec4t!(a, b, c, d).as_array_ref(), &[a, b, c, d]);

    assert_eq!(vec2t!(a, b).as_mut_array(), &mut [a, b]);
    assert_eq!(vec3t!(a, b, c).as_mut_array(), &mut [a, b, c]);
    assert_eq!(vec4t!(a, b, c, d).as_mut_array(), &mut [a, b, c, d]);

    assert_eq!(vec2t!(a, b).as_simd(), vec2!(a, b));
    assert_eq!(vec3t!(a, b, c).as_simd(), vec3!(a, b, c));
    assert_eq!(vec4t!(a, b, c, d).as_simd(), vec4!(a, b, c, d));

    assert_eq!(vec2t!(a, b).as_nonsimd(), vec2s!(a, b));
    assert_eq!(vec3t!(a, b, c).as_nonsimd(), vec3s!(a, b, c));
    assert_eq!(vec4t!(a, b, c, d).as_nonsimd(), vec4s!(a, b, c, d));

    assert_eq!(vec2t!(a, b).get(0), Some(a));
    assert_eq!(vec2t!(a, b).get(1), Some(b));
    assert_eq!(vec2t!(a, b).get(2), None);
    assert_eq!(vec2t!(a, b).get(3), None);

    assert_eq!(vec3t!(a, b, c).get(0), Some(a));
    assert_eq!(vec3t!(a, b, c).get(1), Some(b));
    assert_eq!(vec3t!(a, b, c).get(2), Some(c));
    assert_eq!(vec3t!(a, b, c).get(3), None);
    assert_eq!(vec3t!(a, b, c).get(4), None);

    assert_eq!(vec4t!(a, b, c, d).get(0), Some(a));
    assert_eq!(vec4t!(a, b, c, d).get(1), Some(b));
    assert_eq!(vec4t!(a, b, c, d).get(2), Some(c));
    assert_eq!(vec4t!(a, b, c, d).get(3), Some(d));
    assert_eq!(vec4t!(a, b, c, d).get(4), None);
    assert_eq!(vec4t!(a, b, c, d).get(5), None);

    assert_eq!(vec2t!(a, b).get_mut(0), Some(&mut a.clone()));
    assert_eq!(vec2t!(a, b).get_mut(1), Some(&mut b.clone()));
    assert_eq!(vec2t!(a, b).get_mut(2), None);
    assert_eq!(vec2t!(a, b).get_mut(3), None);

    assert_eq!(vec3t!(a, b, c).get_mut(0), Some(&mut a.clone()));
    assert_eq!(vec3t!(a, b, c).get_mut(1), Some(&mut b.clone()));
    assert_eq!(vec3t!(a, b, c).get_mut(2), Some(&mut c.clone()));
    assert_eq!(vec3t!(a, b, c).get_mut(3), None);
    assert_eq!(vec3t!(a, b, c).get_mut(4), None);

    assert_eq!(vec4t!(a, b, c, d).get_mut(0), Some(&mut a.clone()));
    assert_eq!(vec4t!(a, b, c, d).get_mut(1), Some(&mut b.clone()));
    assert_eq!(vec4t!(a, b, c, d).get_mut(2), Some(&mut c.clone()));
    assert_eq!(vec4t!(a, b, c, d).get_mut(3), Some(&mut d.clone()));
    assert_eq!(vec4t!(a, b, c, d).get_mut(4), None);
    assert_eq!(vec4t!(a, b, c, d).get_mut(5), None);

    assert_eq!(vec2t!(a, b).iter().collect::<Vec<T>>(), vec![a, b]);
    assert_eq!(vec3t!(a, b, c).iter().collect::<Vec<T>>(), vec![a, b, c]);
    assert_eq!(
        vec4t!(a, b, c, d).iter().collect::<Vec<T>>(),
        vec![a, b, c, d]
    );

    assert_eq!(
        vec2t!(a, b).iter_mut().collect::<Vec<&mut T>>(),
        vec![&mut a.clone(), &mut b.clone()]
    );
    assert_eq!(
        vec3t!(a, b, c).iter_mut().collect::<Vec<&mut T>>(),
        vec![&mut a.clone(), &mut b.clone(), &mut c.clone()]
    );
    assert_eq!(
        vec4t!(a, b, c, d).iter_mut().collect::<Vec<&mut T>>(),
        vec![
            &mut a.clone(),
            &mut b.clone(),
            &mut c.clone(),
            &mut d.clone()
        ]
    );

    assert_eq!(vec2t!(a, b).map(|x| x == b), vec2t!(a == b, b == b));
    assert_eq!(
        vec3t!(a, b, c).map(|x| x == b),
        vec3t!(a == b, b == b, c == b)
    );
    assert_eq!(
        vec4t!(a, b, c, d).map(|x| x == b),
        vec4t!(a == b, b == b, c == b, d == b)
    );

    assert_eq!(vec2t!(a, b).reverse(), vec2t!(b, a));
    assert_eq!(vec3t!(a, b, c).reverse(), vec3t!(c, b, a));
    assert_eq!(vec4t!(a, b, c, d).reverse(), vec4t!(d, c, b, a));

    assert_eq!(vec2t!(a, b).swizzle2::<0, 1>(), vec2t!(a, b));
    assert_eq!(vec2t!(a, b).swizzle2::<1, 0>(), vec2t!(b, a));
    assert_eq!(vec2t!(a, b).swizzle2::<1, 1>(), vec2t!(b, b));

    assert_eq!(vec3t!(a, b, c).swizzle2::<0, 1>(), vec2t!(a, b));
    assert_eq!(vec3t!(a, b, c).swizzle2::<0, 2>(), vec2t!(a, c));
    assert_eq!(vec3t!(a, b, c).swizzle2::<2, 1>(), vec2t!(c, b));

    assert_eq!(vec4t!(a, b, c, d).swizzle2::<0, 1>(), vec2t!(a, b));
    assert_eq!(vec4t!(a, b, c, d).swizzle2::<1, 3>(), vec2t!(b, d));
    assert_eq!(vec4t!(a, b, c, d).swizzle2::<3, 1>(), vec2t!(d, b));

    assert_eq!(vec2t!(a, b).swizzle3::<0, 1, 1>(), vec3t!(a, b, b));
    assert_eq!(vec2t!(a, b).swizzle3::<1, 0, 1>(), vec3t!(b, a, b));

    assert_eq!(vec3t!(a, b, c).swizzle3::<0, 1, 2>(), vec3t!(a, b, c));
    assert_eq!(vec3t!(a, b, c).swizzle3::<1, 0, 2>(), vec3t!(b, a, c));
    assert_eq!(vec3t!(a, b, c).swizzle3::<2, 1, 0>(), vec3t!(c, b, a));

    assert_eq!(vec4t!(a, b, c, d).swizzle3::<0, 1, 2>(), vec3t!(a, b, c));
    assert_eq!(vec4t!(a, b, c, d).swizzle3::<1, 0, 2>(), vec3t!(b, a, c));
    assert_eq!(vec4t!(a, b, c, d).swizzle3::<2, 1, 0>(), vec3t!(c, b, a));

    assert_eq!(vec2t!(a, b).swizzle4::<0, 1, 1, 0>(), vec4t!(a, b, b, a));
    assert_eq!(vec2t!(a, b).swizzle4::<1, 0, 1, 0>(), vec4t!(b, a, b, a));
    assert_eq!(vec2t!(a, b).swizzle4::<1, 1, 0, 0>(), vec4t!(b, b, a, a));

    assert_eq!(vec3t!(a, b, c).swizzle4::<0, 1, 2, 0>(), vec4t!(a, b, c, a));
    assert_eq!(vec3t!(a, b, c).swizzle4::<1, 0, 2, 0>(), vec4t!(b, a, c, a));
    assert_eq!(vec3t!(a, b, c).swizzle4::<2, 1, 0, 0>(), vec4t!(c, b, a, a));

    assert_eq!(
        vec4t!(a, b, c, d).swizzle4::<0, 1, 2, 3>(),
        vec4t!(a, b, c, d)
    );
    assert_eq!(
        vec4t!(a, b, c, d).swizzle4::<1, 0, 2, 3>(),
        vec4t!(b, a, c, d)
    );
    assert_eq!(
        vec4t!(a, b, c, d).swizzle4::<2, 1, 0, 3>(),
        vec4t!(c, b, a, d)
    );
    assert_eq!(
        vec4t!(a, b, c, d).swizzle4::<3, 1, 2, 0>(),
        vec4t!(d, b, c, a)
    );

    assert_eq!(vec2t!(a, b)[0], a);
    assert_eq!(vec2t!(a, b)[1], b);

    assert_eq!(vec3t!(a, b, c)[0], a);
    assert_eq!(vec3t!(a, b, c)[1], b);
    assert_eq!(vec3t!(a, b, c)[2], c);

    assert_eq!(vec4t!(a, b, c, d)[0], a);
    assert_eq!(vec4t!(a, b, c, d)[1], b);
    assert_eq!(vec4t!(a, b, c, d)[2], c);
    assert_eq!(vec4t!(a, b, c, d)[3], d);

    assert_panic!(vec2t!(a, b)[2], "({a}, {b})[2]");
    assert_panic!(vec3t!(a, b, c)[3], "({a}, {b}, {c})[3]");
    assert_panic!(vec4t!(a, b, c, d)[4], "({a}, {b}, {c}, {d})[4]");

    assert_eq!(&mut vec2t!(a, b)[0], &mut a.clone());
    assert_eq!(&mut vec2t!(a, b)[1], &mut b.clone());

    assert_eq!(&mut vec3t!(a, b, c)[0], &mut a.clone());
    assert_eq!(&mut vec3t!(a, b, c)[1], &mut b.clone());
    assert_eq!(&mut vec3t!(a, b, c)[2], &mut c.clone());

    assert_eq!(&mut vec4t!(a, b, c, d)[0], &mut a.clone());
    assert_eq!(&mut vec4t!(a, b, c, d)[1], &mut b.clone());
    assert_eq!(&mut vec4t!(a, b, c, d)[2], &mut c.clone());
    assert_eq!(&mut vec4t!(a, b, c, d)[3], &mut d.clone());

    assert_panic!(&mut vec2t!(a, b)[2], "(&mut {a}, {b})[2]");
    assert_panic!(&mut vec3t!(a, b, c)[3], "(&mut {a}, {b}, {c})[3]");
    assert_panic!(&mut vec4t!(a, b, c, d)[4], "(&mut {a}, {b}, {c}, {d})[4]");

    assert_eq!(vec2t!(a, b) == vec2t!(a, b), true);
    assert_eq!(vec2t!(a, b) == vec2t!(b, a), false);
    assert_eq!(vec2t!(a, b) == vec2t!(b, b), false);

    assert_eq!(vec3t!(a, b, c) == vec3t!(a, b, c), true);
    assert_eq!(vec3t!(a, b, c) == vec3t!(d, c, b), false);
    assert_eq!(vec3t!(a, b, c) == vec3t!(b, b, c), false);

    assert_eq!(vec4t!(a, b, c, d) == vec4t!(a, b, c, d), true);
    assert_eq!(vec4t!(a, b, c, d) == vec4t!(d, c, b, a), false);
    assert_eq!(vec4t!(a, b, c, d) == vec4t!(b, b, b, d), false);

    assert_eq!(vec2t!(a, b) != vec2t!(a, b), false);
    assert_eq!(vec2t!(a, b) != vec2t!(b, a), true);
    assert_eq!(vec2t!(a, b) != vec2t!(b, b), true);

    assert_eq!(vec3t!(a, b, c) != vec3t!(a, b, c), false);
    assert_eq!(vec3t!(a, b, c) != vec3t!(d, c, b), true);
    assert_eq!(vec3t!(a, b, c) != vec3t!(b, b, c), true);

    assert_eq!(vec4t!(a, b, c, d) != vec4t!(a, b, c, d), false);
    assert_eq!(vec4t!(a, b, c, d) != vec4t!(d, c, b, a), true);
    assert_eq!(vec4t!(a, b, c, d) != vec4t!(b, b, b, d), true);

    assert_eq!(vec2t!(a, b).to_string(), format!("({a}, {b})"));
    assert_eq!(vec3t!(a, b, c).to_string(), format!("({a}, {b}, {c})"));
    assert_eq!(
        vec4t!(a, b, c, d).to_string(),
        format!("({a}, {b}, {c}, {d})")
    );

    assert_eq!(format!("{:?}", vec2t!(a, b)), format!("({a:?}, {b:?})"));
    assert_eq!(
        format!("{:?}", vec3t!(a, b, c)),
        format!("({a:?}, {b:?}, {c:?})")
    );
    assert_eq!(
        format!("{:?}", vec4t!(a, b, c, d)),
        format!("({a:?}, {b:?}, {c:?}, {d:?})")
    );

    assert_eq!(vec2t!(a, b).x, a);
    assert_eq!(vec2t!(a, b).y, b);

    assert_eq!(vec3t!(a, b, c).x, a);
    assert_eq!(vec3t!(a, b, c).y, b);
    assert_eq!(vec3t!(a, b, c).z, c);

    assert_eq!(vec4t!(a, b, c, d).x, a);
    assert_eq!(vec4t!(a, b, c, d).y, b);
    assert_eq!(vec4t!(a, b, c, d).z, c);
    assert_eq!(vec4t!(a, b, c, d).w, d);

    assert_eq!(&mut vec2t!(a, b).x, &mut a.clone());
    assert_eq!(&mut vec2t!(a, b).y, &mut b.clone());

    assert_eq!(&mut vec3t!(a, b, c).x, &mut a.clone());
    assert_eq!(&mut vec3t!(a, b, c).y, &mut b.clone());
    assert_eq!(&mut vec3t!(a, b, c).z, &mut c.clone());

    assert_eq!(&mut vec4t!(a, b, c, d).x, &mut a.clone());
    assert_eq!(&mut vec4t!(a, b, c, d).y, &mut b.clone());
    assert_eq!(&mut vec4t!(a, b, c, d).z, &mut c.clone());
    assert_eq!(&mut vec4t!(a, b, c, d).w, &mut d.clone());

    #[cfg(feature = "swizzle")]
    {
        assert_eq!(vec2t!(a, b).xy(), vec2t!(a, b));
        assert_eq!(vec2t!(a, b).yx(), vec2t!(b, a));
        assert_eq!(vec2t!(a, b).yy(), vec2t!(b, b));

        assert_eq!(vec3t!(a, b, c).xy(), vec2t!(a, b));
        assert_eq!(vec3t!(a, b, c).xz(), vec2t!(a, c));
        assert_eq!(vec3t!(a, b, c).zy(), vec2t!(c, b));

        assert_eq!(vec4t!(a, b, c, d).xy(), vec2t!(a, b));
        assert_eq!(vec4t!(a, b, c, d).yw(), vec2t!(b, d));
        assert_eq!(vec4t!(a, b, c, d).wy(), vec2t!(d, b));

        assert_eq!(vec2t!(a, b).xyy(), vec3t!(a, b, b));
        assert_eq!(vec2t!(a, b).yxy(), vec3t!(b, a, b));

        assert_eq!(vec3t!(a, b, c).xyz(), vec3t!(a, b, c));
        assert_eq!(vec3t!(a, b, c).yxz(), vec3t!(b, a, c));
        assert_eq!(vec3t!(a, b, c).zyx(), vec3t!(c, b, a));

        assert_eq!(vec4t!(a, b, c, d).xyz(), vec3t!(a, b, c));
        assert_eq!(vec4t!(a, b, c, d).yxz(), vec3t!(b, a, c));
        assert_eq!(vec4t!(a, b, c, d).zyx(), vec3t!(c, b, a));

        assert_eq!(vec2t!(a, b).xyyx(), vec4t!(a, b, b, a));
        assert_eq!(vec2t!(a, b).yxyx(), vec4t!(b, a, b, a));
        assert_eq!(vec2t!(a, b).yyxx(), vec4t!(b, b, a, a));

        assert_eq!(vec3t!(a, b, c).xyzx(), vec4t!(a, b, c, a));
        assert_eq!(vec3t!(a, b, c).yxzx(), vec4t!(b, a, c, a));
        assert_eq!(vec3t!(a, b, c).zyxx(), vec4t!(c, b, a, a));

        assert_eq!(vec4t!(a, b, c, d).xyzw(), vec4t!(a, b, c, d));
        assert_eq!(vec4t!(a, b, c, d).yxzw(), vec4t!(b, a, c, d));
        assert_eq!(vec4t!(a, b, c, d).zyxw(), vec4t!(c, b, a, d));
        assert_eq!(vec4t!(a, b, c, d).wyzx(), vec4t!(d, b, c, a));
    }
}

#[test]
fn test_f32_simd_primitive_api() {
    test_primitive_api::<f32, Simd>(1.0, 2.0, 3.0, 4.0);
}
#[test]
fn test_f32_nonsimd_primitive_api() {
    test_primitive_api::<f32, NonSimd>(1.0, 2.0, 3.0, 4.0);
}

#[test]
fn test_f64_simd_primitive_api() {
    test_primitive_api::<f64, Simd>(1.0, 2.0, 3.0, 4.0);
}
#[test]
fn test_f64_nonsimd_primitive_api() {
    test_primitive_api::<f64, NonSimd>(1.0, 2.0, 3.0, 4.0);
}

#[test]
fn test_i8_simd_primitive_api() {
    test_primitive_api::<i8, Simd>(1, 2, 3, 4);
}
#[test]
fn test_i8_nonsimd_primitive_api() {
    test_primitive_api::<i8, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_i16_simd_primitive_api() {
    test_primitive_api::<i16, Simd>(1, 2, 3, 4);
}
#[test]
fn test_i16_nonsimd_primitive_api() {
    test_primitive_api::<i16, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_i32_simd_primitive_api() {
    test_primitive_api::<i32, Simd>(1, 2, 3, 4);
}
#[test]
fn test_i32_nonsimd_primitive_api() {
    test_primitive_api::<i32, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_i64_simd_primitive_api() {
    test_primitive_api::<i64, Simd>(1, 2, 3, 4);
}
#[test]
fn test_i64_nonsimd_primitive_api() {
    test_primitive_api::<i64, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_i128_simd_primitive_api() {
    test_primitive_api::<i128, Simd>(1, 2, 3, 4);
}
#[test]
fn test_i128_nonsimd_primitive_api() {
    test_primitive_api::<i128, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_isize_simd_primitive_api() {
    test_primitive_api::<isize, Simd>(1, 2, 3, 4);
}
#[test]
fn test_isize_nonsimd_primitive_api() {
    test_primitive_api::<isize, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_u8_simd_primitive_api() {
    test_primitive_api::<u8, Simd>(1, 2, 3, 4);
}
#[test]
fn test_u8_nonsimd_primitive_api() {
    test_primitive_api::<u8, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_u16_simd_primitive_api() {
    test_primitive_api::<u16, Simd>(1, 2, 3, 4);
}
#[test]
fn test_u16_nonsimd_primitive_api() {
    test_primitive_api::<u16, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_u32_simd_primitive_api() {
    test_primitive_api::<u32, Simd>(1, 2, 3, 4);
}
#[test]
fn test_u32_nonsimd_primitive_api() {
    test_primitive_api::<u32, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_u64_simd_primitive_api() {
    test_primitive_api::<u64, Simd>(1, 2, 3, 4);
}
#[test]
fn test_u64_nonsimd_primitive_api() {
    test_primitive_api::<u64, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_u128_simd_primitive_api() {
    test_primitive_api::<u128, Simd>(1, 2, 3, 4);
}
#[test]
fn test_u128_nonsimd_primitive_api() {
    test_primitive_api::<u128, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_usize_simd_primitive_api() {
    test_primitive_api::<usize, Simd>(1, 2, 3, 4);
}
#[test]
fn test_usize_nonsimd_primitive_api() {
    test_primitive_api::<usize, NonSimd>(1, 2, 3, 4);
}

#[test]
fn test_bool_simd_primitive_api() {
    test_primitive_api::<bool, Simd>(false, true, false, true);
}
#[test]
fn test_bool_nonsimd_primitive_api() {
    test_primitive_api::<bool, NonSimd>(false, true, false, true);
}
