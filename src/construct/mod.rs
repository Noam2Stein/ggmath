//! base traits for mathamatical types.

use std::fmt::{Debug, Display};

/// The base trait for mathamatical types.
///
/// Includes core Rust traits like [Copy] and [Display].
/// Is automatically implemented for types that implement its supertraits.
pub trait Construct: InnerConstruct + PartialEq + Display {}

/// The base trait for inner mathamatical types.
///
/// Includes core Rust traits needed for inner data like [Copy] and [Debug], but not outer traits like [Display].
/// - Anything that implements [Construct] also implements [InnerConstruct].
pub trait InnerConstruct: Sized + Send + Sync + Copy + Debug {}

impl<T: InnerConstruct + PartialEq + Display> Construct for T {}
impl<T: Sized + Send + Sync + Copy + Debug> InnerConstruct for T {}
