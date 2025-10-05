use genco::{lang::rust::Tokens, quote};

use crate::iter::{Length, PrimitiveFloat, Simdness, Vector, VectorInfo};

pub fn push(t: PrimitiveFloat, output: &mut Tokens) {
    output.extend(quote! {
        fn approx_eq(a: $t, b: $t) -> bool {
            if a.is_nan() && b.is_nan() {
                true
            } else if a.is_infinite() && b.is_infinite() {
                a.is_sign_positive() == b.is_sign_positive()
            } else {
                (a - b).abs() < 0.1
            }
        }

        fn approx_vec_eq<const N: usize, S: Simdness>(a: Vector<N, $t, S>, b: Vector<N, $t, S>) -> bool
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

    output.extend(quote! {$(
        for vec in Vector::test_iter(t) join($['\n']) =>

        $(let VectorInfo { n, s, alias: ref vec_alias, r#macro: ref vec_macro, name_snakecase: ref vec_name, .. } = vec.info())

        $(let values = (0..n.as_usize()).map(|i| (i as f64) * 1.3).collect::<Vec<f64>>())
        $(let values_str = &values.iter().map(|&x| float_lit(x, t)).collect::<Vec<String>>().join(", "))

        $(let values2 = (0..n.as_usize()).map(|i| ((i + n.as_usize()) as f64) * 5.4).collect::<Vec<f64>>())
        $(let values2_str = &values2.iter().map(|&x| float_lit(x, t)).collect::<Vec<String>>().join(", "))

        $(let values2_with_nan = (0..n.as_usize()).map(|i| if i == 1 { f64::NAN } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_nan_str = &values2_with_nan.iter().map(|&x| float_lit(x, t)).collect::<Vec<String>>().join(", "))

        $(let values2_with_inf = (0..n.as_usize()).map(|i| if i == 1 { f64::INFINITY } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_inf_str = &values2_with_inf.iter().map(|&x| float_lit(x, t)).collect::<Vec<String>>().join(", "))

        $(let values2_with_zero = (0..n.as_usize()).map(|i| if i == 1 { 0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_zero_str = &values2_with_zero.iter().map(|&x| float_lit(x, t)).collect::<Vec<String>>().join(", "))

        $(let values2_with_neg_zero = (0..n.as_usize()).map(|i| if i == 1 { -0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_neg_zero_str = &values2_with_neg_zero.iter().map(|&x| float_lit(x, t)).collect::<Vec<String>>().join(", "))
        
        $(let values2_with_zero_and_neg_zero = (0..n.as_usize()).map(|i| if i == 0 { -0.0 } else if i == 1 { 0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_zero_and_neg_zero_str = &values2_with_zero_and_neg_zero.iter().map(|&x| float_lit(x, t)).collect::<Vec<String>>().join(", "))

        $("// The following code is generated for all float primitives")

        #[test]
        fn test_$(vec_name)_neg() {
            assert_approx_vec_eq!(
                -$vec_macro!($values_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values[i], t))))
            );
            assert_approx_vec_eq!(
                -$vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values2_with_nan[i], t))))
            );
        }

        #[test]
        fn test_$(vec_name)_add() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) + $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) + $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2_with_nan[i], t))))
            );
        }

        #[test]
        fn test_$(vec_name)_sub() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) - $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] - values2[i], t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) - $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] - values2_with_nan[i], t))))
            );
        }

        #[test]
        fn test_$(vec_name)_mul() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) * $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] * values2[i], t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) * $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] * values2_with_nan[i], t))))
            );
        }

        #[test]
        fn test_$(vec_name)_div() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) / $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values2[i], t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) / $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values2_with_nan[i], t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) / $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i] * (i as f64 - 1.0), t)))),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / (values2[i] * (i as f64 - 1.0)), t))))
            );
        }

        #[test]
        fn test_$(vec_name)_rem() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str) % $vec_macro!($values2_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] % values2[i], t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str) % $vec_macro!($values2_with_nan_str),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] % values2_with_nan[i], t))))
            );
        }

        $(
            if n == Length::Four && s == Simdness::NonSimd =>

            #[test]
            fn test_$(vec_name)_add_assign() {
                let mut vec = $vec_macro!($values_str);
                vec += $vec_macro!($values2_str);
                assert_approx_vec_eq!(
                    vec,
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], t))))
                );
            }

            #[test]
            fn test_$(vec_name)_neg_ref() {
                assert_approx_vec_eq!(
                    -&$vec_macro!($values_str),
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values[i], t))))
                );
            }

            #[test]
            fn test_$(vec_name)_add_ref() {
                assert_approx_vec_eq!(
                    $vec_macro!($values_str) + &$vec_macro!($values2_str),
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], t))))
                );
                assert_approx_vec_eq!(
                    &$vec_macro!($values_str) + $vec_macro!($values2_str),
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], t))))
                );
                assert_approx_vec_eq!(
                    &$vec_macro!($values_str) + &$vec_macro!($values2_str),
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], t))))
                );
            }

            #[test]
            fn test_$(vec_name)_add_assign_ref() {
                let mut vec = $vec_macro!($values_str);
                vec += &$vec_macro!($values2_str);
                assert_approx_vec_eq!(
                    vec,
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], t))))
                );
            }

            #[test]
            fn test_$(vec_name)_add_scalar() {
                assert_approx_vec_eq!(
                    $vec_macro!($values_str) + 1.0,
                    $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + 1.0, t))))
                );
            }
        )

        #[test]
        fn test_$(vec_name)_sum() {
            assert_approx_eq!(
                $vec_macro!($values_str).sum(),
                $(float_lit(values.iter().sum::<f64>(), t))
            );
        }

        #[test]
        fn test_$(vec_name)_product() {
            assert_approx_eq!(
                $vec_macro!($values2_str).product(),
                $(float_lit(values2.iter().product::<f64>(), t))
            );
        }

        #[test]
        fn test_$(vec_name)_mag_sq() {
            assert_approx_eq!(
                $vec_macro!($values_str).mag_sq(),
                $(float_lit(values.iter().map(|&x| x * x).sum::<f64>(), t))
            );
        }

        #[test]
        fn test_$(vec_name)_dot() {
            assert_approx_eq!(
                $vec_macro!($values_str).dot($vec_macro!($values2_str)),
                $(float_lit(values.iter().zip(values2.iter()).map(|(&x, &y)| x * y).sum::<f64>(), t))
            );
        }

        $(
            if n == Length::Two =>

            #[test]
            fn test_$(vec_name)_perp() {
                assert_approx_vec_eq!(
                    $vec_macro!(1.0$t, 0.0$t).perp(),
                    $vec_macro!(0.0$t, 1.0$t),
                );
            }
            
            #[test]
            fn test_$(vec_name)_perp_dot() {
                assert_approx_eq!(
                    $vec_macro!(1.0$t, 2.0$t).perp_dot(
                        $vec_macro!(3.0$t, 4.0$t)
                    ),
                    -2.0,
                );
            }
        )

        $(
            if n == Length::Three =>

            #[test]
            fn test_$(vec_name)_cross() {
                assert_approx_vec_eq!(
                    $vec_macro!(1.0$t, 2.0$t, 3.0$t).cross(
                        $vec_macro!(4.0$t, 5.0$t, 6.0$t)
                    ),
                    $vec_macro!(-3.0$t, 6.0$t, -3$t),
                );
            }
        )

        #[test]
        fn test_$(vec_name)_div_euclid() {
            assert_approx_vec_eq!(
                $vec_macro!($values2_str).div_euclid($vec_macro!($values_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i].div_euclid(values[i]), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).div_euclid($vec_macro!($values2_with_nan_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].div_euclid(values2_with_nan[i]), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).div_euclid($vec_macro!($values2_with_zero_and_neg_zero_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].div_euclid(values2_with_zero_and_neg_zero[i]), t))))
            );
        }
        
        #[test]
        fn test_$(vec_name)_rem_euclid() {
            assert_approx_vec_eq!(
                $vec_macro!($values2_str).rem_euclid($vec_macro!($values_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i].rem_euclid(values[i]), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).rem_euclid($vec_macro!($values2_with_nan_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].rem_euclid(values2_with_nan[i]), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).rem_euclid($vec_macro!($values2_with_zero_and_neg_zero_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].rem_euclid(values2_with_zero_and_neg_zero[i]), t))))
            );
        }

        #[test]
        fn test_$(vec_name)_min() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).min($vec_macro!($values2_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].min(values2[i]), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).min($vec_macro!($values2_with_nan_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].min(values2_with_nan[i]), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_zero_str).min($vec_macro!($values2_with_neg_zero_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero[i].min(values2_with_neg_zero[i]), t))))
            );
        }

        #[test]
        fn test_$(vec_name)_max() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).max($vec_macro!($values2_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].max(values2[i]), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values_str).max($vec_macro!($values2_with_nan_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].max(values2_with_nan[i]), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_zero_str).max($vec_macro!($values2_with_neg_zero_str)),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero[i].max(values2_with_neg_zero[i]), t))))
            );
        }

        #[test]
        fn test_$(vec_name)_min_element() {
            assert_approx_eq!(
                $vec_macro!($values_str).min_element(),
                $(float_lit(values.iter().cloned().reduce(f64::min).unwrap(), t))
            );
            assert_approx_eq!(
                $vec_macro!($values2_with_nan_str).min_element(),
                $(float_lit(values2_with_nan.iter().cloned().reduce(f64::min).unwrap(), t))
            );
            assert_approx_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).min_element(),
                $(float_lit(values2_with_zero_and_neg_zero.iter().cloned().reduce(f64::min).unwrap(), t))
            );
        }
        
        #[test]
        fn test_$(vec_name)_max_element() {
            assert_approx_eq!(
                $vec_macro!($values_str).max_element(),
                $(float_lit(values.iter().cloned().reduce(f64::max).unwrap(), t))
            );
            assert_approx_eq!(
                $vec_macro!($values2_with_nan_str).max_element(),
                $(float_lit(values2_with_nan.iter().cloned().reduce(f64::max).unwrap(), t))
            );
            assert_approx_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).max_element(),
                $(float_lit(values2_with_zero_and_neg_zero.iter().cloned().reduce(f64::max).unwrap(), t))
            );
        }

        #[test]
        fn test_$(vec_name)_signum() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).signum(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].signum(), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).signum(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i].signum(), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).signum(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero_and_neg_zero[i].signum(), t))))
            );
        }

        #[test]
        fn test_$(vec_name)_abs() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).abs(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].abs(), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).abs(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i].abs(), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_zero_and_neg_zero_str).abs(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero_and_neg_zero[i].abs(), t))))
            );
        }

        #[test]
        fn test_$(vec_name)_positive_sign_mask() {
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
        fn test_$(vec_name)_negative_sign_mask() {
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
        fn test_$(vec_name)_nan_mask() {
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
        fn test_$(vec_name)_finite_mask() {
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
        fn test_$(vec_name)_is_nan() {
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
        fn test_$(vec_name)_is_finite() {
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
        fn test_$(vec_name)_normalize() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).normalize(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).normalize(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i] / values2_with_nan.iter().map(|&x| x * x).sum::<f64>().sqrt(), t))))
            );
            assert_approx_vec_eq!(
                $vec_alias::<$t>::ZERO.normalize(),
                $vec_macro!($(for _ in 0..n.as_usize() join(, ) => $(float_lit(0.0 / 0.0, t))))
            );
        }

        #[test]
        fn test_$(vec_name)_checked_normalize() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).checked_normalize().unwrap(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), t))))
            );
            assert_eq!(
                $vec_macro!($values2_with_nan_str).checked_normalize(),
                None
            );
            assert_eq!(
                $vec_alias::<$t>::ZERO.checked_normalize(),
                None
            );
        }

        #[test]
        fn test_$(vec_name)_normalize_or() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).normalize_or($vec_alias::<$t>::MAX),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).normalize_or($vec_alias::<$t>::MAX),
                $vec_alias::<$t>::MAX
            );
            assert_approx_vec_eq!(
                $vec_alias::<$t>::ZERO.normalize_or($vec_alias::<$t>::MAX),
                $vec_alias::<$t>::MAX
            );
        }

        #[test]
        fn test_$(vec_name)_normalize_or_zero() {
            assert_approx_vec_eq!(
                $vec_macro!($values_str).normalize_or_zero(),
                $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), t))))
            );
            assert_approx_vec_eq!(
                $vec_macro!($values2_with_nan_str).normalize_or_zero(),
                $vec_alias::<$t>::ZERO
            );
            assert_approx_vec_eq!(
                $vec_alias::<$t>::ZERO.normalize_or_zero(),
                $vec_alias::<$t>::ZERO
            );
        }
    )});
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