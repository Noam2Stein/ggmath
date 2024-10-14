use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <bool>(1):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedBVec2, AlignedBVec4
);
