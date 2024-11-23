use super::*;

ggmath_proc_macros::vector_interface!(
    ScalarRound: Scalar {
        fn floor(self) -> Self;
        fn ceil(self) -> Self;
        fn round(self) -> Self;
        fn trunc(self) -> Self;
    }

    pub impl:

    fn floor(self) -> Self {
        self.map(|c| c.floor())
    }
    fn ceil(self) -> Self {
        self.map(|c| c.ceil())
    }
    fn round(self) -> Self {
        self.map(|c| c.round())
    }
    fn trunc(self) -> Self {
        self.map(|c| c.trunc())
    }
);
