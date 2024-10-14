use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <i32>(4):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedIVec2, AlignedIVec4
);
