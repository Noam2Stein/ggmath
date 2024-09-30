use std::fmt::{self, Display, Formatter};

use crate::element::*;

pub mod inner;
use inner::*;

mod from_split;
pub use from_split::*;

trait Seal {}

#[allow(private_bounds)]
pub trait VecN: Seal + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display {
    type T: Element;
    const N: usize;
}

macro_rules! vecn {
    ($outer:ident($inner:ident): $n:literal) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $outer<T: Element> {
            inner: T::$inner,
        }
        impl<T: Element> Seal for $outer<T> {}

        impl<T: Element> Default for $outer<T> {
            fn default() -> Self {
                Self {
                    inner: T::$inner::default(),
                }
            }
        }
        impl<T: Element> Display for $outer<T> {
            fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
                todo!()
            }
        }
        impl<T: Element> VecN for $outer<T> {
            type T = T;
            const N: usize = $n;
        }
    };
}
vecn!(Vec2(InnerVec2): 2);
vecn!(Vec3(InnerVec3): 3);
vecn!(Vec4(InnerVec4): 4);

type Vec1<T> = T;
