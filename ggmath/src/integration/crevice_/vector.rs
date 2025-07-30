use crevice::{
    std140::{AsStd140, Std140},
    std430::{AsStd430, Std430},
};

use super::*;

pub trait ScalarAsStd140: Scalar + AsStd140 {
    type OutputVec2: Std140;
    type OutputVec3: Std140;
    type OutputVec4: Std140;

    fn to_vec2(value: [Self::Output; 2]) -> Self::OutputVec2;
    fn to_vec3(value: [Self::Output; 3]) -> Self::OutputVec3;
    fn to_vec4(value: [Self::Output; 4]) -> Self::OutputVec4;

    fn from_vec2(vec: Self::OutputVec2) -> [Self::Output; 2];
    fn from_vec3(vec: Self::OutputVec3) -> [Self::Output; 3];
    fn from_vec4(vec: Self::OutputVec4) -> [Self::Output; 4];
}

pub trait ScalarAsStd430: Scalar + AsStd430 {
    type OutputVec2: Std430;
    type OutputVec3: Std430;
    type OutputVec4: Std430;

    fn to_vec2(value: [Self::Output; 2]) -> Self::OutputVec2;
    fn to_vec3(value: [Self::Output; 3]) -> Self::OutputVec3;
    fn to_vec4(value: [Self::Output; 4]) -> Self::OutputVec4;

    fn from_vec2(vec: Self::OutputVec2) -> [Self::Output; 2];
    fn from_vec3(vec: Self::OutputVec3) -> [Self::Output; 3];
    fn from_vec4(vec: Self::OutputVec4) -> [Self::Output; 4];
}

macro_loop! {
    @for N in 2..=4, std in [140, 430] {
        @let ScalarAsStd = @[ScalarAsStd @std];
        @let AsStd = @[AsStd @std];
        @let as_std = @[as_std @std];
        @let from_std = @[from_std @std];

        impl<A: VecAlignment, T: @ScalarAsStd> @AsStd for Vector<@N, T, A> {
            type Output = T::@[OutputVec @N];

            #[inline(always)]
            fn @as_std(&self) -> Self::Output {
                T::@[to_vec @N](self.to_array().map(|t| t.@as_std()))
            }

            #[inline(always)]
            fn @from_std(val: Self::Output) -> Self {
                Self::from_array(T::@[from_vec @N](val).map(|t| T::@from_std(t)))
            }
        }
    }

    @for std in [140, 430], [T, TVec] in [
        [f32, Vec],
        [f64, DVec],
        [i32, IVec],
        [u32, UVec],
        [bool, BVec],
    ] {
        @let ScalarAsStd = @[ScalarAsStd @std];
        @let AsStd = @[AsStd @std];
        @let as_std = @[as_std @std];
        @let from_std = @[from_std @std];

        impl @ScalarAsStd for @T {
            @for N in 2..=4 {
                @let components = [x, y, z, w][0..@N];

                type @[OutputVec @N] = crevice::@[std @std]::@[@TVec @N];

                fn @[from_vec @N](vec: Self::@[OutputVec @N]) -> [Self::Output; @N] {
                    [@for n in 0..@N {
                        vec.@(@components[@n]),
                    }]
                }

                fn @[to_vec @N](value: [Self::Output; @N]) -> Self::@[OutputVec @N] {
                    Self::@[OutputVec @N] {
                        @for n in 0..@N {
                            @(@components[@n]): value[@n],
                        }
                    }
                }
            }
        }
    }
}
