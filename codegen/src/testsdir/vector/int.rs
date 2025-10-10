use genco::{lang::rust::Tokens, quote};

use crate::{iter::Int, testsdir::TokensExtendExt};

pub fn generate(t: Int, result: &mut Tokens) {
    #[expect(non_snake_case)]
    result.extend_for_simdness(|s| {
        quote!(
            $(let Vec2 = &format!("Vec2{}", s.uppercase_postfix()))
            $(let Vec3 = &format!("Vec3{}", s.uppercase_postfix()))
            $(let Vec4 = &format!("Vec4{}", s.uppercase_postfix()))
            $(let vec2 = &format!("vec2{}", s.lowercase_postfix()))
            $(let vec3 = &format!("vec3{}", s.lowercase_postfix()))
            $(let vec4 = &format!("vec4{}", s.lowercase_postfix()))

            #[test]
            fn test_$(s.snakecase())_int_fns() {
                assert_eq!(!$vec2!(5$t, 7$t), $vec2!(!5$t, !7$t));
                assert_eq!(!$vec3!(5$t, 7$t, 9$t), $vec3!(!5$t, !7$t, !9$t));
                assert_eq!(!$vec4!(5$t, 7$t, 9$t, 11$t), $vec4!(!5$t, !7$t, !9$t, !11$t));

                assert_eq!($vec2!(5$t, 7$t) + $vec2!(9$t, 11$t), $vec2!(5$t + 9$t, 7$t + 11$t));
                assert_eq!($vec3!(5$t, 7$t, 9$t) + $vec3!(11$t, 13$t, 15$t), $vec3!(5$t + 11$t, 7$t + 13$t, 9$t + 15$t));
                assert_eq!($vec4!(5$t, 7$t, 9$t, 11$t) + $vec4!(13$t, 15$t, 17$t, 19$t), $vec4!(5$t + 13$t, 7$t + 15$t, 9$t + 17$t, 11$t + 19$t));

                assert_eq!($vec2!(9$t, 11$t) - $vec2!(5$t, 7$t), $vec2!(9$t - 5$t, 11$t - 7$t));
                assert_eq!($vec3!(9$t, 11$t, 13$t) - $vec3!(5$t, 7$t, 9$t), $vec3!(9$t - 5$t, 11$t - 7$t, 13$t - 9$t));
                assert_eq!($vec4!(9$t, 11$t, 13$t, 15$t) - $vec4!(5$t, 7$t, 9$t, 11$t), $vec4!(9$t - 5$t, 11$t - 7$t, 13$t - 9$t, 15$t - 11$t));

                assert_eq!($vec2!(2$t, 4$t) * $vec2!(3$t, 5$t), $vec2!(2$t * 3$t, 4$t * 5$t));
                assert_eq!($vec3!(2$t, 4$t, 6$t) * $vec3!(3$t, 5$t, 7$t), $vec3!(2$t * 3$t, 4$t * 5$t, 6$t * 7$t));
                assert_eq!($vec4!(2$t, 4$t, 6$t, 8$t) * $vec4!(3$t, 5$t, 7$t, 9$t), $vec4!(2$t * 3$t, 4$t * 5$t, 6$t * 7$t, 8$t * 9$t));

                assert_eq!($vec2!(21$t, 23$t) / $vec2!(4$t, 50$t), $vec2!(21$t / 4$t, 23$t / 50$t));
                assert_eq!($vec3!(21$t, 23$t, 25$t) / $vec3!(4$t, 50$t, 3$t), $vec3!(21$t / 4$t, 23$t / 50$t, 25$t / 3$t));
                assert_eq!($vec4!(21$t, 23$t, 25$t, 27$t) / $vec4!(4$t, 50$t, 3$t, 10$t), $vec4!(21$t / 4$t, 23$t / 50$t, 25$t / 3$t, 27$t / 10$t));

                assert_eq!($vec2!(16$t, 19$t) / $vec2!(5$t, 5$t), $vec2!(16$t / 5$t, 19$t / 5$t));
                assert_eq!($vec3!(16$t, 19$t, 22$t) / $vec3!(5$t, 5$t, 5$t), $vec3!(16$t / 5$t, 19$t / 5$t, 22$t / 5$t));
                assert_eq!($vec4!(16$t, 19$t, 22$t, 25$t) / $vec4!(5$t, 5$t, 5$t, 5$t), $vec4!(16$t / 5$t, 19$t / 5$t, 22$t / 5$t, 25$t / 5$t));

                assert_eq!($vec2!(21$t, 23$t) % $vec2!(4$t, 50$t), $vec2!(21$t % 4$t, 23$t % 50$t));
                assert_eq!($vec3!(21$t, 23$t, 25$t) % $vec3!(4$t, 50$t, 3$t), $vec3!(21$t % 4$t, 23$t % 50$t, 25$t % 3$t));
                assert_eq!($vec4!(21$t, 23$t, 25$t, 27$t) % $vec4!(4$t, 50$t, 3$t, 10$t), $vec4!(21$t % 4$t, 23$t % 50$t, 25$t % 3$t, 27$t % 10$t));

                assert_eq!($vec2!(21$t, 23$t) & $vec2!(4$t, 50$t), $vec2!(21$t & 4$t, 23$t & 50$t));
                assert_eq!($vec3!(21$t, 23$t, 25$t) & $vec3!(4$t, 50$t, 3$t), $vec3!(21$t & 4$t, 23$t & 50$t, 25$t & 3$t));
                assert_eq!($vec4!(21$t, 23$t, 25$t, 27$t) & $vec4!(4$t, 50$t, 3$t, 10$t), $vec4!(21$t & 4$t, 23$t & 50$t, 25$t & 3$t, 27$t & 10$t));

                assert_eq!($vec2!(21$t, 23$t) | $vec2!(4$t, 50$t), $vec2!(21$t | 4$t, 23$t | 50$t));
                assert_eq!($vec3!(21$t, 23$t, 25$t) | $vec3!(4$t, 50$t, 3$t), $vec3!(21$t | 4$t, 23$t | 50$t, 25$t | 3$t));
                assert_eq!($vec4!(21$t, 23$t, 25$t, 27$t) | $vec4!(4$t, 50$t, 3$t, 10$t), $vec4!(21$t | 4$t, 23$t | 50$t, 25$t | 3$t, 27$t | 10$t));

                assert_eq!($vec2!(21$t, 23$t) ^ $vec2!(4$t, 50$t), $vec2!(21$t ^ 4$t, 23$t ^ 50$t));
                assert_eq!($vec3!(21$t, 23$t, 25$t) ^ $vec3!(4$t, 50$t, 3$t), $vec3!(21$t ^ 4$t, 23$t ^ 50$t, 25$t ^ 3$t));
                assert_eq!($vec4!(21$t, 23$t, 25$t, 27$t) ^ $vec4!(4$t, 50$t, 3$t, 10$t), $vec4!(21$t ^ 4$t, 23$t ^ 50$t, 25$t ^ 3$t, 27$t ^ 10$t));

                assert_eq!($Vec2::ZERO, $vec2!(0$t, 0$t));
                assert_eq!($Vec3::ZERO, $vec3!(0$t, 0$t, 0$t));
                assert_eq!($Vec4::ZERO, $vec4!(0$t, 0$t, 0$t, 0$t));

                assert_eq!($Vec2::ONE, $vec2!(1$t, 1$t));
                assert_eq!($Vec3::ONE, $vec3!(1$t, 1$t, 1$t));
                assert_eq!($Vec4::ONE, $vec4!(1$t, 1$t, 1$t, 1$t));
                
                assert_eq!($Vec2::X, $vec2!(1$t, 0$t));
                assert_eq!($Vec3::Y, $vec3!(0$t, 1$t, 0$t));
                assert_eq!($Vec4::Z, $vec4!(0$t, 0$t, 1$t, 0$t));

                #[cfg(feature = "right")]
                {
                    use ggmath::right::*;

                    assert_eq!($Vec2::<$t>::RIGHT, $Vec2::<$t>::X);
                    assert_eq!($Vec3::<$t>::RIGHT, $Vec3::<$t>::X);
                    assert_eq!($Vec4::<$t>::RIGHT, $Vec4::<$t>::X);
                }

                #[cfg(feature = "left")]
                {
                    use ggmath::left::*;

                    assert_eq!($Vec2::<$t>::LEFT, $Vec2::<$t>::X);
                    assert_eq!($Vec3::<$t>::LEFT, $Vec3::<$t>::X);
                    assert_eq!($Vec4::<$t>::LEFT, $Vec4::<$t>::X);
                }

                #[cfg(feature = "backwards")]
                {
                    use ggmath::backwards::*;

                    assert_eq!($Vec3::<$t>::BACKWARDS, $Vec3::<$t>::Z);
                    assert_eq!($Vec4::<$t>::BACKWARDS, $Vec4::<$t>::Z);
                }
            }

            macro_rules! test_$(s.snakecase())_binop_edgecase {
                ( $$(#[$$attr:meta])* $$vec2_fn:ident, $$vec3_fn:ident, $$vec4_fn:ident: $$op:tt for $$lhs:expr, $$rhs:expr) => {
                    #[test]
                    $$(#[$$attr])*
                    fn $$vec2_fn() {
                        assert_eq!($vec2!(1$t, $$lhs) $$op $vec2!(1$t, $$rhs), $vec2!(1$t $$op 1$t, $$lhs $$op $$rhs));
                    }
                    
                    #[test]
                    $$(#[$$attr])*
                    fn $$vec3_fn() {
                        assert_eq!($vec3!(1$t, $$lhs, 1$t) $$op $vec3!(1$t, $$rhs, 1$t), $vec3!(1$t $$op 1$t, $$lhs $$op $$rhs, 1$t $$op 1$t));
                    }
                    
                    
                    #[test]
                    $$(#[$$attr])*
                    fn $$vec4_fn() {
                        assert_eq!($vec4!(1$t, $$lhs, 1$t, 1$t) $$op $vec4!(1$t, $$rhs, 1$t, 1$t), $vec4!(1$t $$op 1$t, $$lhs $$op $$rhs, 1$t $$op 1$t, 1$t $$op 1$t));
                    }
                }
            }

            test_$(s.snakecase())_binop_edgecase! {
                #[cfg_attr(debug_assertions, should_panic)]
                test_$(t.lowercase_prefix())vec2$(s.lowercase_postfix())_add_overflow,
                test_$(t.lowercase_prefix())vec3$(s.lowercase_postfix())_add_overflow,
                test_$(t.lowercase_prefix())vec4$(s.lowercase_postfix())_add_overflow:

                + for $t::MAX, 3
            }
            test_$(s.snakecase())_binop_edgecase! {
                #[cfg_attr(debug_assertions, should_panic)]
                test_$(t.lowercase_prefix())vec2$(s.lowercase_postfix())_add_exact_overflow,
                test_$(t.lowercase_prefix())vec3$(s.lowercase_postfix())_add_exact_overflow,
                test_$(t.lowercase_prefix())vec4$(s.lowercase_postfix())_add_exact_overflow:

                + for $t::MAX, 1
            }

            test_$(s.snakecase())_binop_edgecase! {
                #[cfg_attr(debug_assertions, should_panic)]
                test_$(t.lowercase_prefix())vec2$(s.lowercase_postfix())_sub_overflow,
                test_$(t.lowercase_prefix())vec3$(s.lowercase_postfix())_sub_overflow,
                test_$(t.lowercase_prefix())vec4$(s.lowercase_postfix())_sub_overflow:

                - for $t::MIN, 3
            }
            test_$(s.snakecase())_binop_edgecase! {
                #[cfg_attr(debug_assertions, should_panic)]
                test_$(t.lowercase_prefix())vec2$(s.lowercase_postfix())_sub_exact_overflow,
                test_$(t.lowercase_prefix())vec3$(s.lowercase_postfix())_sub_exact_overflow,
                test_$(t.lowercase_prefix())vec4$(s.lowercase_postfix())_sub_exact_overflow:

                - for $t::MIN, 1
            }

            test_$(s.snakecase())_binop_edgecase! {
                #[cfg_attr(debug_assertions, should_panic)]
                test_$(t.lowercase_prefix())vec2$(s.lowercase_postfix())_mul_overflow,
                test_$(t.lowercase_prefix())vec3$(s.lowercase_postfix())_mul_overflow,
                test_$(t.lowercase_prefix())vec4$(s.lowercase_postfix())_mul_overflow:

                * for $t::MAX, 3
            }

            test_$(s.snakecase())_binop_edgecase! {
                #[should_panic]
                test_$(t.lowercase_prefix())vec2$(s.lowercase_postfix())_div_by_zero,
                test_$(t.lowercase_prefix())vec3$(s.lowercase_postfix())_div_by_zero,
                test_$(t.lowercase_prefix())vec4$(s.lowercase_postfix())_div_by_zero:

                / for 5$t, 0$t
            }

            test_$(s.snakecase())_binop_edgecase! {
                #[should_panic]
                test_$(t.lowercase_prefix())vec2$(s.lowercase_postfix())_rem_by_zero,
                test_$(t.lowercase_prefix())vec3$(s.lowercase_postfix())_rem_by_zero,
                test_$(t.lowercase_prefix())vec4$(s.lowercase_postfix())_rem_by_zero:

                % for 5$t, 0$t
            }
        )
    });
}
