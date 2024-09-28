use std::fmt::{self, Display, Formatter};

use crate::element::*;

pub trait VecN: fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display {
    type T: Element;
    const N: usize;
}

macro_rules! vecn {
    ($outer:ident($inner:ident): $n:literal) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $outer<T: Element> {
            inner: T::$inner,
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

vecn!(Vec2(Vec2Inner): 2);
vecn!(Vec3(Vec3Inner): 3);
vecn!(Vec4(Vec4Inner): 4);
