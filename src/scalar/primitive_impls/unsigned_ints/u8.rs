use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <u8>(1):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedU8Vec2, AlignedU8Vec4
);
