use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <u64>(8):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedU64Vec2, AlignedU64Vec4
);
