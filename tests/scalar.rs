use std::ops::Add;

use ggmath::{CustomScalar, Vec3};

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

impl CustomScalar for Foo {}

#[test]
fn test_add() {
    let a = Vec3::new(Foo(1.0), Foo(2.0), Foo(3.0));
    let b = Vec3::new(Foo(4.0), Foo(5.0), Foo(6.0));

    assert_eq!(a + b, Vec3::new(Foo(5.0), Foo(7.0), Foo(9.0)));
}
