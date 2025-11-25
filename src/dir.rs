/// `RIGHT` and `LEFT` constants where right is positive.
pub mod right {
    use crate::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `RIGHT` constant where right is positive.
    pub trait RightExt {
        /// Points right (right is positive).
        const RIGHT: Self;
    }

    /// `LEFT` constant where right is positive.
    pub trait NegLeftExt {
        /// Points left (right is positive).
        const LEFT: Self;
    }

    impl<T: ScalarOne> RightExt for T {
        const RIGHT: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegLeftExt for T {
        const LEFT: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> RightExt for Vector<2, T, A> {
        const RIGHT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegLeftExt for Vector<2, T, A> {
        const LEFT: Self = Self::NEG_X;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> RightExt for Vector<3, T, A> {
        const RIGHT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegLeftExt for Vector<3, T, A> {
        const LEFT: Self = Self::NEG_X;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> RightExt for Vector<4, T, A> {
        const RIGHT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegLeftExt for Vector<4, T, A> {
        const LEFT: Self = Self::NEG_X;
    }
}

/// `LEFT` and `RIGHT` constants where left is positive.
pub mod left {
    use crate::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `LEFT` constant where left is positive.
    pub trait LeftExt {
        /// Points left (left is positive).
        const LEFT: Self;
    }

    /// `RIGHT` constant where left is positive.
    pub trait NegRightExt {
        /// Points right (left is positive).
        const RIGHT: Self;
    }

    impl<T: ScalarOne> LeftExt for T {
        const LEFT: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegRightExt for T {
        const RIGHT: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> LeftExt for Vector<2, T, A> {
        const LEFT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegRightExt for Vector<2, T, A> {
        const RIGHT: Self = Self::NEG_X;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> LeftExt for Vector<3, T, A> {
        const LEFT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegRightExt for Vector<3, T, A> {
        const RIGHT: Self = Self::NEG_X;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> LeftExt for Vector<4, T, A> {
        const LEFT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegRightExt for Vector<4, T, A> {
        const RIGHT: Self = Self::NEG_X;
    }
}

/// `UP` and `DOWN` constants where up is positive.
pub mod up {
    use crate::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `UP` constant where up is positive.
    pub trait UpExt {
        /// Points up (up is positive).
        const UP: Self;
    }

    /// `DOWN` constant where up is positive.
    pub trait NegDownExt {
        /// Points down (up is positive).
        const DOWN: Self;
    }

    impl<T: ScalarOne> UpExt for T {
        const UP: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegDownExt for T {
        const DOWN: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> UpExt for Vector<2, T, A> {
        const UP: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegDownExt for Vector<2, T, A> {
        const DOWN: Self = Self::NEG_Y;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> UpExt for Vector<3, T, A> {
        const UP: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegDownExt for Vector<3, T, A> {
        const DOWN: Self = Self::NEG_Y;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> UpExt for Vector<4, T, A> {
        const UP: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegDownExt for Vector<4, T, A> {
        const DOWN: Self = Self::NEG_Y;
    }
}

/// `DOWN` and `UP` constants where down is positive.
pub mod down {
    use crate::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `DOWN` constant where down is positive.
    pub trait DownExt {
        /// Points down (down is positive).
        const DOWN: Self;
    }

    /// `UP` constant where down is positive.
    pub trait NegUpExt {
        /// Points up (down is positive).
        const UP: Self;
    }

    impl<T: ScalarOne> DownExt for T {
        const DOWN: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegUpExt for T {
        const UP: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> DownExt for Vector<2, T, A> {
        const DOWN: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegUpExt for Vector<2, T, A> {
        const UP: Self = Self::NEG_Y;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> DownExt for Vector<3, T, A> {
        const DOWN: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegUpExt for Vector<3, T, A> {
        const UP: Self = Self::NEG_Y;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> DownExt for Vector<4, T, A> {
        const DOWN: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegUpExt for Vector<4, T, A> {
        const UP: Self = Self::NEG_Y;
    }
}

/// `FORWARD` and `BACKWARD` constants where forwards is positive.
pub mod forward {
    use crate::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `FORWARD` constant where forwards is positive.
    pub trait ForwardExt {
        /// Points forward (forwards is positive).
        const FORWARD: Self;
    }

    /// `BACKWARD` constant where forwards is positive.
    pub trait NegBackwardExt {
        /// Points backward (forwards is positive).
        const BACKWARD: Self;
    }

    impl<T: ScalarOne> ForwardExt for T {
        const FORWARD: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegBackwardExt for T {
        const BACKWARD: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> ForwardExt for Vector<3, T, A> {
        const FORWARD: Self = Self::Z;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegBackwardExt for Vector<3, T, A> {
        const BACKWARD: Self = Self::NEG_Z;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> ForwardExt for Vector<4, T, A> {
        const FORWARD: Self = Self::Z;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegBackwardExt for Vector<4, T, A> {
        const BACKWARD: Self = Self::NEG_Z;
    }
}

/// `BACKWARD` and `FORWARD` constants where backwards is positive.
pub mod backward {
    use crate::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `BACKWARD` constant where backwards is positive.
    pub trait BackwardExt {
        /// Points backward (backwards is positive).
        const BACKWARD: Self;
    }

    /// `FORWARD` constant where backwards is positive.
    pub trait NegForwardExt {
        /// Points forward (backwards is positive).
        const FORWARD: Self;
    }

    impl<T: ScalarOne> BackwardExt for T {
        const BACKWARD: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegForwardExt for T {
        const FORWARD: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> BackwardExt for Vector<3, T, A> {
        const BACKWARD: Self = Self::Z;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegForwardExt for Vector<3, T, A> {
        const FORWARD: Self = Self::NEG_Z;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> BackwardExt for Vector<4, T, A> {
        const BACKWARD: Self = Self::Z;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegForwardExt for Vector<4, T, A> {
        const FORWARD: Self = Self::NEG_Z;
    }
}
