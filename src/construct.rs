//! The base trait for mathamatical types.

use std::panic::{RefUnwindSafe, UnwindSafe};

/// The base trait for mathamatical types.
///
/// Makes sure a type represents data that is always valid and can be copied and sent anywhere.
/// Is automatically implemented for types that implement its supertraits.
pub trait Construct: Sized + Send + Sync + Copy + 'static + UnwindSafe + RefUnwindSafe {}

impl<T: Sized + Send + Sync + Copy + 'static + UnwindSafe + RefUnwindSafe> Construct for T {}
