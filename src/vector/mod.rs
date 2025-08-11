//! Vector!

use super::*;

mod declaration;
pub use declaration::*;

mod cmp;
mod conversion;
mod default;
mod fmt;
mod generics;
mod index;
mod iterator;
mod new;
mod ops;
mod swizzle;
pub use generics::*;
pub use new::*;
