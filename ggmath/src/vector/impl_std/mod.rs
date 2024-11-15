use super::*;

mod copy;
mod fmt;
mod index;
mod iter;
mod scalar_ops;

const _: () = {
    fn ensure_vector_is_construct<const N: usize, T: Scalar, A: VecAlignment>()
    where
        ScalarCount<N>: VecLen<N>,
    {
        fn despicable_me<JasonSegel: Construct>() {}

        despicable_me::<Vector<N, T, A>>();
    }
};
