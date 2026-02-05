use crate::{Alignment, F32VectorBackend, Length, ScalarBackend, SupportedLength};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for f32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> F32VectorBackend<N, A> for f32 where Length<N>: SupportedLength {}
