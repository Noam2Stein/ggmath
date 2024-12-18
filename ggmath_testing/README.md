
GGMath is a generic-graphics-math crate.
So types like ```Vector``` and ```Matrix``` are generic over ```T: Scalar```.

You can implement ```Scalar``` for your custom types,
and when you do so you can override the implementations of vector functions for your custom scalar
to make optimizations only possible with your type.

These custom function implementations can have bugs,
so this crate helps you test them.

## Usage

1. Create a custom scalar type
2. Implement ```TestableScalar``` for your type. Requires you to provide 16 values which will be used for testing.
3. ```println!("{}", test_scalar::<MyScalar>().scalar_test_fmt())```

```rust
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
```