#![allow(dead_code)]

use std::ops::Add;

use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength, Vector};

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Foo(f32);

impl Add for Foo {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// SAFETY: `Foo` and `i32` are both 4-bytes long, and `Foo` has no uninitialized
// bytes because its a simple wrapper around `f32`.
unsafe impl Scalar for Foo {
    type Repr = i32;
}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn vec_add(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        (vec.to_f32() + rhs.to_f32()).to_foo()
    }
}

trait ToF32 {
    type Output;

    fn to_f32(self) -> Self::Output;
}

trait ToFoo {
    type Output;

    fn to_foo(self) -> Self::Output;
}

impl<const N: usize, A: Alignment> ToF32 for Vector<N, Foo, A>
where
    Length<N>: SupportedLength,
{
    type Output = Vector<N, f32, A>;

    #[inline]
    fn to_f32(self) -> Self::Output {
        // `f32` accepts all bit-patterns.
        unsafe { self.to_repr() }
    }
}

impl<const N: usize, A: Alignment> ToFoo for Vector<N, f32, A>
where
    Length<N>: SupportedLength,
{
    type Output = Vector<N, Foo, A>;

    #[inline]
    fn to_foo(self) -> Self::Output {
        // `Foo` accepts all bit-patterns.
        unsafe { self.to_repr() }
    }
}
