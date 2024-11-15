use std::fmt::{Debug, Formatter, Result};

ggmath_proc_macros::vec_interface!(
    ScalarDebug: Scalar<InnerAlignedVec2: Debug, InnerAlignedVec4: Debug> + Debug;

    impl Debug:

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vector({:?})", self.inner)
    }
);
