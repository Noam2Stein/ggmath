use gomath_proc_macros::{assign_ops, ops_macro};

use super::*;

ops_macro!(
    m {
        $(
            pub trait $element_trait<Rhs: Element = Self>: Element + $std_trait<Rhs> + $vec_trait<2, Rhs> + $vec_trait<3, Rhs> + $vec_trait<4, Rhs> {}
        )*
    }
);
assign_ops!(m);
