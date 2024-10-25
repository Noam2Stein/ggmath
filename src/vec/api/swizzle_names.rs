use ggmath_proc_macros::swizzles;

use super::*;

ggmath_proc_macros::swizzles_macro!(
    vec2_swizzles {
        impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A> where ScalarCount<N>: VecLen<N> {

        }
    }
);
swizzles!(vec2_swizzles(x, y));
