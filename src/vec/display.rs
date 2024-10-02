use std::fmt::{self, Display, Formatter};

use super::*;

impl<T: Element> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}
impl<T: Element> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
impl<T: Element> Display for Vec4<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}
