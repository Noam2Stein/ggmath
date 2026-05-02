/// Contains 2 consecutive values of `T`.
///
/// This is used instead of `[T; 2]` because for some reason arrays lead to
/// worse assembly than structs.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Repr2<T>(pub T, pub T);

/// Contains 3 consecutive values of `T`.
///
/// This is used instead of `[T; 3]` because for some reason arrays lead to
/// worse assembly than structs.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Repr3<T>(pub T, pub T, pub T);

/// Contains 4 consecutive values of `T`.
///
/// This is used instead of `[T; 4]` because for some reason arrays lead to
/// worse assembly than structs.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Repr4<T>(pub T, pub T, pub T, pub T);
