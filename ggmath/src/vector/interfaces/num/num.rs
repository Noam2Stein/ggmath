use super::*;

ggmath_proc_macros::vec_interface!(
    pub impl:

    ScalarNum:
        Scalar +
        ScalarAdd<T, Output = T> +
        ScalarSub<T, Output = T> +
        ScalarMul<T, Output = T> +
        ScalarDiv<T, Output = T> +
        ScalarRem<T, Output = T> +
        Num,
);
