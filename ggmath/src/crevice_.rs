//! Contains `Std140` and `Std430` ggmath types for use with `crevice`.
//! Crevice support does not include `Aabb` or `Quaternion` yet.

use crevice::{
    std140::{AsStd140, Std140},
    std430::{AsStd430, Std430},
};

use super::*;

macro_loop! {
    @for std_num in [140, 430] {
        @let ScalarAsStd = @[ScalarAsStd @std_num];
        @let ScalarMatAsStd = @[ScalarMatAsStd @std_num];
        @let AsStd = @[AsStd @std_num];
        @let Std = @[Std @std_num];
        @let std = @[std @std_num];
        @let as_std = @[as_std @std_num];
        @let from_std = @[from_std @std_num];

        #[cfg(feature = "vector")]
        #[doc = @(
            "A trait that allows vectors of `Self` to implement `" + @AsStd + "`.\n" +
            "\n" +
            "Vectors cannot implement `" + @AsStd + "` using only `T: " + @AsStd + "`.\n" +
            "This is because `T` has to be an `" + @Std + "` primitive with its own vector types."
        )]
        pub trait @ScalarAsStd: Scalar + @AsStd {
            @for N in 2..=4 {
                @let Vec = @[Vec @N];
                @let OutputVec = @[OutputVec @N];
                @let std_to_vec = @[@std _to_vec @N];
                @let std_from_vec = @[@std _from_vec @N];

                #[doc = @("The `" + @AsStd + "` output of `" + @Vec + "<Self>`.")]
                type @OutputVec: @Std;

                #[doc = @("Converts an array of `<Self as " + @AsStd + ">::Output` to `<" + @Vec + "<Self> as " + @AsStd + ">::Output`.")]
                fn @std_to_vec(value: [Self::Output; @N]) -> Self::@OutputVec;

                #[doc = @("Converts back from `<" + @Vec + "<Self> as " + @AsStd + ">::Output` to an array of `<Self as " + @AsStd + ">::Output`.")]
                fn @std_from_vec(vec: Self::@OutputVec) -> [Self::Output; @N];
            }
        }

        // Implement `AsStd` for vectors
        @for N in 2..=4 {
            @let Vec = @[Vec @N];
            @let OutputVec = @[OutputVec @N];
            @let std_to_vec = @[@std _to_vec @N];
            @let std_from_vec = @[@std _from_vec @N];

            #[cfg(feature = "vector")]
            impl<A: VecAlignment, T: @ScalarAsStd> @AsStd for Vector<@N, T, A> {
                type Output = T::@OutputVec;

                #[inline(always)]
                fn @as_std(&self) -> Self::Output {
                    T::@std_to_vec(self.to_array().map(|t| t.@as_std()))
                }

                #[inline(always)]
                fn @from_std(val: Self::Output) -> Self {
                    Self::from_array(T::@std_from_vec(val).map(|t| T::@from_std(t)))
                }
            }
        }

        // Implement `ScalarAsStd` for primitive types
        @for [T, TPrefix] in [
            [f32, ""],
            [f64, D],
            [i32, I],
            [u32, U],
            [bool, B],
        ] {
            #[cfg(feature = "vector")]
            impl @ScalarAsStd for @T {
                @for N in 2..=4 {
                    @let components = [x, y, z, w][0..@N];
                    @let std_to_vec = @[@std _to_vec @N];
                    @let std_from_vec = @[@std _from_vec @N];
                    @let OutputVec = @[OutputVec @N];
                    @let CreviceVec = @[@TPrefix Vec @N];

                    type @[OutputVec @N] = crevice::@std::@CreviceVec;

                    fn @std_to_vec(value: [Self::Output; @N]) -> Self::@OutputVec {
                        Self::@OutputVec {
                            @for n in 0..@N {
                                @(@components[@n]): value[@n],
                            }
                        }
                    }

                    fn @std_from_vec(vec: Self::@OutputVec) -> [Self::Output; @N] {
                        [@for n in 0..@N {
                            vec.@(@components[@n]),
                        }]
                    }
                }
            }
        }

        // Matrix

        #[cfg(feature = "matrix")]
        #[doc = @(
            "A trait that allows matrices of `Self` to implement `" + @AsStd + "`.\n" +
            "\n" +
            "Matrices cannot implement `" + @AsStd + "` using only `T: " + @AsStd + "`.\n" +
            "This is because `T` has to be an `" + @Std + "` primitive with its own matrix types."
        )]
        pub trait @ScalarMatAsStd: @ScalarAsStd {
            @for N in 2..=4 {
                @let Vec = @[Vec @N];
                @let OutputVec = @[OutputVec @N];
                @let std_to_vec = @[@std _to_vec @N];
                @let std_from_vec = @[@std _from_vec @N];

                @let Mat = @[Mat @N];
                @let OutputMat = @[OutputMat @N];
                @let std_to_mat = @[@std _to_mat @N];
                @let std_from_mat = @[@std _from_mat @N];

                #[doc = @("The `" + @AsStd + "` output of `" + @Mat + "<Self>`.")]
                type @OutputMat: @Std;

                #[doc = @("Converts an array of `<" + @Vec + "<Self> as " + @AsStd + ">::Output` to `<" + @Mat + "<Self> as " + @AsStd + ">::Output`.")]
                fn @std_to_mat(value: [Self::@OutputVec; @N]) -> Self::@OutputMat;

                #[doc = @("Converts back from `<" + @Mat + "<Self> as " + @AsStd + ">::Output` to an array of `<" + @Vec + "<Self> as " + @AsStd + ">::Output`.")]
                fn @std_from_mat(mat: Self::@OutputMat) -> [Self::@OutputVec; @N];
            }
        }

        // Implement `AsStd` for matrices
        @for N in 2..=4 {
            @let Vec = @[Vec @N];
            @let OutputVec = @[OutputVec @N];
            @let std_to_vec = @[@std _to_vec @N];
            @let std_from_vec = @[@std _from_vec @N];

            @let Mat = @[Mat @N];
            @let OutputMat = @[OutputMat @N];
            @let std_to_mat = @[@std _to_mat @N];
            @let std_from_mat = @[@std _from_mat @N];

            #[cfg(feature = "matrix")]
            impl<T: @ScalarMatAsStd, A: VecAlignment, M: MatMajorAxis> @AsStd for Matrix<@N, @N, T, A, M> {
                type Output = T::@OutputMat;

                #[inline(always)]
                fn @as_std(&self) -> Self::Output {
                    T::@std_to_mat(self.columns().map(|column| column.@as_std()))
                }

                #[inline(always)]
                fn @from_std(val: Self::Output) -> Self {
                    Self::from_columns(T::@std_from_mat(val).map(|column| Vector::<@N, T, A>::@from_std(column)))
                }
            }
        }

        // Implement `ScalarMatAsStd` for primitive types
        @for [T, TPrefix, std140_vec2_has_padding] in [
            [f32, "", true],
            [f64, D, false],
        ] {
            #[cfg(feature = "matrix")]
            impl @ScalarMatAsStd for @T {
                @for N in 2..=4 {
                    @let components = [x, y, z, w][0..@N];
                    @let std_to_vec = @[@std _to_vec @N];
                    @let std_from_vec = @[@std _from_vec @N];
                    @let OutputVec = @[OutputVec @N];
                    @let CreviceVec = @[@TPrefix Vec @N];
                    @let std_to_mat = @[@std _to_mat @N];
                    @let std_from_mat = @[@std _from_mat @N];
                    @let OutputMat = @[OutputMat @N];
                    @let CreviceMat = @[@TPrefix Mat @N];
                    @let is_std140 = @std_num == 140;

                    type @OutputMat = crevice::@std::@CreviceMat;

                    fn @std_to_mat(value: [Self::@OutputVec; @N]) -> Self::@OutputMat {
                        Self::@OutputMat {
                            @for [c_idx, c] in @components.enumerate() {
                                @c: value[@c_idx].@as_std(),

                                @if @N == 3 || @N == 2 && @is_std140 && @std140_vec2_has_padding {
                                    @[_pad_ @c]: Default::default(),
                                }
                            }
                        }
                    }

                    fn @std_from_mat(mat: Self::@OutputMat) -> [Self::@OutputVec; @N] {
                        [@for c in @components {
                            mat.@c,
                        }]
                    }
                }
            }
        }
    }
}
