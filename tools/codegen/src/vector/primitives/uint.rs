use genco::lang::rust::Tokens;

use crate::vector::primitives::{PrimitiveSrcMod, PrimitiveUint};

pub fn push_src(_primitive: PrimitiveUint, _output: &mut PrimitiveSrcMod) {}

pub fn push_tests(_n: usize, _primitive: &str, _is_simd: bool, _tests: &mut Vec<Tokens>) {}
