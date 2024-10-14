use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <f32>(4):
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    AlignedFVec2, AlignedFVec4
);
