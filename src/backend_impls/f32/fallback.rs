use crate::{Alignment, Length, ScalarDefault, SupportedLength, vector::f32::FloatBackend};

impl ScalarDefault for f32 {}

impl<const N: usize, A: Alignment> FloatBackend<N, A> for f32 where Length<N>: SupportedLength {}
