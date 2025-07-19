use crevice::{std140::AsStd140, std430::AsStd430};

use super::*;

macro_loop! {
    @for N in 2..=4, [T, TMat, std140_vec2_has_padding] in [
        [f32, Mat, true],
        [f64, DMat, false],
    ] {
        @let components = [x, y, z, w][0..@N];

        impl<A: VecAlignment, M: MatrixMajorAxis> AsStd140 for Matrix<@N, @N, @T, A, M> {
            type Output = crevice::std140::@[@TMat @N];

            fn as_std140(&self) -> Self::Output {
                Self::Output {
                    @for [c_idx, c] in @components.enumerate() {
                        @c: self.@[c @c_idx]().as_std140(),

                        @if @N == 3 || @N == 2 && @std140_vec2_has_padding {
                            @[_pad_ @c]: Default::default(),
                        }
                    }
                }
            }

            fn from_std140(val: Self::Output) -> Self {
                Self::from_columns([
                    @for c in @components {
                        Vector::<@N, @T, A>::from_std140(val.@c),
                    }
                ])
            }
        }

        impl<A: VecAlignment, M: MatrixMajorAxis> AsStd430 for Matrix<@N, @N, @T, A, M> {
            type Output = crevice::std430::@[@TMat @N];

            fn as_std430(&self) -> Self::Output {
                Self::Output {
                    @for [c_idx, c] in @components.enumerate() {
                        @c: self.@[c @c_idx]().as_std430(),

                        @if @N == 3 {
                            @[_pad_ @c]: Default::default(),
                        }
                    }
                }
            }

            fn from_std430(val: Self::Output) -> Self {
                Self::from_columns([
                    @for c in @components {
                        Vector::<@N, @T, A>::from_std430(val.@c),
                    }
                ])
            }
        }
    }
}
