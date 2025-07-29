#![deny(missing_docs)]

//! Vector!

use super::{construct::*, scalar::*, *};

mod declaration;
pub use declaration::*;

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
