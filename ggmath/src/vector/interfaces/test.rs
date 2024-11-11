use std::mem::MaybeUninit;
use std::ops::*;

ggmath_proc_macros::vec_interface!(
    impl<Rhs: Scalar> Add<Vector<N, Rhs, A>>:

    ScalarAdd: Scalar + Add<Rhs, Output: Scalar>,

    type Output = Vector<N, <T as Add<Rhs>>::Output, A>;

    fn add(mut self, rhs: Vector<N, Rhs, A>) -> Vector<N, <T as Add<Rhs>>::Output, A>  {
        let mut output_array = unsafe { MaybeUninit::<[<T as Add<Rhs>>::Output; N]>::uninit().assume_init() };
        for i in 0..N {
            output_array[i] = self[i].add(rhs[i])
        }

        Vector::from_array(output_array)
    }
);
