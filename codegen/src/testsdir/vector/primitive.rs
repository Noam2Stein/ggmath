use genco::quote;

use crate::{iter::Primitive, testsdir::TokensExtendExt, util::TokensExt};

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

    result.extend(quote!(
        use ggmath::*;

        #[test]
        fn test_nonsimd_$(t)_vec_layout() {
            assert_eq!(size_of::<Vec2S<$t>>(), size_of::<$t>() * 2);
            assert_eq!(size_of::<Vec3S<$t>>(), size_of::<$t>() * 3);
            assert_eq!(size_of::<Vec4S<$t>>(), size_of::<$t>() * 4);

            assert_eq!(align_of::<Vec2S<$t>>(), align_of::<$t>());
            assert_eq!(align_of::<Vec3S<$t>>(), align_of::<$t>());
            assert_eq!(align_of::<Vec4S<$t>>(), align_of::<$t>());
        }
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
            fn test_$(s.snakecase())_$(t)_vec_constructors() {
                assert_eq!($Vec2::from_array([$val1, $val2]).as_array(), [$val1, $val2]);
                assert_eq!($Vec3::from_array([$val1, $val2, $val3]).as_array(), [$val1, $val2, $val3]);
                assert_eq!($Vec4::from_array([$val1, $val2, $val3, $val4]).as_array(), [$val1, $val2, $val3, $val4]);

                assert_eq!($Vec2::splat($val1).as_array(), [$val1; 2]);
                assert_eq!($Vec3::splat($val1).as_array(), [$val1; 3]);
                assert_eq!($Vec4::splat($val1).as_array(), [$val1; 4]);

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
            }
        )
    });

    result.write_in_tests(format!("vector/{t}.rs"));
}
