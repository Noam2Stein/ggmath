use super::*;

ggmath_proc_macros::vector_interface!(
    ScalarCross: ScalarMul<T, Output = T> + ScalarSub<T, Output = T>;

    pub impl for<N: 3>:

    fn cross(self, other: Self) -> Self {
        (self.zxy() * other - self * other.zxy()).zxy()
    }
);
