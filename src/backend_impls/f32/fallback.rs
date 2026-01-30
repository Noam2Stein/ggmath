use crate::{Alignment, F32VectorBackend, Length, ScalarDefault, SupportedLength};

impl ScalarDefault for f32 {}

impl<const N: usize, A: Alignment> F32VectorBackend<N, A> for f32 where Length<N>: SupportedLength {}
