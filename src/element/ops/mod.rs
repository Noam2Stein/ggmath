use std::ops::*;

use super::*;

pub trait ElementAdd<Rhs: Element>: Element + Add<Rhs, Output: Element> {
    fn vec2_add(vec: Self::InnerVec2, rhs: Self::InnerVec2);
}
