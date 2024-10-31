use std::ops::*;

ggmath_proc_macros::self_ops!(
    $(
        ggmath_proc_macros::vec_interface!(
            $scalar_trait: Scalar + $std_trait,
        );
    )*
);
