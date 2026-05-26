use std::ops::Add;

use ggmath::{
    Aligned, Backend, DefaultBackend, Length, Mask, Scalar, SupportedLength, Unaligned, Vec3A,
    Vector,
};

#[test]
fn test_add() {
    let a = Vec3A::new(Foo(1.0), Foo(2.0), Foo(3.0));
    let b = Vec3A::new(Foo(4.0), Foo(5.0), Foo(6.0));

    assert_eq!(a + b, Vec3A::new(Foo(5.0), Foo(7.0), Foo(9.0)));
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct Foo(f32);

impl Add for Foo {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Scalar for Foo {}

impl<const N: usize> DefaultBackend<N, Unaligned> for Foo {}

// SAFETY: All associated types uphold requirements.
unsafe impl<const N: usize> Backend<N, Aligned> for Foo
where
    Length<N>: SupportedLength,
{
    type Vector = Vector<N, f32, Aligned>;
    type Mask = Mask<N, f32, Aligned>;

    #[inline]
    fn vector_add(
        vector: Vector<N, Self, Aligned>,
        rhs: Vector<N, Self, Aligned>,
    ) -> Vector<N, Self, Aligned> {
        Vector::from_inner(vector.inner() + rhs.inner())
    }

    #[inline]
    fn mask_from_array(array: [bool; N]) -> Mask<N, Self, Aligned> {
        Mask::from_inner(Mask::from_array(array))
    }

    #[inline]
    fn mask_to_array(mask: Mask<N, Self, Aligned>) -> [bool; N] {
        mask.inner().to_array()
    }
}
