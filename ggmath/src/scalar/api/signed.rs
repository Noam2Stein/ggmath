use super::*;

ggmath_proc_macros::vector_interface!(
    ScalarSigned: ScalarNeg {
        #[inline(always)]
        fn is_positive(self) -> bool {
            !self.is_negative()
        }
        fn is_negative(self) -> bool;

        fn signum(self) -> Self;

        fn abs(self) -> Self;
    }

    pub impl:

    fn c_are_positive(self) -> Vector<N, bool, A> {
        self.map(|c| c.is_positive())
    }
    fn c_are_negative(self) -> Vector<N, bool, A> {
        self.map(|c| c.is_negative())
    }

    fn signum(self) -> Self {
        self.map(|c| c.signum())
    }

    fn abs(self) -> Self {
        self.map(|c| c.abs())
    }
);
