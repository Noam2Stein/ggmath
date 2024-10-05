use gomath_proc_macros::{ops_macro, rhs_ops};

use super::*;

ops_macro!(
    m {
        $(
            pub trait $element_trait<Rhs: Element = Self>: Element + $std_trait<Rhs, Output: Element> + $vec_trait<2, Rhs> + $vec_trait<3, Rhs> + $vec_trait<4, Rhs> {}
        )*
    }
);
rhs_ops!(m);
