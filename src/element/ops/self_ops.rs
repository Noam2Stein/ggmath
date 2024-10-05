use gomath_proc_macros::{ops_macro, self_ops};

use super::*;

ops_macro!(
    m {
        $(
            pub trait $element_trait: Element + $std_trait<Output: Element> + $vec_trait<2> + $vec_trait<3> + $vec_trait<4> {}
        )*
    }
);
self_ops!(m);
