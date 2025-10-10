use genco::{quote, tokens::quoted};

use crate::{
    iter::{Primitive, Simdness},
    testsdir::{vector::{float, int}, TokensExtendExt},
    util::TokensExt,
};

pub fn generate(t: Primitive) {
    let mut result = quote!();

    // tests rely on `val2` being greater than `val1`, and `val4` being greater than `val3`.
    let (val1, val2, val3, val4) = &match t {
        Primitive::Float(t) => (
            format!("1.0{t}"),
            format!("2.0{t}"),
            format!("3.0{t}"),
            format!("4.0{t}"),
        ),
        Primitive::Int(t) => (
            format!("1{t}"),
            format!("2{t}"),
            format!("3{t}"),
            format!("4{t}"),
        ),
        Primitive::Bool => (
            "false".to_string(),
            "true".to_string(),
            "false".to_string(),
            "true".to_string(),
        ),
        Primitive::Char => (
            "'a'".to_string(),
            "'b'".to_string(),
            "'c'".to_string(),
            "'d'".to_string(),
        ),
    };

    let (val1_dbg, val2_dbg, val3_dbg, val4_dbg) = &match t {
        Primitive::Float(_) => ("1.0", "2.0", "3.0", "4.0"),
        Primitive::Int(_) => ("1", "2", "3", "4"),
        Primitive::Bool => ("false", "true", "false", "true"),
        Primitive::Char => ("'a'", "'b'", "'c'", "'d'"),
    };

    let (val1_display, val2_display, val3_display, val4_display) = &match t {
        Primitive::Float(_) => ("1", "2", "3", "4"),
        Primitive::Int(_) => ("1", "2", "3", "4"),
        Primitive::Bool => ("false", "true", "false", "true"),
        Primitive::Char => ("a", "b", "c", "d"),
    };

    result.extend(quote!(
        use ggmath::*;
    ));

    #[allow(non_snake_case)]
    result.extend_for_simdness(|s| {
        quote!(
            $(let Vec2 = &format!("Vec2{}", s.uppercase_postfix()))
            $(let Vec3 = &format!("Vec3{}", s.uppercase_postfix()))
            $(let Vec4 = &format!("Vec4{}", s.uppercase_postfix()))

            $(let vec2 = &format!("vec2{}", s.lowercase_postfix()))
            $(let vec3 = &format!("vec3{}", s.lowercase_postfix()))
            $(let vec4 = &format!("vec4{}", s.lowercase_postfix()))

            #[test]
            fn test_$(s.snakecase())_primitive_fns() {
                $(
                    if s == Simdness::NonSimd =>

                    assert_eq!(size_of::<Vec2S<$t>>(), size_of::<$t>() * 2);
                    assert_eq!(size_of::<Vec3S<$t>>(), size_of::<$t>() * 3);
                    assert_eq!(size_of::<Vec4S<$t>>(), size_of::<$t>() * 4);

                    assert_eq!(align_of::<Vec2S<$t>>(), align_of::<$t>());
                    assert_eq!(align_of::<Vec3S<$t>>(), align_of::<$t>());
                    assert_eq!(align_of::<Vec4S<$t>>(), align_of::<$t>());
                )

                assert_eq!($Vec2::from_array([$val1, $val2]).as_array(), [$val1, $val2]);
                assert_eq!($Vec3::from_array([$val1, $val2, $val3]).as_array(), [$val1, $val2, $val3]);
                assert_eq!($Vec4::from_array([$val1, $val2, $val3, $val4]).as_array(), [$val1, $val2, $val3, $val4]);

                assert_eq!($Vec2::from_fn(|i| [$val1, $val2][i]).as_array(), [$val1, $val2]);
                assert_eq!($Vec3::from_fn(|i| [$val1, $val2, $val3][i]).as_array(), [$val1, $val2, $val3]);
                assert_eq!($Vec4::from_fn(|i| [$val1, $val2, $val3, $val4][i]).as_array(), [$val1, $val2, $val3, $val4]);

                assert_eq!($Vec2::const_from_array([$val1, $val2]).as_array(), [$val1, $val2]);
                assert_eq!($Vec3::const_from_array([$val1, $val2, $val3]).as_array(), [$val1, $val2, $val3]);
                assert_eq!($Vec4::const_from_array([$val1, $val2, $val3, $val4]).as_array(), [$val1, $val2, $val3, $val4]);

                assert_eq!($vec2!($val1, $val2).as_array(), [$val1, $val2]);
                assert_eq!($vec2!($vec2!($val1, $val2)).as_array(), [$val1, $val2]);
                assert_eq!($vec3!($val1, $val2, $val3).as_array(), [$val1, $val2, $val3]);
                assert_eq!($vec3!($val1, $vec2!($val2, $val3)).as_array(), [$val1, $val2, $val3]);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).as_array(), [$val1, $val2, $val3, $val4]);
                assert_eq!($vec4!($val1, $vec2!($val2, $val3), $val4).as_array(), [$val1, $val2, $val3, $val4]);
                assert_eq!($vec4!($val1, $vec2!($val2, $val3), Vector::<1, $t, $s>::from_array([$val4])).as_array(), [$val1, $val2, $val3, $val4]);

                assert_eq!($vec2!($val1).as_array(), [$val1; 2]);
                assert_eq!($vec3!($val1).as_array(), [$val1; 3]);
                assert_eq!($vec4!($val1).as_array(), [$val1; 4]);

                assert_eq!($vec2!($val1, $val2).as_array_ref(), &[$val1, $val2]);
                assert_eq!($vec3!($val1, $val2, $val3).as_array_ref(), &[$val1, $val2, $val3]);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).as_array_ref(), &[$val1, $val2, $val3, $val4]);

                assert_eq!($vec2!($val1, $val2).as_mut_array(), &mut [$val1, $val2]);
                assert_eq!($vec3!($val1, $val2, $val3).as_mut_array(), &mut [$val1, $val2, $val3]);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).as_mut_array(), &mut [$val1, $val2, $val3, $val4]);

                assert_eq!($vec2!($val1, $val2).as_simd(), vec2!($val1, $val2));
                assert_eq!($vec3!($val1, $val2, $val3).as_simd(), vec3!($val1, $val2, $val3));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).as_simd(), vec4!($val1, $val2, $val3, $val4));

                assert_eq!($vec2!($val1, $val2).as_nonsimd(), vec2s!($val1, $val2));
                assert_eq!($vec3!($val1, $val2, $val3).as_nonsimd(), vec3s!($val1, $val2, $val3));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).as_nonsimd(), vec4s!($val1, $val2, $val3, $val4));

                assert_eq!($vec2!($val1, $val2).len(), 2);
                assert_eq!($vec3!($val1, $val2, $val3).len(), 3);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).len(), 4);

                assert_eq!($vec2!($val1, $val2).is_simd(), $((s == Simdness::Simd).to_string()));
                assert_eq!($vec3!($val1, $val2, $val3).is_simd(), $((s == Simdness::Simd).to_string()));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).is_simd(), $((s == Simdness::Simd).to_string()));

                assert_eq!($vec2!($val1, $val2).get(0), Some($val1));
                assert_eq!($vec2!($val1, $val2).get(1), Some($val2));
                assert_eq!($vec2!($val1, $val2).get(2), None);
                assert_eq!($vec2!($val1, $val2).get(3), None);

                assert_eq!($vec3!($val1, $val2, $val3).get(0), Some($val1));
                assert_eq!($vec3!($val1, $val2, $val3).get(1), Some($val2));
                assert_eq!($vec3!($val1, $val2, $val3).get(2), Some($val3));
                assert_eq!($vec3!($val1, $val2, $val3).get(3), None);
                assert_eq!($vec3!($val1, $val2, $val3).get(4), None);

                assert_eq!($vec4!($val1, $val2, $val3, $val4).get(0), Some($val1));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get(1), Some($val2));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get(2), Some($val3));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get(3), Some($val4));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get(4), None);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get(5), None);

                assert_eq!($vec2!($val1, $val2).get_mut(0), Some(&mut $val1));
                assert_eq!($vec2!($val1, $val2).get_mut(1), Some(&mut $val2));
                assert_eq!($vec2!($val1, $val2).get_mut(2), None);
                assert_eq!($vec2!($val1, $val2).get_mut(3), None);

                assert_eq!($vec3!($val1, $val2, $val3).get_mut(0), Some(&mut $val1));
                assert_eq!($vec3!($val1, $val2, $val3).get_mut(1), Some(&mut $val2));
                assert_eq!($vec3!($val1, $val2, $val3).get_mut(2), Some(&mut $val3));
                assert_eq!($vec3!($val1, $val2, $val3).get_mut(3), None);
                assert_eq!($vec3!($val1, $val2, $val3).get_mut(4), None);

                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_mut(0), Some(&mut $val1));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_mut(1), Some(&mut $val2));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_mut(2), Some(&mut $val3));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_mut(3), Some(&mut $val4));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_mut(4), None);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_mut(5), None);

                // SAFETY: only test indices that are in bounds
                unsafe {
                    assert_eq!($vec2!($val1, $val2).get_unchecked(0), $val1);
                    assert_eq!($vec2!($val1, $val2).get_unchecked(1), $val2);

                    assert_eq!($vec3!($val1, $val2, $val3).get_unchecked(0), $val1);
                    assert_eq!($vec3!($val1, $val2, $val3).get_unchecked(1), $val2);
                    assert_eq!($vec3!($val1, $val2, $val3).get_unchecked(2), $val3);

                    assert_eq!($vec4!($val1, $val2, $val3, $val4).get_unchecked(0), $val1);
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).get_unchecked(1), $val2);
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).get_unchecked(2), $val3);
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).get_unchecked(3), $val4);

                    assert_eq!($vec2!($val1, $val2).get_unchecked_mut(0), &mut $val1);
                    assert_eq!($vec2!($val1, $val2).get_unchecked_mut(1), &mut $val2);

                    assert_eq!($vec3!($val1, $val2, $val3).get_unchecked_mut(0), &mut $val1);
                    assert_eq!($vec3!($val1, $val2, $val3).get_unchecked_mut(1), &mut $val2);
                    assert_eq!($vec3!($val1, $val2, $val3).get_unchecked_mut(2), &mut $val3);

                    assert_eq!($vec4!($val1, $val2, $val3, $val4).get_unchecked_mut(0), &mut $val1);
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).get_unchecked_mut(1), &mut $val2);
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).get_unchecked_mut(2), &mut $val3);
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).get_unchecked_mut(3), &mut $val4);
                }

                assert_eq!($vec2!($val1, $val2).iter().collect::<Vec<$t>>(), vec![$val1, $val2]);
                assert_eq!($vec3!($val1, $val2, $val3).iter().collect::<Vec<$t>>(), vec![$val1, $val2, $val3]);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).iter().collect::<Vec<$t>>(), vec![$val1, $val2, $val3, $val4]);

                assert_eq!($vec2!($val1, $val2).iter_mut().collect::<Vec<&mut $t>>(), vec![&mut $val1, &mut $val2]);
                assert_eq!($vec3!($val1, $val2, $val3).iter_mut().collect::<Vec<&mut $t>>(), vec![&mut $val1, &mut $val2, &mut $val3]);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).iter_mut().collect::<Vec<&mut $t>>(), vec![&mut $val1, &mut $val2, &mut $val3, &mut $val4]);

                assert_eq!($vec2!($val1, $val2).map(|x| x == $val2), $vec2!(false, true));
                assert_eq!($vec3!($val1, $val2, $val3).map(|x| x == $val2), $vec3!(false, true, $((val3 == val2).to_string())));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).map(|x| x == $val2), $vec4!(false, true, $((val3 == val2).to_string()), $((val4 == val2).to_string())));

                assert_eq!($vec2!($val1, $val2).zip($vec2!($val2, $val1)), $vec2!(($val1, $val2), ($val2, $val1)));
                assert_eq!($vec3!($val1, $val2, $val3).zip($vec3!($val2, $val3, $val1)), $vec3!(($val1, $val2), ($val2, $val3), ($val3, $val1)));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).zip($vec4!($val2, $val3, $val4, $val1)), $vec4!(($val1, $val2), ($val2, $val3), ($val3, $val4), ($val4, $val1)));

                assert_eq!($vec2!($val1, $val2).reverse(), $vec2!($val2, $val1));
                assert_eq!($vec3!($val1, $val2, $val3).reverse(), $vec3!($val3, $val2, $val1));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).reverse(), $vec4!($val4, $val3, $val2, $val1));

                assert_eq!($vec2!($val1, $val2).get_const_vec2::<0, 1>(), $vec2!($val1, $val2));
                assert_eq!($vec2!($val1, $val2).get_const_vec2::<1, 0>(), $vec2!($val2, $val1));
                assert_eq!($vec2!($val1, $val2).get_const_vec2::<1, 1>(), $vec2!($val2, $val2));

                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec2::<0, 1>(), $vec2!($val1, $val2));
                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec2::<0, 2>(), $vec2!($val1, $val3));
                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec2::<2, 1>(), $vec2!($val3, $val2));

                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec2::<0, 1>(), $vec2!($val1, $val2));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec2::<1, 3>(), $vec2!($val2, $val4));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec2::<3, 1>(), $vec2!($val4, $val2));

                assert_eq!($vec2!($val1, $val2).get_const_vec3::<0, 1, 1>(), $vec3!($val1, $val2, $val2));
                assert_eq!($vec2!($val1, $val2).get_const_vec3::<1, 0, 1>(), $vec3!($val2, $val1, $val2));

                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec3::<0, 1, 2>(), $vec3!($val1, $val2, $val3));
                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec3::<1, 0, 2>(), $vec3!($val2, $val1, $val3));
                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec3::<2, 1, 0>(), $vec3!($val3, $val2, $val1));

                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec3::<0, 1, 2>(), $vec3!($val1, $val2, $val3));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec3::<1, 0, 2>(), $vec3!($val2, $val1, $val3));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec3::<2, 1, 0>(), $vec3!($val3, $val2, $val1));

                assert_eq!($vec2!($val1, $val2).get_const_vec4::<0, 1, 1, 0>(), $vec4!($val1, $val2, $val2, $val1));
                assert_eq!($vec2!($val1, $val2).get_const_vec4::<1, 0, 1, 0>(), $vec4!($val2, $val1, $val2, $val1));
                assert_eq!($vec2!($val1, $val2).get_const_vec4::<1, 1, 0, 0>(), $vec4!($val2, $val2, $val1, $val1));

                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec4::<0, 1, 2, 0>(), $vec4!($val1, $val2, $val3, $val1));
                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec4::<1, 0, 2, 0>(), $vec4!($val2, $val1, $val3, $val1));
                assert_eq!($vec3!($val1, $val2, $val3).get_const_vec4::<2, 1, 0, 0>(), $vec4!($val3, $val2, $val1, $val1));

                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec4::<0, 1, 2, 3>(), $vec4!($val1, $val2, $val3, $val4));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec4::<1, 0, 2, 3>(), $vec4!($val2, $val1, $val3, $val4));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec4::<2, 1, 0, 3>(), $vec4!($val3, $val2, $val1, $val4));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).get_const_vec4::<3, 1, 2, 0>(), $vec4!($val4, $val2, $val3, $val1));

                assert_eq!($vec2!($val1, $val2)[0], $val1);
                assert_eq!($vec2!($val1, $val2)[1], $val2);

                assert_eq!($vec3!($val1, $val2, $val3)[0], $val1);
                assert_eq!($vec3!($val1, $val2, $val3)[1], $val2);
                assert_eq!($vec3!($val1, $val2, $val3)[2], $val3);

                assert_eq!($vec4!($val1, $val2, $val3, $val4)[0], $val1);
                assert_eq!($vec4!($val1, $val2, $val3, $val4)[1], $val2);
                assert_eq!($vec4!($val1, $val2, $val3, $val4)[2], $val3);
                assert_eq!($vec4!($val1, $val2, $val3, $val4)[3], $val4);

                assert_eq!(&mut $vec2!($val1, $val2)[0], &mut $val1);
                assert_eq!(&mut $vec2!($val1, $val2)[1], &mut $val2);

                assert_eq!(&mut $vec3!($val1, $val2, $val3)[0], &mut $val1);
                assert_eq!(&mut $vec3!($val1, $val2, $val3)[1], &mut $val2);
                assert_eq!(&mut $vec3!($val1, $val2, $val3)[2], &mut $val3);

                assert_eq!(&mut $vec4!($val1, $val2, $val3, $val4)[0], &mut $val1);
                assert_eq!(&mut $vec4!($val1, $val2, $val3, $val4)[1], &mut $val2);
                assert_eq!(&mut $vec4!($val1, $val2, $val3, $val4)[2], &mut $val3);
                assert_eq!(&mut $vec4!($val1, $val2, $val3, $val4)[3], &mut $val4);

                assert_eq!($vec2!($val1, $val2) == $vec2!($val1, $val2), true);
                assert_eq!($vec2!($val1, $val2) == $vec2!($val2, $val1), false);
                assert_eq!($vec2!($val1, $val2) == $vec2!($val2, $val2), false);

                assert_eq!($vec3!($val1, $val2, $val3) == $vec3!($val1, $val2, $val3), true);
                assert_eq!($vec3!($val1, $val2, $val3) == $vec3!($val4, $val3, $val2), false);
                assert_eq!($vec3!($val1, $val2, $val3) == $vec3!($val2, $val2, $val3), false);

                assert_eq!($vec4!($val1, $val2, $val3, $val4) == $vec4!($val1, $val2, $val3, $val4), true);
                assert_eq!($vec4!($val1, $val2, $val3, $val4) == $vec4!($val4, $val3, $val2, $val1), false);
                assert_eq!($vec4!($val1, $val2, $val3, $val4) == $vec4!($val2, $val2, $val2, $val4), false);

                assert_eq!($vec2!($val1, $val2) != $vec2!($val1, $val2), false);
                assert_eq!($vec2!($val1, $val2) != $vec2!($val2, $val1), true);
                assert_eq!($vec2!($val1, $val2) != $vec2!($val2, $val2), true);

                assert_eq!($vec3!($val1, $val2, $val3) != $vec3!($val1, $val2, $val3), false);
                assert_eq!($vec3!($val1, $val2, $val3) != $vec3!($val4, $val3, $val2), true);
                assert_eq!($vec3!($val1, $val2, $val3) != $vec3!($val2, $val2, $val3), true);

                assert_eq!($vec4!($val1, $val2, $val3, $val4) != $vec4!($val1, $val2, $val3, $val4), false);
                assert_eq!($vec4!($val1, $val2, $val3, $val4) != $vec4!($val4, $val3, $val2, $val1), true);
                assert_eq!($vec4!($val1, $val2, $val3, $val4) != $vec4!($val2, $val2, $val2, $val4), true);

                assert_eq!($vec2!($val1, $val2).to_string(), $(quoted(format!("({val1_display}, {val2_display})"))));
                assert_eq!($vec3!($val1, $val2, $val3).to_string(), $(quoted(format!("({val1_display}, {val2_display}, {val3_display})"))));
                assert_eq!($vec4!($val1, $val2, $val3, $val4).to_string(), $(quoted(format!("({val1_display}, {val2_display}, {val3_display}, {val4_display})"))));

                assert_eq!(format!("{:?}", $vec2!($val1, $val2)), $(quoted(format!("({val1_dbg}, {val2_dbg})"))));
                assert_eq!(format!("{:?}", $vec3!($val1, $val2, $val3)), $(quoted(format!("({val1_dbg}, {val2_dbg}, {val3_dbg})"))));
                assert_eq!(format!("{:?}", $vec4!($val1, $val2, $val3, $val4)), $(quoted(format!("({val1_dbg}, {val2_dbg}, {val3_dbg}, {val4_dbg})"))));

                assert_eq!($vec2!($val1, $val2).x, $val1);
                assert_eq!($vec2!($val1, $val2).y, $val2);

                assert_eq!($vec3!($val1, $val2, $val3).x, $val1);
                assert_eq!($vec3!($val1, $val2, $val3).y, $val2);
                assert_eq!($vec3!($val1, $val2, $val3).z, $val3);

                assert_eq!($vec4!($val1, $val2, $val3, $val4).x, $val1);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).y, $val2);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).z, $val3);
                assert_eq!($vec4!($val1, $val2, $val3, $val4).w, $val4);

                assert_eq!(&mut $vec2!($val1, $val2).x, &mut $val1);
                assert_eq!(&mut $vec2!($val1, $val2).y, &mut $val2);

                assert_eq!(&mut $vec3!($val1, $val2, $val3).x, &mut $val1);
                assert_eq!(&mut $vec3!($val1, $val2, $val3).y, &mut $val2);
                assert_eq!(&mut $vec3!($val1, $val2, $val3).z, &mut $val3);

                assert_eq!(&mut $vec4!($val1, $val2, $val3, $val4).x, &mut $val1);
                assert_eq!(&mut $vec4!($val1, $val2, $val3, $val4).y, &mut $val2);
                assert_eq!(&mut $vec4!($val1, $val2, $val3, $val4).z, &mut $val3);
                assert_eq!(&mut $vec4!($val1, $val2, $val3, $val4).w, &mut $val4);

                #[cfg(feature = "swizzle")]
                {
                    assert_eq!($vec2!($val1, $val2).xy(), $vec2!($val1, $val2));
                    assert_eq!($vec2!($val1, $val2).yx(), $vec2!($val2, $val1));
                    assert_eq!($vec2!($val1, $val2).yy(), $vec2!($val2, $val2));

                    assert_eq!($vec3!($val1, $val2, $val3).xy(), $vec2!($val1, $val2));
                    assert_eq!($vec3!($val1, $val2, $val3).xz(), $vec2!($val1, $val3));
                    assert_eq!($vec3!($val1, $val2, $val3).zy(), $vec2!($val3, $val2));

                    assert_eq!($vec4!($val1, $val2, $val3, $val4).xy(), $vec2!($val1, $val2));
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).yw(), $vec2!($val2, $val4));
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).wy(), $vec2!($val4, $val2));

                    assert_eq!($vec2!($val1, $val2).xyy(), $vec3!($val1, $val2, $val2));
                    assert_eq!($vec2!($val1, $val2).yxy(), $vec3!($val2, $val1, $val2));

                    assert_eq!($vec3!($val1, $val2, $val3).xyz(), $vec3!($val1, $val2, $val3));
                    assert_eq!($vec3!($val1, $val2, $val3).yxz(), $vec3!($val2, $val1, $val3));
                    assert_eq!($vec3!($val1, $val2, $val3).zyx(), $vec3!($val3, $val2, $val1));

                    assert_eq!($vec4!($val1, $val2, $val3, $val4).xyz(), $vec3!($val1, $val2, $val3));
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).yxz(), $vec3!($val2, $val1, $val3));
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).zyx(), $vec3!($val3, $val2, $val1));

                    assert_eq!($vec2!($val1, $val2).xyyx(), $vec4!($val1, $val2, $val2, $val1));
                    assert_eq!($vec2!($val1, $val2).yxyx(), $vec4!($val2, $val1, $val2, $val1));
                    assert_eq!($vec2!($val1, $val2).yyxx(), $vec4!($val2, $val2, $val1, $val1));

                    assert_eq!($vec3!($val1, $val2, $val3).xyzx(), $vec4!($val1, $val2, $val3, $val1));
                    assert_eq!($vec3!($val1, $val2, $val3).yxzx(), $vec4!($val2, $val1, $val3, $val1));
                    assert_eq!($vec3!($val1, $val2, $val3).zyxx(), $vec4!($val3, $val2, $val1, $val1));

                    assert_eq!($vec4!($val1, $val2, $val3, $val4).xyzw(), $vec4!($val1, $val2, $val3, $val4));
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).yxzw(), $vec4!($val2, $val1, $val3, $val4));
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).zyxw(), $vec4!($val3, $val2, $val1, $val4));
                    assert_eq!($vec4!($val1, $val2, $val3, $val4).wyzx(), $vec4!($val4, $val2, $val3, $val1));
                }
            }

            #[test]
            #[should_panic]
            fn test_vec2$(s.lowercase_postfix())_index_panic() {
                vec2!($val1, $val2)[2];
            }

            #[test]
            #[should_panic]
            fn test_vec3$(s.lowercase_postfix())_index_panic() {
                vec3!($val1, $val2, $val3)[3];
            }
            
            #[test]
            #[should_panic]
            fn test_vec4$(s.lowercase_postfix())_index_panic() {
                vec4!($val1, $val2, $val3, $val4)[4];
            }
        )
    });

    match t {
        Primitive::Int(t) => int::generate(t, &mut result),
        Primitive::Float(t) => float::generate(t, &mut result),
        _ => {},
    }

    result.write_in_tests(format!("vector/{t}.rs"));
}
