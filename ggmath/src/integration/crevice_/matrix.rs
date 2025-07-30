use crevice::{
    std140::{AsStd140, Std140},
    std430::{AsStd430, Std430},
};

use super::*;

pub trait ScalarMatAsStd140: Scalar + AsStd140 + ScalarAsStd140 {
    type OutputMat2: Std140;
    type OutputMat3: Std140;
    type OutputMat4: Std140;

    fn to_mat2(value: [Self::OutputVec2; 2]) -> Self::OutputMat2;
    fn to_mat3(value: [Self::OutputVec3; 3]) -> Self::OutputMat3;
    fn to_mat4(value: [Self::OutputVec4; 4]) -> Self::OutputMat4;

    fn from_mat2(mat: Self::OutputMat2) -> [Self::OutputVec2; 2];
    fn from_mat3(mat: Self::OutputMat3) -> [Self::OutputVec3; 3];
    fn from_mat4(mat: Self::OutputMat4) -> [Self::OutputVec4; 4];
}

pub trait ScalarMatAsStd430: Scalar + AsStd430 + ScalarAsStd430 {
    type OutputMat2: Std430;
    type OutputMat3: Std430;
    type OutputMat4: Std430;

    fn to_mat2(value: [Self::OutputVec2; 2]) -> Self::OutputMat2;
    fn to_mat3(value: [Self::OutputVec3; 3]) -> Self::OutputMat3;
    fn to_mat4(value: [Self::OutputVec4; 4]) -> Self::OutputMat4;

    fn from_mat2(mat: Self::OutputMat2) -> [Self::OutputVec2; 2];
    fn from_mat3(mat: Self::OutputMat3) -> [Self::OutputVec3; 3];
    fn from_mat4(mat: Self::OutputMat4) -> [Self::OutputVec4; 4];
}

macro_loop! {
    @for N in 2..=4, std in [140, 430] {
        @let ScalarMatAsStd = @[ScalarMatAsStd @std];
        @let AsStd = @[AsStd @std];
        @let as_std = @[as_std @std];
        @let from_std = @[from_std @std];

        impl<T: @ScalarMatAsStd, A: VecAlignment, M: MatrixMajorAxis> @AsStd for Matrix<@N, @N, T, A, M> {
            type Output = T::@[OutputMat @N];

            #[inline(always)]
            fn @as_std(&self) -> Self::Output {
                T::@[to_mat @N](self.columns().map(|column| column.@as_std()))
            }

            #[inline(always)]
            fn @from_std(val: Self::Output) -> Self {
                Self::from_columns(T::@[from_mat @N](val).map(|column| Vector::<@N, T, A>::@from_std(column)))
            }
        }
    }

    @for [T, TMat, std140_vec2_has_padding] in [
        [f32, Mat, true],
        [f64, DMat, false],
    ], [std, vec2_has_padding] in [
        [140, @std140_vec2_has_padding],
        [430, false],
    ] {
        @let ScalarAsStd = @[ScalarAsStd @std];
        @let ScalarMatAsStd = @[ScalarMatAsStd @std];
        @let AsStd = @[AsStd @std];
        @let as_std = @[as_std @std];
        @let from_std = @[from_std @std];

        impl @ScalarMatAsStd for @T {
            @for N in 2..=4 {
                @let components = [x, y, z, w][0..@N];

                type @[OutputMat @N] = crevice::@[std @std]::@[@TMat @N];

                fn @[to_mat @N](value: [Self::@[OutputVec @N]; @N]) -> Self::@[OutputMat @N] {
                    Self::@[OutputMat @N] {
                        @for [c_idx, c] in @components.enumerate() {
                            @c: value[@c_idx].@as_std(),

                            @if @N == 3 || @N == 2 && @vec2_has_padding {
                                @[_pad_ @c]: Default::default(),
                            }
                        }
                    }
                }

                fn @[from_mat @N](mat: Self::@[OutputMat @N]) -> [Self::@[OutputVec @N]; @N] {
                    [@for c in @components {
                        mat.@c,
                    }]
                }
            }
        }
    }
}
