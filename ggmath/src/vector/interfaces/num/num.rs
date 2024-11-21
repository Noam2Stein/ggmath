use std::fmt::{Debug, Display};

use super::*;

ggmath_proc_macros::vector_interface!(
    ScalarNum:

    Scalar +

    ScalarDefault +
    ScalarPartialEq<T> +
    ScalarPartialOrd +
    Debug +
    Display +

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

    ScalarAbsDiff<T, Output = T> +

    Num +
    NumCast +
    NumRef +
    NumAssign +
    NumAssignRef;
);
