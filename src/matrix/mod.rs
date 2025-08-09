//! Matrix type?

use derive_where::derive_where;

use super::*;

mod conversion;
mod declaration;
mod default;
mod fmt;
mod hash;
mod index;
mod new;
mod ops;
mod swizzle;
pub use declaration::*;
pub use new::*;

mod generics;
pub use generics::*;
