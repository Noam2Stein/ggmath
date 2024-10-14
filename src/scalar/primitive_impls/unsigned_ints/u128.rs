use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <u128>(16):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedU128Vec2, AlignedU128Vec4
);
