use genco::quote;

use crate::{iter::{Length, PrimitiveFloat, Simdness}, testdir::vector::PrimitiveTestMod};

pub fn push_tests(primitive: PrimitiveFloat, output: &mut PrimitiveTestMod) {
    output.items.push(quote! {
        fn approx_eq(a: $primitive, b: $primitive) -> bool {
            if a.is_nan() && b.is_nan() {
                true
            } else if a.is_infinite() && b.is_infinite() {
                a.is_sign_positive() == b.is_sign_positive()
            } else {
                (a - b).abs() < 0.1
            }
        }

        fn approx_vec_eq<const N: usize, S: Simdness>(a: Vector<N, $primitive, S>, b: Vector<N, $primitive, S>) -> bool
        where
            Usize<N>: VecLen,
        {
            (0..N).all(|i| approx_eq(a.index(i), b.index(i)))
        }

        macro_rules! assert_approx_eq {
            ($$a:expr, $$b:expr $$(,)?) => {
                let a = $$a;
                let b = $$b;
                
                if !approx_eq(a, b) {
                    panic!("approx_eq failed: {a:?} != {b:?}");
                }
            };
        }

        macro_rules! assert_approx_vec_eq {
            ($$a:expr, $$b:expr $$(,)?) => {
                let a = $$a;
                let b = $$b;
                
                if !approx_vec_eq(a, b) {
                    panic!("approx_vec_eq failed: {a:?} != {b:?}");
                }
            };
        }
    });

    output.push_for_all_vectors(primitive, |n, s| quote! {
        $(let vec_label = &format!("{t_prefix}vec{n}{s_postfix}", t_prefix = primitive.prefix_lowercase(), s_postfix = s.postfix_lowercase()))
        $(let vec_macro = &format!("vec{n}{s_postfix}", s_postfix = s.postfix_lowercase()))
        $(let vec_type = &format!("Vec{n}{s_postfix}", s_postfix = s.postfix_uppercase()))

        $(let values = (0..n.as_usize()).map(|i| (i as f64) * 1.3).collect::<Vec<f64>>())
        $(let values_str = &values.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2 = (0..n.as_usize()).map(|i| ((i + n.as_usize()) as f64) * 5.4).collect::<Vec<f64>>())
        $(let values2_str = &values2.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2_with_nan = (0..n.as_usize()).map(|i| if i == 1 { f64::NAN } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_nan_str = &values2_with_nan.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2_with_inf = (0..n.as_usize()).map(|i| if i == 1 { f64::INFINITY } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_inf_str = &values2_with_inf.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2_with_zero = (0..n.as_usize()).map(|i| if i == 1 { 0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_zero_str = &values2_with_zero.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2_with_neg_zero = (0..n.as_usize()).map(|i| if i == 1 { -0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_neg_zero_str = &values2_with_neg_zero.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))
        
        $(let values2_with_zero_and_neg_zero = (0..n.as_usize()).map(|i| if i == 0 { -0.0 } else if i == 1 { 0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_zero_and_neg_zero_str = &values2_with_zero_and_neg_zero.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $("// The following code is generated for all float primitives")

        #[test]
        fn test_$(vec_label)_neg() {
            assert_approx_vec_eq!(
                -$vec_macro!($values_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values[i], primitive))))
            );
            assert_approx_vec_eq!(
                -$vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values2_with_nan[i], primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_add() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) + $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) + $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2_with_nan[i], primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_sub() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) - $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] - values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) - $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] - values2_with_nan[i], primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_mul() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) * $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] * values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) * $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] * values2_with_nan[i], primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_div() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) / $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) / $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values2_with_nan[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) / $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i] * (i as f64 - 1.0), primitive)))),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / (values2[i] * (i as f64 - 1.0)), primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_rem() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) % $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] % values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) % $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] % values2_with_nan[i], primitive))))
            );
        }

        $(
            if n == Length::N4 && s == Simdness::NonSimd =>

            #[test]
            fn test_$(vec_label)_add_assign() {
                let mut vec = $vec_macro!($values_str);
                vec += $vec_macro!($values2_str);
                assert_approx_vec_eq!(
                    vec,
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
            }

            #[test]
            fn test_$(vec_label)_neg_ref() {
                assert_approx_vec_eq!(
                    -&$vec_macro!($values_str),
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values[i], primitive))))
                );
            }

            #[test]
            fn test_$(vec_label)_add_ref() {
                assert_approx_vec_eq!(
                    $vec_macro!($values_str) + &$vec_macro!($values2_str),
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
                assert_approx_vec_eq!(
                    &$vec_macro!($values_str) + $vec_macro!($values2_str),
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
                assert_approx_vec_eq!(
                    &$vec_macro!($values_str) + &$vec_macro!($values2_str),
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
            }

            #[test]
            fn test_$(vec_label)_add_assign_ref() {
                let mut vec = $vec_macro!($values_str);
                vec += &$vec_macro!($values2_str);
                assert_approx_vec_eq!(
                    vec,
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
            }

            #[test]
            fn test_$(vec_label)_add_scalar() {
                assert_approx_vec_eq!(
                    $vec_macro!($values_str) + 1.0,
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + 1.0, primitive))))
                );
            }
        )

        #[test]
        fn test_$(vec_label)_sum() {
            assert_approx_eq!(
                $vec_macro!($values_str).sum(),
                $(float_lit(values.iter().sum::<f64>(), primitive))
            );
        }

        #[test]
        fn test_$(vec_label)_product() {
            assert_approx_eq!(
                $vec_macro!($values2_str).product(),
                $(float_lit(values2.iter().product::<f64>(), primitive))
            );
        }

        #[test]
        fn test_$(vec_label)_mag_sq() {
            assert_approx_eq!(
                $vec_macro!($values_str).mag_sq(),
                $(float_lit(values.iter().map(|&x| x * x).sum::<f64>(), primitive))
            );
        }

        #[test]
        fn test_$(vec_label)_dot() {
            assert_approx_eq!(
                $vec_macro!($values_str).dot($vec_macro!($values2_str)),
                $(float_lit(values.iter().zip(values2.iter()).map(|(&x, &y)| x * y).sum::<f64>(), primitive))
            );
        }

        $(
            if n == Length::N2 =>

            #[test]
            fn test_$(vec_label)_perp() {
                assert_approx_vec_eq!(
                    $vec_macro!(1.0$primitive, 0.0$primitive).perp(),
                    $vec_macro!(0.0$primitive, 1.0$primitive),
                );
            }
            
            fn test_$(vec_label)_perp_dot() {
                assert_approx_eq!(
                    $vec_macro!(1.0$primitive, 2.0$primitive).perp_dot(
                        $vec_macro!(3.0$primitive, 4.0$primitive)
                    ),
                    -2.0,
                );
            }
        )

        $(
            if n == Length::N3 =>

            #[test]
            fn test_$(vec_label)_cross() {
                assert_approx_vec_eq!(
                    $vec_macro!(1.0$primitive, 2.0$primitive, 3.0$primitive).cross(
                        $vec_macro!(4.0$primitive, 5.0$primitive, 6.0$primitive)
                    ),
                    $vec_macro!(-3.0$primitive, 6.0$primitive, -3$primitive),
                );
            }
        )

        #[test]
        fn test_$(vec_label)_div_euclid() {
            assert_approx_vec_eq!(
                $vec_macro!($values2_str).div_euclid($vec_macro!($values_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i].div_euclid(values[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).div_euclid($vec_macro!($values2_with_nan_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].div_euclid(values2_with_nan[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).div_euclid($vec_macro!($values2_with_zero_and_neg_zero_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].div_euclid(values2_with_zero_and_neg_zero[i]), primitive))))
            );
        }
        
        #[test]
        fn test_$(vec_label)_rem_euclid() {
            assert_approx_vec_eq!(
                $vec_macro!($values2_str).rem_euclid($vec_macro!($values_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i].rem_euclid(values[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).rem_euclid($vec_macro!($values2_with_nan_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].rem_euclid(values2_with_nan[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).rem_euclid($vec_macro!($values2_with_zero_and_neg_zero_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].rem_euclid(values2_with_zero_and_neg_zero[i]), primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_min() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).min($vec_macro!($values2_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].min(values2[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).min($vec_macro!($values2_with_nan_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].min(values2_with_nan[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_zero_str).min($vec_macro!($values2_with_neg_zero_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero[i].min(values2_with_neg_zero[i]), primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_max() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).max($vec_macro!($values2_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].max(values2[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).max($vec_macro!($values2_with_nan_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].max(values2_with_nan[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_zero_str).max($vec_macro!($values2_with_neg_zero_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero[i].max(values2_with_neg_zero[i]), primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_min_element() {
            assert_approx_eq!(
                $vec_macro!($values_str).min_element(),
                $(float_lit(values.iter().cloned().reduce(f64::min).unwrap(), primitive))
            );
            assert_approx_eq!(
                $vec_macro!($values2_with_nan_str).min_element(),
                $(float_lit(values2_with_nan.iter().cloned().reduce(f64::min).unwrap(), primitive))
            );
            assert_approx_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).min_element(),
                $(float_lit(values2_with_zero_and_neg_zero.iter().cloned().reduce(f64::min).unwrap(), primitive))
            );
        }
        
        #[test]
        fn test_$(vec_label)_max_element() {
            assert_approx_eq!(
                $vec_macro!($values_str).max_element(),
                $(float_lit(values.iter().cloned().reduce(f64::max).unwrap(), primitive))
            );
            assert_approx_eq!(
                $vec_macro!($values2_with_nan_str).max_element(),
                $(float_lit(values2_with_nan.iter().cloned().reduce(f64::max).unwrap(), primitive))
            );
            assert_approx_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).max_element(),
                $(float_lit(values2_with_zero_and_neg_zero.iter().cloned().reduce(f64::max).unwrap(), primitive))
            );
        }

        #[test]
        fn test_$(vec_label)_signum() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).signum(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].signum(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).signum(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i].signum(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).signum(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero_and_neg_zero[i].signum(), primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_abs() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).abs(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].abs(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).abs(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i].abs(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).abs(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero_and_neg_zero[i].abs(), primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_positive_sign_mask() {
            assert_eq!(
                $vec_macro!($values_str).positive_sign_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values[i].is_sign_positive().to_string())))
            );
            assert_eq!(
                $vec_macro!($values2_with_nan_str).positive_sign_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values2_with_nan[i].is_sign_positive().to_string())))
            );
            assert_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).positive_sign_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values2_with_zero_and_neg_zero[i].is_sign_positive().to_string())))
            );
        }

        #[test]
        fn test_$(vec_label)_negative_sign_mask() {
            assert_eq!(
                $vec_macro!($values_str).negative_sign_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values[i].is_sign_negative().to_string())))
            );
            assert_eq!(
                $vec_macro!($values2_with_nan_str).negative_sign_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values2_with_nan[i].is_sign_negative().to_string())))
            );
            assert_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).negative_sign_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values2_with_zero_and_neg_zero[i].is_sign_negative().to_string())))
            );
        }

        #[test]
        fn test_$(vec_label)_nan_mask() {
            assert_eq!(
                $vec_macro!($values_str).nan_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values[i].is_nan().to_string())))
            );
            assert_eq!(
                $vec_macro!($values2_with_nan_str).nan_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values2_with_nan[i].is_nan().to_string())))
            );
            assert_eq!(
                $vec_macro!($values2_with_inf_str).nan_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values2_with_inf[i].is_nan().to_string())))
            );
        }
        
        #[test]
        fn test_$(vec_label)_finite_mask() {
            assert_eq!(
                $vec_macro!($values_str).finite_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values[i].is_finite().to_string())))
            );
            assert_eq!(
                $vec_macro!($values2_with_nan_str).finite_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values2_with_nan[i].is_finite().to_string())))
            );
            assert_eq!(
                $vec_macro!($values2_with_inf_str).finite_mask(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(values2_with_inf[i].is_finite().to_string())))
            );
        }

        #[test]
        fn test_$(vec_label)_is_nan() {
            assert_eq!(
                $vec_macro!($values_str).is_nan(),
                $(values.iter().any(|&x| x.is_nan()).to_string())
            );
            assert_eq!(
                $vec_macro!($values2_with_nan_str).is_nan(),
                $(values2_with_nan.iter().any(|&x| x.is_nan()).to_string())
            );
            assert_eq!(
                $vec_macro!($values2_with_inf_str).is_nan(),
                $(values2_with_inf.iter().any(|&x| x.is_nan()).to_string())
            );
        }

        #[test]
        fn test_$(vec_label)_is_finite() {
            assert_eq!(
                $vec_macro!($values_str).is_finite(),
                $(values.iter().all(|&x| x.is_finite()).to_string())
            );
            assert_eq!(
                $vec_macro!($values2_with_nan_str).is_finite(),
                $(values2_with_nan.iter().all(|&x| x.is_finite()).to_string())
            );
            assert_eq!(
                $vec_macro!($values2_with_inf_str).is_finite(),
                $(values2_with_inf.iter().all(|&x| x.is_finite()).to_string())
            );
        }

        #[test]
        fn test_$(vec_label)_normalize() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).normalize(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).normalize(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i] / values2_with_nan.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_type::<$primitive>::ZERO.normalize(),
                $vec_macro!($(for _ in 0..n.as_usize() join(, ) => $(float_lit(0.0 / 0.0, primitive))))
            );
        }

        #[test]
        fn test_$(vec_label)_checked_normalize() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).checked_normalize().unwrap(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_eq!(
                $vec_macro!($values2_with_nan_str).checked_normalize(),
                None
            );
            assert_eq!(
                $vec_type::<$primitive>::ZERO.checked_normalize(),
                None
            );
        }

        #[test]
        fn test_$(vec_label)_normalize_or() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).normalize_or($vec_type::<$primitive>::MAX),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).normalize_or($vec_type::<$primitive>::MAX),
                $vec_type::<$primitive>::MAX
            );
            assert_approx_vec_eq!(
                $vec_type::<$primitive>::ZERO.normalize_or($vec_type::<$primitive>::MAX),
                $vec_type::<$primitive>::MAX
            );
        }

        #[test]
        fn test_$(vec_label)_normalize_or_zero() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).normalize_or_zero(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).normalize_or_zero(),
                $vec_type::<$primitive>::ZERO
            );
            assert_approx_vec_eq!(
                $vec_type::<$primitive>::ZERO.normalize_or_zero(),
                $vec_type::<$primitive>::ZERO
            );
        }
    });
}

fn float_lit(value: f64, primitive: PrimitiveFloat) -> String {
    if value.is_nan() {
        format!("{primitive}::NAN")
    } else if value.is_infinite() {
        if value.is_sign_positive() {
            format!("{primitive}::INFINITY")
        } else {
            format!("{primitive}::NEG_INFINITY")
        }
    } else {
        let mut lit = format!("{value:.4}").trim_end_matches("0").to_string();
        if lit.ends_with(".") {
            lit += "0";
        }

        format!("{}{primitive}", lit)
    }
}