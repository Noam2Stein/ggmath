use super::*;

mod copy;
mod debug;
mod display;
mod partial_eq;

const _: () = {
    fn ensure_matrix_is_construct<
        const C: usize,
        const R: usize,
        T: Scalar,
        A: VecAlignment,
        M: MatrixMajorAxis,
    >()
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        fn megamind<DavidCross: Construct>() {}

        megamind::<Matrix<C, R, T, A, M>>();
    }
};
