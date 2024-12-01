use super::*;

macro_rules! const_traits {
    ($($trait_ident:ident($fn_ident:ident, $vector_fn_ident:ident)), * $(,)?) => {$(
        pub trait $trait_ident: Scalar {
            fn $fn_ident() -> Self;

            fn $vector_fn_ident<const N: usize, A: VecAlignment>() -> Vector<N, Self, A>
            where
                ScalarCount<N>: VecLen,
            {
                Vector::splat(Self::$fn_ident())
            }
        }

        impl<const N: usize, T: $trait_ident, A: VecAlignment> Vector<N, T, A>
        where
            ScalarCount<N>: VecLen,
        {
            #[inline(always)]
            pub fn $fn_ident() -> Self {
                T::$vector_fn_ident()
            }
        }
    )*};
}

const_traits!(
    ScalarZero(zero, vector_zero),
    ScalarOne(one, vector_one),
    ScalarTwo(two, vector_two),
);
