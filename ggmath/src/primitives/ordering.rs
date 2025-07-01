use std::cmp::Ordering;

use super::*;

impl Scalar for Ordering {
    type Vec2Alignment = Align<1>;
    type Vec3Alignment = Align<1>;
    type Vec4Alignment = Align<1>;
}
