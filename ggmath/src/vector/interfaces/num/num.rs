use std::fmt::{Debug, Display};

use super::*;

ggmath_proc_macros::vec_interface!(
    ScalarNum:

    Scalar +

    ScalarDefault +
    ScalarPartialEq<T> +
    Debug +
    Display +
    PartialOrd +

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

    Num +
    NumCast +
    NumRef +
    NumAssign +
    NumAssignRef;
);
