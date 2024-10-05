use vec::swizzle::ElementVecSwizzle;

use super::*;

mod get;
mod with;

impl<T: ElementDefaultImpl> ElementVecSwizzle for T {}
