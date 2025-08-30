use crate::{
    Construct,
    vector::{ScalarNegOne, ScalarOne, ScalarZero, VecAlignment, Vector},
};

/// A trait for a `Right` constant where right is the positive direction.
#[cfg(feature = "right")]
pub trait PositiveRight: Construct {
    /// A value that points right with magnitude `1` where right is the positive direction.
    const RIGHT: Self;
}

/// A trait for a `Left` constant where left is the positive direction.
#[cfg(feature = "left")]
pub trait PositiveLeft: Construct {
    /// A value that points left with magnitude `1` where left is the positive direction.
    const LEFT: Self;
}

/// A trait for a `Right` constant where left is the positive direction.
#[cfg(feature = "left")]
pub trait NegativeRight: Construct {
    /// A value that points right with magnitude `1` where left is the positive direction.
    const RIGHT: Self;
}

/// A trait for a `Left` constant where right is the positive direction.
#[cfg(feature = "right")]
pub trait NegativeLeft: Construct {
    /// A value that points left with magnitude `1` where right is the positive direction.
    const LEFT: Self;
}

/// A trait for a `Up` constant where up is the positive direction.
#[cfg(feature = "up")]
pub trait PositiveUp: Construct {
    /// A value that points up with magnitude `1` where up is the positive direction.
    const UP: Self;
}

/// A trait for a `Down` constant where down is the positive direction.
#[cfg(feature = "down")]
pub trait PositiveDown: Construct {
    /// A value that points down with magnitude `1` where down is the positive direction.
    const DOWN: Self;
}

/// A trait for a `Up` constant where down is the positive direction.
#[cfg(feature = "down")]
pub trait NegativeUp: Construct {
    /// A value that points up with magnitude `1` where down is the positive direction.
    const UP: Self;
}

/// A trait for a `Down` constant where up is the positive direction.
#[cfg(feature = "up")]
pub trait NegativeDown: Construct {
    /// A value that points down with magnitude `1` where up is the positive direction.
    const DOWN: Self;
}

/// A trait for a `Forward` constant where forwards is the positive direction.
#[cfg(feature = "forwards")]
pub trait PositiveForward: Construct {
    /// A value that points forwards with magnitude `1` where forwards is the positive direction.
    const FORWARD: Self;
}

/// A trait for a `Backward` constant where backwards is the positive direction.
#[cfg(feature = "backwards")]
pub trait PositiveBackward: Construct {
    /// A value that points backwards with magnitude `1` where backwards is the positive direction.
    const BACKWARD: Self;
}

/// A trait for a `Forward` constant where backwards is the positive direction.
#[cfg(feature = "backwards")]
pub trait NegativeForward: Construct {
    /// A value that points forwards with magnitude `1` where backwards is the positive direction.
    const FORWARD: Self;
}

/// A trait for a `Backward` constant where forwards is the positive direction.
#[cfg(feature = "forwards")]
pub trait NegativeBackward: Construct {
    /// A value that points backwards with magnitude `1` where forwards is the positive direction.
    const BACKWARD: Self;
}

#[cfg(feature = "right")]
impl<T: ScalarOne> PositiveRight for T {
    const RIGHT: Self = Self::ONE;
}

#[cfg(feature = "left")]
impl<T: ScalarOne> PositiveLeft for T {
    const LEFT: Self = Self::ONE;
}

#[cfg(feature = "left")]
impl<T: ScalarNegOne> NegativeRight for T {
    const RIGHT: Self = Self::NEG_ONE;
}

#[cfg(feature = "right")]
impl<T: ScalarNegOne> NegativeLeft for T {
    const LEFT: Self = Self::NEG_ONE;
}

#[cfg(feature = "right")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveRight for Vector<2, T, A> {
    const RIGHT: Self = Self::from_array([T::ONE, T::ZERO]);
}

#[cfg(feature = "left")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveLeft for Vector<2, T, A> {
    const LEFT: Self = Self::from_array([T::ONE, T::ZERO]);
}

#[cfg(feature = "left")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeRight for Vector<2, T, A> {
    const RIGHT: Self = Self::from_array([T::NEG_ONE, T::ZERO]);
}

#[cfg(feature = "right")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeLeft for Vector<2, T, A> {
    const LEFT: Self = Self::from_array([T::NEG_ONE, T::ZERO]);
}

#[cfg(feature = "right")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveRight for Vector<3, T, A> {
    const RIGHT: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO]);
}

#[cfg(feature = "left")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveLeft for Vector<3, T, A> {
    const LEFT: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO]);
}

#[cfg(feature = "left")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeRight for Vector<3, T, A> {
    const RIGHT: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO]);
}

#[cfg(feature = "right")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeLeft for Vector<3, T, A> {
    const LEFT: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO]);
}

#[cfg(feature = "right")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveRight for Vector<4, T, A> {
    const RIGHT: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
}

#[cfg(feature = "left")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveLeft for Vector<4, T, A> {
    const LEFT: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
}

#[cfg(feature = "left")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeRight for Vector<4, T, A> {
    const RIGHT: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO]);
}

#[cfg(feature = "right")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeLeft for Vector<4, T, A> {
    const LEFT: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO]);
}

#[cfg(feature = "up")]
impl<T: ScalarOne> PositiveUp for T {
    const UP: Self = Self::ONE;
}

#[cfg(feature = "down")]
impl<T: ScalarOne> PositiveDown for T {
    const DOWN: Self = Self::ONE;
}

#[cfg(feature = "down")]
impl<T: ScalarNegOne> NegativeUp for T {
    const UP: Self = Self::NEG_ONE;
}

#[cfg(feature = "up")]
impl<T: ScalarNegOne> NegativeDown for T {
    const DOWN: Self = Self::NEG_ONE;
}

#[cfg(feature = "up")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveUp for Vector<2, T, A> {
    const UP: Self = Self::from_array([T::ZERO, T::ONE]);
}

#[cfg(feature = "down")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveDown for Vector<2, T, A> {
    const DOWN: Self = Self::from_array([T::ZERO, T::ONE]);
}

#[cfg(feature = "down")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeUp for Vector<2, T, A> {
    const UP: Self = Self::from_array([T::ZERO, T::NEG_ONE]);
}

#[cfg(feature = "up")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeDown for Vector<2, T, A> {
    const DOWN: Self = Self::from_array([T::ZERO, T::NEG_ONE]);
}

#[cfg(feature = "up")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveUp for Vector<3, T, A> {
    const UP: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO]);
}

#[cfg(feature = "down")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveDown for Vector<3, T, A> {
    const DOWN: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO]);
}

#[cfg(feature = "down")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeUp for Vector<3, T, A> {
    const UP: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO]);
}

#[cfg(feature = "up")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeDown for Vector<3, T, A> {
    const DOWN: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO]);
}

#[cfg(feature = "up")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveUp for Vector<4, T, A> {
    const UP: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
}

#[cfg(feature = "down")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveDown for Vector<4, T, A> {
    const DOWN: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
}

#[cfg(feature = "down")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeUp for Vector<4, T, A> {
    const UP: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO]);
}

#[cfg(feature = "up")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeDown for Vector<4, T, A> {
    const DOWN: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO]);
}

#[cfg(feature = "forwards")]
impl<T: ScalarOne> PositiveForward for T {
    const FORWARD: Self = Self::ONE;
}

#[cfg(feature = "backwards")]
impl<T: ScalarOne> PositiveBackward for T {
    const BACKWARD: Self = Self::ONE;
}

#[cfg(feature = "backwards")]
impl<T: ScalarNegOne> NegativeForward for T {
    const FORWARD: Self = Self::NEG_ONE;
}

#[cfg(feature = "forwards")]
impl<T: ScalarNegOne> NegativeBackward for T {
    const BACKWARD: Self = Self::NEG_ONE;
}

#[cfg(feature = "forwards")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveForward for Vector<2, T, A> {
    const FORWARD: Self = Self::from_array([T::ZERO, T::ZERO]);
}

#[cfg(feature = "backwards")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveBackward for Vector<2, T, A> {
    const BACKWARD: Self = Self::from_array([T::ZERO, T::ZERO]);
}

#[cfg(feature = "backwards")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeForward for Vector<2, T, A> {
    const FORWARD: Self = Self::from_array([T::ZERO, T::ZERO]);
}

#[cfg(feature = "forwards")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeBackward for Vector<2, T, A> {
    const BACKWARD: Self = Self::from_array([T::ZERO, T::ZERO]);
}

#[cfg(feature = "forwards")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveForward for Vector<3, T, A> {
    const FORWARD: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE]);
}

#[cfg(feature = "backwards")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveBackward for Vector<3, T, A> {
    const BACKWARD: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE]);
}

#[cfg(feature = "backwards")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeForward for Vector<3, T, A> {
    const FORWARD: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE]);
}

#[cfg(feature = "forwards")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeBackward for Vector<3, T, A> {
    const BACKWARD: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE]);
}

#[cfg(feature = "forwards")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveForward for Vector<4, T, A> {
    const FORWARD: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
}

#[cfg(feature = "backwards")]
impl<T: ScalarZero + ScalarOne, A: VecAlignment> PositiveBackward for Vector<4, T, A> {
    const BACKWARD: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
}

#[cfg(feature = "backwards")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeForward for Vector<4, T, A> {
    const FORWARD: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO]);
}

#[cfg(feature = "forwards")]
impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> NegativeBackward for Vector<4, T, A> {
    const BACKWARD: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO]);
}
