use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <u32>(4):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedUVec2, AlignedUVec4
);
