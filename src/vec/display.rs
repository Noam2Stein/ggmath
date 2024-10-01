use std::fmt::{self, Display, Formatter};

use super::*;

impl<T: Element> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.cget::<0>(), self.cget::<1>())
    }
}
impl<T: Element> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.cget::<0>(),
            self.cget::<1>(),
            self.cget::<2>()
        )
    }
}
impl<T: Element> Display for Vec4<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.cget::<0>(),
            self.cget::<1>(),
            self.cget::<2>(),
            self.cget::<3>()
        )
    }
}
