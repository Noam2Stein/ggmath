use std::fmt::{self, Display, Formatter};

use crate::element::*;

pub trait MatMajor: fmt::Debug + Copy + PartialEq + PartialOrd + Default {
    type Mat2Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat2x3Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat2x4Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat3x2Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat3Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat3x4Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat4x2Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat4x3Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat4Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
}
pub trait MatCxR: fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display {
    type T: Element;
    const C: usize;
    const R: usize;
    type M: MatMajor;
}

macro_rules! mat {
    ($outer:ident($inner:ident): $c:literal * $r:literal) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $outer<T: Element, M: MatMajor> {
            inner: M::$inner<T>,
        }
        impl<T: Element, M: MatMajor> Display for $outer<T, M> {
            fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
                todo!()
            }
        }
        impl<T: Element, M: MatMajor> MatCxR for $outer<T, M> {
            type T = T;
            const C: usize = $c;
            const R: usize = $r;
            type M = M;
        }
    };
}

mat!(Mat2  (Mat2Inner):   2 * 2);
mat!(Mat2x3(Mat2x3Inner): 2 * 3);
mat!(Mat2x4(Mat2x4Inner): 2 * 4);
mat!(Mat3x2(Mat2Inner):   3 * 2);
mat!(Mat3  (Mat2x3Inner): 3 * 3);
mat!(Mat3x4(Mat2x4Inner): 3 * 4);
mat!(Mat4x2(Mat2Inner):   4 * 2);
mat!(Mat4x3(Mat2x3Inner): 4 * 3);
mat!(Mat4  (Mat2x4Inner): 4 * 4);
