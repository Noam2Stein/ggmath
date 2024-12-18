use ggmath::scalar::{scalar_inner_vectors, Scalar};
use ggmath_testing::{test_scalar, ScalarTestingResultFmt, TestableScalar};

// Custom Type
#[derive(Debug, Clone, Copy, PartialEq)]
struct MyScalar(i32);

// Implement Scalar for MyScalar
scalar_inner_vectors!(MyScalar(4));
impl Scalar for MyScalar {}

// Implement TestableScalar for MyScalar
impl TestableScalar for MyScalar {
    const VALUES: [Self; 16] = [
        Self(1),
        Self(2),
        Self(4),
        Self(6),
        Self(7),
        Self(4201),
        Self(1257),
        Self(74),
        Self(-64),
        Self(23),
        Self(87),
        Self(-8765),
        Self(-5236),
        Self(0),
        Self(i32::MIN), // Give it values that might trigger edge cases
        Self(i32::MAX),
    ];
}

fn main() {
    println!("{}", test_scalar::<MyScalar>().scalar_test_fmt())
}
