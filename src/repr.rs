/// Contains 2 consecutive values of `T`.
///
/// This is used instead of `[T; 2]` because for some reason arrays lead to
/// worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr2<T>(pub T, pub T);

/// Contains 3 consecutive values of `T`.
///
/// This is used instead of `[T; 3]` because for some reason arrays lead to
/// worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr3<T>(pub T, pub T, pub T);

/// Contains 4 consecutive values of `T`.
///
/// This is used instead of `[T; 4]` because for some reason arrays lead to
/// worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr4<T>(pub T, pub T, pub T, pub T);

/// Contains 5 consecutive values of `T`.
///
/// This is used instead of `[T; 5]` because for some reason arrays lead to
/// worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr5<T>(pub T, pub T, pub T, pub T, pub T);
