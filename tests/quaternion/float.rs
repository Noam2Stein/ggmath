use ggmath::{Alignment, Quaternion, Vector};

use crate::assert_float_eq;

macro_rules! test {
    ($T:ident, $test_t:ident) => {
        pub fn $test_t<A: Alignment>() {
            assert!(size_of::<Quaternion<$T, A>>() == size_of::<Vector<4, $T, A>>());
            assert!(align_of::<Quaternion<$T, A>>() == align_of::<Vector<4, $T, A>>());

            assert_eq!(
                Quaternion::<$T, A>::ZERO,
                Quaternion::<$T, A>::new(0.0, 0.0, 0.0, 0.0)
            );
            assert_eq!(
                Quaternion::<$T, A>::IDENTITY,
                Quaternion::<$T, A>::new(0.0, 0.0, 0.0, 1.0)
            );
            assert_float_eq!(
                Quaternion::<$T, A>::NAN,
                Quaternion::<$T, A>::new($T::NAN, $T::NAN, $T::NAN, $T::NAN)
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0).to_array(),
                [1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::from_array([1.0, 2.0, 3.0, 4.0]).to_array(),
                [1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::from_vec(Vector::from_array([1.0, 2.0, 3.0, 4.0])).to_array(),
                [1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    .align()
                    .to_array(),
                [1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    .unalign()
                    .to_array(),
                [1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0).as_array_ref(),
                &[1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0).as_array_mut(),
                &mut [1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    .to_vec()
                    .to_array(),
                [1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    .as_vec_ref()
                    .as_array_ref(),
                &[1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    .as_vec_mut()
                    .as_array_mut(),
                &mut [1.0, 2.0, 3.0, 4.0]
            );

            assert_eq!(Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0).x, 1.0);
            assert_eq!(Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0).y, 2.0);
            assert_eq!(Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0).z, 3.0);
            assert_eq!(Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0).w, 4.0);

            assert_eq!(
                format!("{:?}", Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)),
                "Quat(1.0, 2.0, 3.0, 4.0)"
            );

            assert_eq!(
                format!("{}", Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)),
                "(1, 2, 3, 4)"
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    == Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0),
                true
            );
            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    == Quaternion::<$T, A>::new(5.0, 6.0, 7.0, 8.0),
                false
            );
            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    == Quaternion::<$T, A>::new(1.0, 5.0, 3.0, 4.0),
                false
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    != Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0),
                false
            );
            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    != Quaternion::<$T, A>::new(5.0, 6.0, 7.0, 8.0),
                true
            );
            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    != Quaternion::<$T, A>::new(1.0, 5.0, 3.0, 4.0),
                true
            );

            assert_eq!(
                Quaternion::<$T, A>::default(),
                Quaternion::<$T, A>::IDENTITY
            );

            assert_eq!(
                -Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0),
                Quaternion::<$T, A>::new(-1.0, -2.0, -3.0, -4.0)
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    + Quaternion::<$T, A>::new(5.0, 6.0, 7.0, 8.0),
                Quaternion::<$T, A>::new(6.0, 8.0, 10.0, 12.0)
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0)
                    - Quaternion::<$T, A>::new(8.0, 7.0, 6.0, 5.0),
                Quaternion::<$T, A>::new(-7.0, -5.0, -3.0, -1.0)
            );

            assert_eq!(
                Quaternion::<$T, A>::new(1.0, 2.0, 3.0, 4.0) * 2.0,
                Quaternion::<$T, A>::new(2.0, 4.0, 6.0, 8.0)
            );

            let mut q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
            q += Quaternion::<$T, A>::new(5.0, 6.0, 7.0, 8.0);
            assert_eq!(q, Quaternion::<$T, A>::new(6.0, 8.0, 10.0, 12.0));

            let mut q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
            q -= Quaternion::<$T, A>::new(8.0, 7.0, 6.0, 5.0);
            assert_eq!(q, Quaternion::<$T, A>::new(-7.0, -5.0, -3.0, -1.0));

            let mut q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
            q *= 2.0;
            assert_eq!(q, Quaternion::<$T, A>::new(2.0, 4.0, 6.0, 8.0));
        }
    };
}
test!(f32, test_f32);
test!(f64, test_f64);
