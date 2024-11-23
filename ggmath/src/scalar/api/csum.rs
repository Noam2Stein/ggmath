use super::*;

ggmath_proc_macros::vector_interface!(
    ScalarCSum: ScalarAdd<T, Output = T>;

    pub impl:

    fn csum(self) -> T {
        match self.resolve_length() {
            LengthResolvedVector::Vec2(vec) => vec.x() + vec.y(),
            LengthResolvedVector::Vec3(vec) => vec.x() + vec.y() + vec.z(),
            LengthResolvedVector::Vec4(vec) => vec.x() + vec.y() + vec.z() + vec.w(),
        }
    }
);
