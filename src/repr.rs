/*
These types are used instead of arrays because for some reason arrays lead to
worse assembly than structs.

When this is fixed, these structs should be removed.
*/

/// Contains two values of type `T`.
///
/// This type is used instead of `[T; 2]` because for some reason arrays lead
/// to worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr2<T>(pub T, pub T);

/// Contains three values of type `T`.
///
/// This type is used instead of `[T; 3]` because for some reason arrays lead
/// to worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr3<T>(pub T, pub T, pub T);

/// Contains four values of type `T`.
///
/// This type is used instead of `[T; 4]` because for some reason arrays lead
/// to worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr4<T>(pub T, pub T, pub T, pub T);

/// Contains five values of type `T`.
///
/// This type is used instead of `[T; 5]` because for some reason arrays lead
/// to worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr5<T>(pub T, pub T, pub T, pub T, pub T);
