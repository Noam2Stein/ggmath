use crevice::{std140::AsStd140, std430::AsStd430};

use super::*;

macro_loop! {
    @for N in 2..=4, [T, TVec] in [
        [f32, Vec],
        [f64, DVec],
        [i32, IVec],
        [u32, UVec],
        [bool, BVec],
    ] {
        @let components = [x, y, z, w][0..@N];

        impl<A: VecAlignment> AsStd140 for Vector<@N, @T, A> {
            type Output = crevice::std140::@[@TVec @N];

            fn as_std140(&self) -> Self::Output {
                Self::Output {
                    @for c in @components {
                        @c: self.@c().as_std140(),
                    }
                }
            }

            fn from_std140(val: Self::Output) -> Self {
                vector!(
                    @for c in @components {
                        @T::from_std140(val.@c),
                    }
                )
            }
        }

        impl<A: VecAlignment> AsStd430 for Vector<@N, @T, A> {
            type Output = crevice::std430::@[@TVec @N];

            fn as_std430(&self) -> Self::Output {
                Self::Output {
                    @for c in @components {
                        @c: self.@c().as_std430(),
                    }
                }
            }

            fn from_std430(val: Self::Output) -> Self {
                vector!(
                    @for c in @components {
                        @T::from_std430(val.@c),
                    }
                )
            }
        }
    }
}
