use crate::{construct::Construct, vec::ScalarVec};

pub trait Scalar: Construct + ScalarVec {}
