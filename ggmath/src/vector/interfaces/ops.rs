use std::mem::MaybeUninit;
use std::ops::*;

ggmath_proc_macros::self_ops!(
    $(ggmath_proc_macros::vec_interface!(
        impl $std_trait:

        $scalar_trait: Scalar + $std_trait<Output: Scalar>,

        type Output = Vector<N, <T as $std_trait>::Output, A>;

        fn $std_fn(self) -> Vector<N, <T as $std_trait>::Output, A> {
            Vector::from_array(self.into_array().map(|c| c.$std_fn()))
        }
    );)*
);
ggmath_proc_macros::rhs_ops!(
    $(ggmath_proc_macros::vec_interface!(
        impl<Rhs: Scalar> $std_trait<Vector<N, Rhs, A>>:

        $scalar_trait: Scalar + $std_trait<Rhs, Output: Scalar>,

        type Output = Vector<N, <T as $std_trait<Rhs>>::Output, A>;

        fn $std_fn(self, rhs: Vector<N, Rhs, A>) -> Vector<N, <T as $std_trait<Rhs>>::Output, A> {
            let mut output_array = unsafe { MaybeUninit::<[<T as $std_trait<Rhs>>::Output; N]>::uninit().assume_init() };
            for i in 0..N {
                output_array[i] = self[i].$std_fn(rhs[i])
            }

            Vector::from_array(output_array)
        }
    );)*
);
ggmath_proc_macros::assign_ops!(
    $(ggmath_proc_macros::vec_interface!(
        impl<Rhs: Scalar> $std_trait<Vector<N, Rhs, A>>:

        $scalar_trait: Scalar + $std_trait<Rhs>,

        fn $std_fn(&mut self, rhs: Vector<N, Rhs, A>) {
            for i in 0..N {
                self[i].$std_fn(rhs[i])
            }
        }
    );)*
);
