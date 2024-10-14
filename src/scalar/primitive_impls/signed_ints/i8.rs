use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <i8>(1):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedI8Vec2, AlignedI8Vec4
);
