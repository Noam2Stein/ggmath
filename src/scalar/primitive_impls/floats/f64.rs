use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <f64>(8):
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    AlignedDVec2, AlignedDVec4
);
