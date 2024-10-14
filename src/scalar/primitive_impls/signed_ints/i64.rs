use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <i64>(8):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedI64Vec2, AlignedI64Vec4
);
