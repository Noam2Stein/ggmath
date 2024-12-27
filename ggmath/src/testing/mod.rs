//!
//! GGMath is a generic-graphics-math crate.
//! So types like ```Vector``` and ```Matrix``` are generic over ```T: Scalar```.
//!
//! You can implement ```Scalar``` for your custom types,
//! and when you do so you can override the implementations of vector functions for your custom scalar
//! to make optimizations only possible with your type.
//!
//! These custom function implementations can have bugs,
//! so this crate helps you test them.
//!
//! ## Usage
//!
//! 1. Make sure that in your Cargo.toml, you configure:
//! ```toml
//! [profile]
//! dev.panic     = "unwind"
//! release.panic = "unwind"
//! ```
//!
//! 2. Create a custom scalar type
//! 3. Implement ```TestableScalar``` for your type. Requires you to provide 16 values which will be used for testing.
//! 4. ```println!("{}", test_scalar::<MyScalar>().fmt_test_result())```
//!
//! ```rust
//! use ggmath::scalar::{scalar_inner_vectors, Scalar};
//! use ggmath_testing::{FormatTestingResult, test_scalar, TestableScalar};
//!
//! // Custom Type
//! #[derive(Debug, Clone, Copy, PartialEq)]
//! struct MyScalar(i32);
//!
//! // Implement Scalar for MyScalar
//! scalar_inner_vectors!(MyScalar(4));
//! impl Scalar for MyScalar {}
//!
//! // Implement TestableScalar for MyScalar
//! impl TestableScalar for MyScalar {
//!     const VALUES: [Self; 16] = [
//!         Self(1),
//!         Self(2),
//!         Self(4),
//!         Self(6),
//!         Self(7),
//!         Self(4201),
//!         Self(1257),
//!         Self(74),
//!         Self(-64),
//!         Self(23),
//!         Self(87),
//!         Self(-8765),
//!         Self(-5236),
//!         Self(0),
//!         Self(i32::MIN), // Provide values that might trigger edge cases
//!         Self(i32::MAX),
//!     ];
//! }
//!
//! fn main() {
//!     println!("{}", test_scalar::<MyScalar>().fmt_test_result())
//! }
//! ```

mod failed_fn;
mod test_assert;
mod test_eq;
mod test_error;
mod test_scalar;
mod testable_scalar;
pub use failed_fn::*;
pub use test_assert::*;
pub use test_eq::*;
pub use test_error::*;
pub use test_scalar::*;
pub use testable_scalar::*;

#[cfg(feature = "matrix")]
mod matrix;
#[cfg(feature = "matrix")]
pub use matrix::*;

#[cfg(feature = "rectangle")]
mod rectangle;
#[cfg(feature = "rectangle")]
pub use rectangle::*;
