use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <i128>(16):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedI128Vec2, AlignedI128Vec4
);
