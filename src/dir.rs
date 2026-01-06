/// `RIGHT` and `LEFT` constants where right is positive along the x-axis.
pub mod right {
    use crate::vector::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `RIGHT` constant where right is positive along the x-axis.
    pub trait RightExt {
        /// Points right (right is positive along the x-axis).
        const RIGHT: Self;
    }

    /// `LEFT` constant where right is positive along the x-axis.
    pub trait NegLeftExt {
        /// Points left (right is positive along the x-axis).
        const LEFT: Self;
    }

    impl<T: ScalarOne> RightExt for T {
        /// `1`.
        const RIGHT: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegLeftExt for T {
        /// `-1`.
        const LEFT: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> RightExt for Vector<2, T, A> {
        /// `(1, 0)`.
        const RIGHT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegLeftExt for Vector<2, T, A> {
        /// `(-1, 0)`.
        const LEFT: Self = Self::NEG_X;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> RightExt for Vector<3, T, A> {
        /// `(1, 0, 0)`.
        const RIGHT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegLeftExt for Vector<3, T, A> {
        /// `(-1, 0, 0)`.
        const LEFT: Self = Self::NEG_X;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> RightExt for Vector<4, T, A> {
        /// `(1, 0, 0, 0)`.
        const RIGHT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegLeftExt for Vector<4, T, A> {
        /// `(-1, 0, 0, 0)`.
        const LEFT: Self = Self::NEG_X;
    }
}

/// `LEFT` and `RIGHT` constants where left is positive along the x-axis.
pub mod left {
    use crate::vector::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `LEFT` constant where left is positive along the x-axis.
    pub trait LeftExt {
        /// Points left (left is positive along the x-axis).
        const LEFT: Self;
    }

    /// `RIGHT` constant where left is positive along the x-axis.
    pub trait NegRightExt {
        /// Points right (left is positive along the x-axis).
        const RIGHT: Self;
    }

    impl<T: ScalarOne> LeftExt for T {
        /// `1`.
        const LEFT: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegRightExt for T {
        /// `-1`.
        const RIGHT: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> LeftExt for Vector<2, T, A> {
        /// `(1, 0)`.
        const LEFT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegRightExt for Vector<2, T, A> {
        /// `(-1, 0)`.
        const RIGHT: Self = Self::NEG_X;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> LeftExt for Vector<3, T, A> {
        /// `(1, 0, 0)`.
        const LEFT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegRightExt for Vector<3, T, A> {
        /// `(-1, 0, 0)`.
        const RIGHT: Self = Self::NEG_X;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> LeftExt for Vector<4, T, A> {
        /// `(1, 0, 0, 0)`.
        const LEFT: Self = Self::X;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegRightExt for Vector<4, T, A> {
        /// `(-1, 0, 0, 0)`.
        const RIGHT: Self = Self::NEG_X;
    }
}

/// `UP` and `DOWN` constants where up is positive along the y-axis.
pub mod up {
    use crate::vector::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `UP` constant where up is positive along the y-axis.
    pub trait UpExt {
        /// Points up (up is positive along the y-axis).
        const UP: Self;
    }

    /// `DOWN` constant where up is positive along the y-axis.
    pub trait NegDownExt {
        /// Points down (up is positive along the y-axis).
        const DOWN: Self;
    }

    impl<T: ScalarOne> UpExt for T {
        /// `1`.
        const UP: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegDownExt for T {
        /// `-1`.
        const DOWN: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> UpExt for Vector<2, T, A> {
        /// `(0, 1)`.
        const UP: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegDownExt for Vector<2, T, A> {
        /// `(0, -1)`.
        const DOWN: Self = Self::NEG_Y;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> UpExt for Vector<3, T, A> {
        /// `(0, 1, 0)`.
        const UP: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegDownExt for Vector<3, T, A> {
        /// `(0, -1, 0)`.
        const DOWN: Self = Self::NEG_Y;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> UpExt for Vector<4, T, A> {
        /// `(0, 1, 0, 0)`.
        const UP: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegDownExt for Vector<4, T, A> {
        /// `(0, -1, 0, 0)`.
        const DOWN: Self = Self::NEG_Y;
    }
}

/// `DOWN` and `UP` constants where down is positive along the y-axis.
pub mod down {
    use crate::vector::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `DOWN` constant where down is positive along the y-axis.
    pub trait DownExt {
        /// Points down (down is positive along the y-axis).
        const DOWN: Self;
    }

    /// `UP` constant where down is positive along the y-axis.
    pub trait NegUpExt {
        /// Points up (down is positive along the y-axis).
        const UP: Self;
    }

    impl<T: ScalarOne> DownExt for T {
        /// `1`.
        const DOWN: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegUpExt for T {
        /// `-1`.
        const UP: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> DownExt for Vector<2, T, A> {
        /// `(0, 1)`.
        const DOWN: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegUpExt for Vector<2, T, A> {
        /// `(0, -1)`.
        const UP: Self = Self::NEG_Y;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> DownExt for Vector<3, T, A> {
        /// `(0, 1, 0)`.
        const DOWN: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegUpExt for Vector<3, T, A> {
        /// `(0, -1, 0)`.
        const UP: Self = Self::NEG_Y;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> DownExt for Vector<4, T, A> {
        /// `(0, 1, 0, 0)`.
        const DOWN: Self = Self::Y;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegUpExt for Vector<4, T, A> {
        /// `(0, -1, 0, 0)`.
        const UP: Self = Self::NEG_Y;
    }
}

/// `FORWARD` and `BACKWARD` constants where forward is positive along the
/// z-axis.
pub mod forward {
    use crate::vector::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `FORWARD` constant where forward is positive along the z-axis.
    pub trait ForwardExt {
        /// Points forward (forward is positive along the z-axis).
        const FORWARD: Self;
    }

    /// `BACKWARD` constant where forward is positive along the z-axis.
    pub trait NegBackwardExt {
        /// Points backward (forward is positive along the z-axis).
        const BACKWARD: Self;
    }

    impl<T: ScalarOne> ForwardExt for T {
        /// `1`.
        const FORWARD: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegBackwardExt for T {
        /// `-1`.
        const BACKWARD: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> ForwardExt for Vector<3, T, A> {
        /// `(0, 0, 1)`.
        const FORWARD: Self = Self::Z;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegBackwardExt for Vector<3, T, A> {
        /// `(0, 0, -1)`.
        const BACKWARD: Self = Self::NEG_Z;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> ForwardExt for Vector<4, T, A> {
        /// `(0, 0, 1, 0)`.
        const FORWARD: Self = Self::Z;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegBackwardExt for Vector<4, T, A> {
        /// `(0, 0, -1, 0)`.
        const BACKWARD: Self = Self::NEG_Z;
    }
}

/// `BACKWARD` and `FORWARD` constants where backward is positive along the
/// z-axis.
pub mod backward {
    use crate::vector::{Alignment, ScalarNegOne, ScalarOne, ScalarZero, Vector};

    /// `BACKWARD` constant where backward is positive along the z-axis.
    pub trait BackwardExt {
        /// Points backward (backward is positive along the z-axis).
        const BACKWARD: Self;
    }

    /// `FORWARD` constant where backward is positive along the z-axis.
    pub trait NegForwardExt {
        /// Points forward (backward is positive along the z-axis).
        const FORWARD: Self;
    }

    impl<T: ScalarOne> BackwardExt for T {
        /// `1`.
        const BACKWARD: Self = Self::ONE;
    }

    impl<T: ScalarNegOne> NegForwardExt for T {
        /// `-1`.
        const FORWARD: Self = Self::NEG_ONE;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> BackwardExt for Vector<3, T, A> {
        /// `(0, 0, 1)`.
        const BACKWARD: Self = Self::Z;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegForwardExt for Vector<3, T, A> {
        /// `(0, 0, -1)`.
        const FORWARD: Self = Self::NEG_Z;
    }

    impl<T: ScalarZero + ScalarOne, A: Alignment> BackwardExt for Vector<4, T, A> {
        /// `(0, 0, 1, 0)`.
        const BACKWARD: Self = Self::Z;
    }

    impl<T: ScalarZero + ScalarNegOne, A: Alignment> NegForwardExt for Vector<4, T, A> {
        /// `(0, 0, -1, 0)`.
        const FORWARD: Self = Self::NEG_Z;
    }
}
