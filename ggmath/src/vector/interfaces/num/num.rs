use super::*;

ggmath_proc_macros::vec_interface!(
    pub impl:

    ScalarNum:
        Scalar +
        ScalarDefault +
        ScalarAdd<T, Output = T> +
        ScalarSub<T, Output = T> +
        ScalarMul<T, Output = T> +
        ScalarDiv<T, Output = T> +
        ScalarRem<T, Output = T> +
        ScalarAddAssign<T> +
        ScalarSubAssign<T> +
        ScalarMulAssign<T> +
        ScalarDivAssign<T> +
        ScalarRemAssign<T> +
        PartialOrd +
        Num +
        NumCast +
        NumRef +
        NumAssign +
        NumAssignRef,
);
