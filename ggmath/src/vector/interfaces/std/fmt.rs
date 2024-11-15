use std::fmt::{Debug, Formatter, Result};

ggmath_proc_macros::vec_interface!(
    ScalarDebug: Scalar + Debug
    where
        T::InnerAlignedVec2: Debug,
        T::InnerAlignedVec4: Debug;

    impl Debug:

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vector({:?})", self.inner)
    }
);
