use ggmath::*;
use repetitive::repetitive;

repetitive! {
    // ints
    @for prim in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize] {
        #[test]
        #[should_panic]
        fn @['test_ prim '_add_panic]() {
            let _ =
                vec3!(@prim::MAX, 2 as @prim, 3 as @prim)
                + vec3!(1 as @prim, 5 as @prim, 6 as @prim);
        }

        #[test]
        #[should_panic]
        fn @['test_ prim '_sub_panic]() {
            let _ =
                vec3!(@prim::MIN, 2 as @prim, 3 as @prim)
                - vec3!(1 as @prim, 5 as @prim, 6 as @prim);
        }

        #[test]
        #[should_panic]
        fn @['test_ prim '_mul_panic]() {
            let _ =
                vec3!(@prim::MAX, 2 as @prim, 3 as @prim)
                * vec3!(2 as @prim, 5 as @prim, 6 as @prim);
        }

        #[test]
        #[should_panic]
        fn @['test_ prim '_div_panic]() {
            let _ =
                vec3!(3 as @prim, 2 as @prim, 3 as @prim)
                / vec3!(0 as @prim, 5 as @prim, 6 as @prim);
        }

        #[test]
        #[should_panic]
        fn @['test_ prim '_rem_panic]() {
            let _ =
                vec3!(3 as @prim, 2 as @prim, 3 as @prim)
                % vec3!(0 as @prim, 5 as @prim, 6 as @prim);
        }
    }

    // signed ints
    @for prim in ['i8, 'i16, 'i32, 'i64, 'i128, 'isize] {
        #[test]
        #[should_panic]
        fn @['test_ prim '_neg_panic]() {
            let _ = -vec3!(@prim::MIN, 2 as @prim, 3 as @prim);
        }
    }
}

#[test]
fn test_bool() {
    assert_eq!(vec3!(true, true, false).as_u8(), vec3!(1, 1, 0));
    assert_eq!(vec3!(true, true, false).as_u16(), vec3!(1, 1, 0));
    assert_eq!(vec3!(true, true, false).as_u32(), vec3!(1, 1, 0));
    assert_eq!(vec3!(true, true, false).as_i128(), vec3!(1, 1, 0));
}

#[test]
fn test_num() {
    repetitive! {
        @for prim in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 3 as @prim).min(
                    vec3!(4 as @prim, 2 as @prim, 6 as @prim),
                ),
                vec3!(1 as @prim, 2 as @prim, 3 as @prim),
            );
            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 3 as @prim).max(
                    vec3!(4 as @prim, 2 as @prim, 6 as @prim),
                ),
                vec3!(4 as @prim, 5 as @prim, 6 as @prim),
            );
            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 3 as @prim).clamp(
                    vec3!(4 as @prim, 2 as @prim, 6 as @prim),
                    vec3!(7 as @prim, 8 as @prim, 9 as @prim),
                ),
                vec3!(4 as @prim, 5 as @prim, 6 as @prim),
            );

            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 6 as @prim).abs_diff(
                    vec3!(4 as @prim, 5 as @prim, 3 as @prim),
                ),
                vec3!(3 as @prim, 0 as @prim, 3 as @prim),
            );

            assert_eq!(
                vec3!(1 as @prim, 0 as @prim, 3 as @prim).positive_mask(),
                vec3!(true, false, true),
            );
            assert_eq!(
                vec3!(1 as @prim, 0 as @prim, 3 as @prim).zero_mask(),
                vec3!(false, true, false),
            );

            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 3 as @prim).mag_sq(),
                35 as @prim,
            );

            @if prim == 'f32 || prim == 'f64 {
                assert_eq!(
                    vec3!(1 as @prim, 0 as @prim, 3 as @prim).tri_signum(),
                    vec3!(1 as @prim, 0 as @prim, 1 as @prim),
                );
            } else {
                assert_eq!(
                    vec3!(1 as @prim, 0 as @prim, 3 as @prim).signum(),
                    vec3!(1 as @prim, 0 as @prim, 1 as @prim),
                );
            }

            @for other_prim in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
                @if other_prim != prim {
                    assert_eq!(
                        vec3!(1 as @prim, 5 as @prim, 3 as @prim).@['as_ other_prim](),
                        vec3!(1 as @other_prim, 5 as @other_prim, 3 as @other_prim),
                    );
                }
            }
        }
    }
}

#[test]
fn test_signed_num() {
    repetitive! {
        @for prim in ['i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, -3 as @prim).positive_mask(),
                vec3!(false, true, false),
            );
            assert_eq!(
                vec3!(-1 as @prim, 0 as @prim, -3 as @prim).zero_mask(),
                vec3!(false, true, false),
            );
            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, 0 as @prim).negative_mask(),
                vec3!(true, false, false),
            );

            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, 0 as @prim).bin_signum(),
                vec3!(-1 as @prim, 1 as @prim, 1 as @prim),
            );
            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, 0 as @prim).bin_positive_mask(),
                vec3!(false, true, true),
            );

            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, -0 as @prim).abs(),
                vec3!(1 as @prim, 5 as @prim, 0 as @prim),
            );

            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, -3 as @prim).mag_sq(),
                35 as @prim,
            );

            @for other_prim in ['i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
                @if other_prim != prim {
                    assert_eq!(
                        vec3!(-1 as @prim, 5 as @prim, -3 as @prim).@['as_ other_prim](),
                        vec3!(-1 as @other_prim, 5 as @other_prim, -3 as @other_prim),
                    );
                }
            }
        }
    }
}

repetitive! {
    @for prim in ['f32, 'f64] {
        #[test]
        fn @['test_ prim '_round]() {
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).round(),
                vec4!(6.0, 5.0, -3.0, -4.0),
                "round",
            );
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).floor(),
                vec4!(5.0, 5.0, -4.0, -4.0),
                "floor",
            );
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).ceil(),
                vec4!(6.0, 6.0, -3.0, -3.0),
                "ceil",
            );
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).trunc(),
                vec4!(5.0, 5.0, -3.0, -3.0),
                "trunc",
            );
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).atrunc(),
                vec4!(6.0, 6.0, -4.0, -4.0),
                "atrunc",
            );
        }

        #[test]
        fn @['test_ prim '_trig]() {
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).sin(),
                vec3!((5.6 as @prim).sin(), (5.1 as @prim).sin(), (-3.3 as @prim).sin()),
                "sin",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).cos(),
                vec3!((5.6 as @prim).cos(), (5.1 as @prim).cos(), (-3.3 as @prim).cos()),
                "cos",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).tan(),
                vec3!((5.6 as @prim).tan(), (5.1 as @prim).tan(), (-3.3 as @prim).tan()),
                "tan",
            );
            assert_eq!(
                vec3!(0.5, 0.0, -0.5 as @prim).asin(),
                vec3!((0.5 as @prim).asin(), (0.0 as @prim).asin(), (-0.5 as @prim).asin()),
                "asin",
            );
            assert_eq!(
                vec3!(0.5, 0.0, -0.5 as @prim).acos(),
                vec3!((0.5 as @prim).acos(), (0.0 as @prim).acos(), (-0.5 as @prim).acos()),
                "acos",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).atan(),
                vec3!((5.6 as @prim).atan(), (5.1 as @prim).atan(), (-3.3 as @prim).atan()),
                "atan",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).sinh(),
                vec3!((5.6 as @prim).sinh(), (5.1 as @prim).sinh(), (-3.3 as @prim).sinh()),
                "sinh",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).cosh(),
                vec3!((5.6 as @prim).cosh(), (5.1 as @prim).cosh(), (-3.3 as @prim).cosh()),
                "cosh",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).tanh(),
                vec3!((5.6 as @prim).tanh(), (5.1 as @prim).tanh(), (-3.3 as @prim).tanh()),
                "tanh",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).asinh(),
                vec3!((5.6 as @prim).asinh(), (5.1 as @prim).asinh(), (-3.3 as @prim).asinh()),
                "asinh",
            );
            assert_eq!(
                vec3!(1.5, 2.0, 3.0 as @prim).acosh(),
                vec3!((1.5 as @prim).acosh(), (2.0 as @prim).acosh(), (3.0 as @prim).acosh()),
                "acosh",
            );
            assert_eq!(
                vec3!(0.5, 0.0, -0.5 as @prim).atanh(),
                vec3!((0.5 as @prim).atanh(), (0.0 as @prim).atanh(), (-0.5 as @prim).atanh()),
                "atanh",
            );
        }

        #[test]
        fn @['test_ prim '_lerp]() {
            assert_eq!(
                vec3!(1.0, 2.0, 3.0 as @prim).lerp(vec3!(4.0, 5.0, 6.0 as @prim), -1.0),
                vec3!(1.0, 2.0, 3.0 as @prim),
                "lerp",
            );
            assert_eq!(
                vec3!(1.0, 2.0, 3.0 as @prim).lerp(vec3!(4.0, 5.0, 6.0 as @prim), 0.0),
                vec3!(1.0, 2.0, 3.0 as @prim),
                "lerp",
            );
            assert_eq!(
                vec3!(1.0, 2.0, 3.0 as @prim).lerp(vec3!(4.0, 5.0, 6.0 as @prim), 1.0),
                vec3!(4.0, 5.0, 6.0 as @prim),
                "lerp",
            );
            assert_eq!(
                vec3!(1.0, 2.0, 3.0 as @prim).lerp(vec3!(4.0, 5.0, 6.0 as @prim), 2.0),
                vec3!(4.0, 5.0, 6.0 as @prim),
                "lerp",
            );

            assert_eq!(
                vec3!(1.0, 2.0, 3.0 as @prim).lerp_unclamped(vec3!(4.0, 5.0, 6.0 as @prim), -1.0),
                vec3!(-2.0, -1.0, 0.0 as @prim),
                "lerp_unclamped",
            );
            assert_eq!(
                vec3!(1.0, 2.0, 3.0 as @prim).lerp_unclamped(vec3!(4.0, 5.0, 6.0 as @prim), 0.0),
                vec3!(1.0, 2.0, 3.0 as @prim),
                "lerp_unclamped",
            );
            assert_eq!(
                vec3!(1.0, 2.0, 3.0 as @prim).lerp_unclamped(vec3!(4.0, 5.0, 6.0 as @prim), 1.0),
                vec3!(4.0, 5.0, 6.0 as @prim),
                "lerp_unclamped",
            );
            assert_eq!(
                vec3!(1.0, 2.0, 3.0 as @prim).lerp_unclamped(vec3!(4.0, 5.0, 6.0 as @prim), 2.0),
                vec3!(7.0, 8.0, 9.0 as @prim),
                "lerp_unclamped",
            );
        }

        #[test]
        fn @['test_ prim '_mag]() {
            assert_eq!(vec3!(0.0, 0.0, -0.0 as @prim).mag(), 0.0, "mag");
            assert_eq!(vec3!(0.0, 0.0, -0.0 as @prim).mag_sq(), 0.0, "mag_sq");

            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).mag(),
                vec3!(5.6, 5.1, -3.3 as @prim).mag_sq().sqrt(),
                "mag",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).normalized(),
                vec3!(5.6, 5.1, -3.3 as @prim) / vec3!(5.6, 5.1, -3.3 as @prim).mag(),
                "normalize",
            );
            assert_eq!(
                vec3!(1.0, 0.0, 0.0 as @prim).normalized(),
                vec3!(1.0, 0.0, 0.0 as @prim),
                "normalize",
            );
            assert_eq!(
                vec3!(-1.0, 0.0, 0.0 as @prim).normalized(),
                vec3!(-1.0, 0.0, 0.0 as @prim),
                "normalize",
            );
            assert_eq!(
                vec3!(3.0, 0.0, 0.0 as @prim).with_mag(2.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "with_mag",
            );
            assert_eq!(
                vec3!(-5.0, 0.0, 0.0 as @prim).with_mag(2.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "with_mag",
            );
            assert_eq!(
                vec3!(3.0, 0.0, 0.0 as @prim).with_mag_sq(4.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "with_mag_sq",
            );
            assert_eq!(
                vec3!(-5.0, 0.0, 0.0 as @prim).with_mag_sq(4.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "with_mag_sq",
            );

            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).with_min_mag(2.0),
                vec3!(5.6, 5.1, -3.3 as @prim),
                "with_min_mag",
            );
            assert_eq!(
                vec3!(3.0, 0.0, 0.0 as @prim).with_max_mag(2.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "with_max_mag",
            );
            assert_eq!(
                vec3!(-5.0, 0.0, 0.0 as @prim).with_max_mag(2.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "with_max_mag",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).clamp_mag(7.0, 9.0),
                vec3!(5.6, 5.1, -3.3 as @prim),
                "clamp_mag",
            );
            assert_eq!(
                vec3!(6.0, 0.0, 0.0 as @prim).clamp_mag(2.0, 3.0),
                vec3!(3.0, 0.0, 0.0 as @prim),
                "clamp_mag",
            );
            assert_eq!(
                vec3!(-6.0, 0.0, 0.0 as @prim).clamp_mag(2.0, 3.0),
                vec3!(-3.0, 0.0, 0.0 as @prim),
                "clamp_mag",
            );
            assert_eq!(
                vec3!(1.0, 0.0, 0.0 as @prim).clamp_mag(2.0, 3.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "clamp_mag",
            );
            assert_eq!(
                vec3!(-1.0, 0.0, 0.0 as @prim).clamp_mag(2.0, 3.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "clamp_mag",
            );

            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).with_min_mag_sq(4.0),
                vec3!(5.6, 5.1, -3.3 as @prim),
                "with_min_mag_sq",
            );
            assert_eq!(
                vec3!(3.0, 0.0, 0.0 as @prim).with_max_mag_sq(4.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "with_max_mag_sq",
            );
            assert_eq!(
                vec3!(-5.0, 0.0, 0.0 as @prim).with_max_mag_sq(4.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "with_max_mag_sq",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).clamp_mag_sq(49.0, 81.0),
                vec3!(5.6, 5.1, -3.3 as @prim),
                "clamp_mag_sq",
            );
            assert_eq!(
                vec3!(6.0, 0.0, 0.0 as @prim).clamp_mag_sq(4.0, 9.0),
                vec3!(3.0, 0.0, 0.0 as @prim),
                "clamp_mag_sq",
            );
            assert_eq!(
                vec3!(-6.0, 0.0, 0.0 as @prim).clamp_mag_sq(4.0, 9.0),
                vec3!(-3.0, 0.0, 0.0 as @prim),
                "clamp_mag_sq",
            );
            assert_eq!(
                vec3!(1.0, 0.0, 0.0 as @prim).clamp_mag_sq(4.0, 9.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "clamp_mag_sq",
            );
            assert_eq!(
                vec3!(-1.0, 0.0, 0.0 as @prim).clamp_mag_sq(4.0, 9.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "clamp_mag_sq",
            );
        }

        #[test]
        fn @['test_ prim '_mag_nan]() {
            assert!(vec3!(0.0, 0.0, 0.0 as @prim).normalized().into_iter().all(|x| x.is_nan()));
            assert!(vec3!(0.0, 0.0, 0.0 as @prim).with_mag(1.0).into_iter().all(|x| x.is_nan()));
            assert!(vec3!(0.0, 0.0, 0.0 as @prim).with_mag_sq(1.0).into_iter().all(|x| x.is_nan()));
            assert!(vec3!(0.0, 0.0, 0.0 as @prim).with_min_mag(1.0).into_iter().all(|x| x.is_nan()));
            assert!(vec3!(0.0, 0.0, 0.0 as @prim).clamp_mag(1.0, 2.0).into_iter().all(|x| x.is_nan()));
            assert!(vec3!(0.0, 0.0, 0.0 as @prim).with_min_mag_sq(1.0).into_iter().all(|x| x.is_nan()));
            assert!(vec3!(0.0, 0.0, 0.0 as @prim).clamp_mag_sq(1.0, 2.0).into_iter().all(|x| x.is_nan()));
        }
    }
}
