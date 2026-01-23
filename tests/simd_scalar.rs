#![allow(unused)]

use std::ops::Add;

use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength, Unaligned, Vector};

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

impl Scalar for Foo {}

unsafe impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
where
    Length<N>: SupportedLength,
{
    type VectorRepr = Vector<N, f32, A>;

    #[inline]
    fn vec_add(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        (vec.repr() + rhs.repr()).to_vec()
    }
}

trait ToVec {
    type Vector;

    fn to_vec(self) -> Self::Vector;
}

impl<const N: usize, A: Alignment> ToVec for Vector<N, f32, A>
where
    Length<N>: SupportedLength,
{
    type Vector = Vector<N, Foo, A>;

    #[inline]
    fn to_vec(self) -> Self::Vector {
        // SAFETY: any value of `f32` is a valid value of `Foo`.
        unsafe { Vector::from_repr(self) }
    }
}
