use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <i16>(2):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedI16Vec2, AlignedI16Vec4
);
