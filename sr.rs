#![feature(prelude_import)]
//! Generic-Graphics-Math with internal optimized SIMD.
//!
//! - Fully generic (Vector<Len, Scalar, Alignment>...).
//! - Optimized with SIMD internally.
//! - Simple API (FVec2...).
//! - Both column-major and row-major matricies.
//! - Num traits (FloatScalar...).
//! - Optimal for GPU structs.
//! - Optional additional types (Rect, Ray...).
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod construct {
    //! base traits for mathamatical types.
    use std::fmt::{Debug, Display};
    /// The base trait for mathamatical types.
    ///
    /// Includes core Rust traits like [Copy] and [Display].
    /// Is automatically implemented for types that implement its supertraits.
    pub trait Construct: InnerConstruct + PartialEq + Display {}
    /// The base trait for inner mathamatical types.
    ///
    /// Includes core Rust traits needed for inner data like [Copy] and [Debug], but not outer traits like [Display].
    /// - Anything that implements [Construct] also implements [InnerConstruct].
    pub trait InnerConstruct: Sized + Send + Sync + Copy + Debug {}
    impl<T: InnerConstruct + PartialEq + Display> Construct for T {}
    impl<T: Sized + Send + Sync + Copy + Debug> InnerConstruct for T {}
}
pub mod scalar {
    //! Scalars are mathamatical types that have magnitude but no direction.
    //! - [f32] and [bool] are scalars.
    //! - [Vec3](crate::vec::Vec3) is not a scalar.
    mod primitive_impls {
        mod bool {
            use ggmath_proc_macros::inner_vecs;
            use crate::{self as ggmath, scalar::Scalar};
            mod inner_vecs_6861707285520076819 {
                use super::*;
                unsafe impl ggmath::vector::inner::ScalarInnerVecs for bool {
                    type InnerAlignedVec2 = InnerAlignedVec2;
                    type InnerAlignedVec4 = InnerAlignedVec4;
                }
                #[repr(align(2))]
                pub struct InnerAlignedVec2([bool; 2]);
                #[automatically_derived]
                impl ::core::fmt::Debug for InnerAlignedVec2 {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "InnerAlignedVec2",
                            &&self.0,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for InnerAlignedVec2 {
                    #[inline]
                    fn clone(&self) -> InnerAlignedVec2 {
                        let _: ::core::clone::AssertParamIsClone<[bool; 2]>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for InnerAlignedVec2 {}
                #[repr(align(4))]
                pub struct InnerAlignedVec4([bool; 4]);
                #[automatically_derived]
                impl ::core::fmt::Debug for InnerAlignedVec4 {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "InnerAlignedVec4",
                            &&self.0,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for InnerAlignedVec4 {
                    #[inline]
                    fn clone(&self) -> InnerAlignedVec4 {
                        let _: ::core::clone::AssertParamIsClone<[bool; 4]>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for InnerAlignedVec4 {}
                const _: () = if !(size_of::<bool>() == 1) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "the provided size for bool: 1 bytes, is not its size",
                            ),
                        );
                    }
                };
                const _: () = if !(size_of::<InnerAlignedVec2>() == 1 * 2) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                            ),
                        );
                    }
                };
                const _: () = if !(align_of::<InnerAlignedVec2>() == 1 * 2) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                            ),
                        );
                    }
                };
                const _: () = if !(size_of::<InnerAlignedVec4>() == 1 * 4) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                            ),
                        );
                    }
                };
                const _: () = if !(align_of::<InnerAlignedVec4>() == 1 * 4) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                            ),
                        );
                    }
                };
            }
            impl Scalar for bool {}
        }
        mod floats {
            mod f32 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_14215670250043581766 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for f32 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(8))]
                    pub struct InnerAlignedVec2([f32; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[f32; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(16))]
                    pub struct InnerAlignedVec4([f32; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[f32; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<f32>() == 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for f32: 4 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for f32 {}
            }
            mod f64 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_11668025185570632548 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for f64 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([f64; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[f64; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([f64; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[f64; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<f64>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for f64: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for f64 {}
            }
        }
        mod signed_ints {
            mod i128 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_3887899319199191083 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for i128 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(32))]
                    pub struct InnerAlignedVec2([i128; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i128; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(64))]
                    pub struct InnerAlignedVec4([i128; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i128; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i128>() == 16) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i128: 16 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 16 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 16 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 16 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 16 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for i128 {}
            }
            mod i16 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_10900643894459623307 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for i16 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(4))]
                    pub struct InnerAlignedVec2([i16; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i16; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(8))]
                    pub struct InnerAlignedVec4([i16; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i16; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i16>() == 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i16: 2 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 2 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 2 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 2 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 2 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for i16 {}
            }
            mod i32 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_9840377960296536376 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for i32 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(8))]
                    pub struct InnerAlignedVec2([i32; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i32; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(16))]
                    pub struct InnerAlignedVec4([i32; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i32; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i32>() == 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i32: 4 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for i32 {}
            }
            mod i64 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_17059750471201740779 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for i64 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([i64; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i64; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([i64; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i64; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i64>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i64: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for i64 {}
            }
            mod i8 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_15064204760916713565 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for i8 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(2))]
                    pub struct InnerAlignedVec2([i8; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i8; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(4))]
                    pub struct InnerAlignedVec4([i8; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i8; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i8>() == 1) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i8: 1 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 1 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 1 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 1 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 1 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for i8 {}
            }
            mod isize {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_1706441205137726090 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for isize {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([isize; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[isize; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([isize; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[isize; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<isize>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for isize: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for isize {}
            }
        }
        mod unsigned_ints {
            mod u128 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_7026937442784286838 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for u128 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(32))]
                    pub struct InnerAlignedVec2([u128; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u128; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(64))]
                    pub struct InnerAlignedVec4([u128; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u128; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u128>() == 16) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u128: 16 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 16 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 16 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 16 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 16 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for u128 {}
            }
            mod u16 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_14807517481747284577 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for u16 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(4))]
                    pub struct InnerAlignedVec2([u16; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u16; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(8))]
                    pub struct InnerAlignedVec4([u16; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u16; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u16>() == 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u16: 2 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 2 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 2 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 2 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 2 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for u16 {}
            }
            mod u32 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_17805753030591285685 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for u32 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(8))]
                    pub struct InnerAlignedVec2([u32; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u32; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(16))]
                    pub struct InnerAlignedVec4([u32; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u32; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u32>() == 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u32: 4 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for u32 {}
            }
            mod u64 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_2583076820244694339 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for u64 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([u64; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u64; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([u64; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u64; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u64>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u64: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for u64 {}
            }
            mod u8 {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_7395965558579549491 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for u8 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(2))]
                    pub struct InnerAlignedVec2([u8; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u8; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(4))]
                    pub struct InnerAlignedVec4([u8; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u8; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u8>() == 1) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u8: 1 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 1 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 1 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 1 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 1 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for u8 {}
            }
            mod usize {
                use ggmath_proc_macros::inner_vecs;
                use crate::{self as ggmath, scalar::Scalar};
                mod inner_vecs_3088197000634950847 {
                    use super::*;
                    unsafe impl ggmath::vector::inner::ScalarInnerVecs for usize {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([usize; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[usize; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([usize; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[usize; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<usize>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for usize: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
                impl Scalar for usize {}
            }
        }
    }
    pub use crate::vector::interfaces::scalar_traits::*;
    pub use ggmath_proc_macros::scalar_aliases;
}
pub mod vector {
    //! Staticly-lengthed vectors of [scalars](scalar) with lengths between 2 and 4.
    pub mod alignment {
        use super::*;
        /// Sealed trait for the alignment rules of a vector.
        /// - Doesn't affect the outer vector API, just the inner implementation.
        /// - Use the [```VecN```]```<N, T>``` type alias to use the default storage.
        ///
        /// Implemented by ```VecAligned``` and ```VecPacked```, each have different uses and advantages.
        /// To understand them first understand [Rust type-layout](<https://doc.rust-lang.org/reference/type-layout.html>).
        ///
        /// ### [VecPacked]
        ///
        /// ensures that the vector has the same type-layout as ```[T; N]```.
        /// ```
        /// // VecNP<N, T> = Vector<N, T, VecPacked>
        /// assert_eq!(
        ///     size_of::<VecNP<N, T>>(),
        ///     size_of::<T>() * N
        /// );
        /// assert_eq!(
        ///     align_of::<VecNP<N, T>>(),
        ///     align_of::<T>()
        /// );
        /// ```
        ///
        /// - use [```VecNP```]<N, T>
        ///
        /// ### [VecAligned]
        ///
        /// ensures that the vector has the next alignment from ```[T; N]```'s size, and a size equal to the alignment.
        /// ```
        /// // VecN<N, T> = Vector<N, T, VecAligned>
        /// assert_eq!(
        ///     size_of::<VecN<N, T>>(),
        ///     (size_of::<T>() * N).next_power_of_two()
        /// );
        /// assert_eq!(
        ///     align_of::<VecN<N, T>>(),
        ///     (size_of::<T>() * N).next_power_of_two()
        /// );
        /// ```
        ///
        /// - This means that the size and alignment of ```Vec3<T>``` is the same as ```Vec4<T>```'s.
        /// - This means that ```size/align_of<Vec2> = size_of<T> * 2```, and ```size/align_of<Vec3> = size/align_of<Vec4> = size_of<T> * 4```.
        ///
        /// - use [```VecN```]<N, T>
        ///
        /// ## How to pick
        ///
        /// Sometimes the ```VecAligned``` type-layout is required.
        /// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
        /// When neither storage is required, choose based on their performance advantages/disadvantages:
        ///
        /// - ```VecAligned``` results in faster computations because of SIMD registers which require the extra alignment.
        /// - ```VecAligned``` may take more space because of the larger alignment, and that a ```Vec3``` always takes the space of a ```Vec4```.
        /// - ```VecPacked``` takes less space because of the minimal alignment and the lack of padding.
        /// - ```VecPacked``` may result in slower computations because of the SIMD register's requirements.
        ///
        /// Basically only use ```VecPacked``` (```VecNP```) when storing large arrays of vectors that you don't perform much computation on.
        /// On any other case use ```VecAligned``` (```VecN```, The default).
        #[allow(private_bounds)]
        pub trait VecAlignment: alignment_seal::VecAlignment + interfaces::VecAlignmentInterfaces<
                2,
            > + interfaces::VecAlignmentInterfaces<
                3,
            > + interfaces::VecAlignmentInterfaces<4> {}
        /// Vector inner storage that ensures that the vector has the next alignment from ```[T; N]```'s size, and a size equal to the alignment.
        /// ```
        /// // VecN<N, T> = Vector<N, T, VecAligned>
        /// assert_eq!(
        ///     size_of::<VecN<N, T>>(),
        ///     (size_of::<T>() * N).next_power_of_two()
        /// );
        /// assert_eq!(
        ///     align_of::<VecN<N, T>>(),
        ///     (size_of::<T>() * N).next_power_of_two()
        /// );
        /// ```
        ///
        /// - This means that the size and alignment of ```Vec3<T>``` is the same as ```Vec4<T>```'s.
        /// - This means that ```size/align_of<Vec2> = size_of<T> * 2```, and ```size/align_of<Vec3> = size/align_of<Vec4> = size_of<T> * 4```.
        ///
        /// ## When to use
        ///
        /// Sometimes the ```VecAligned``` type-layout is required.
        /// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
        /// When not required, choose based on performance advantages/disadvantages:
        ///
        /// - results in faster computations than ```VecPacked``` because of SIMD registers which require the extra alignment.
        /// - may take more space than ```VecPacked``` because of the larger alignment, and that a ```Vec3``` always takes the space of a ```Vec4```.
        ///
        /// Always recommended except for when storing large arrays of vectors that you don't perform much computation on.
        pub struct VecAligned;
        impl alignment_seal::VecAlignment for VecAligned {}
        impl VecAlignment for VecAligned {}
        /// Vector inner storage that ensures that the vector has the same type-layout as ```[T; N]```.
        /// ```
        /// // VecNP<N, T> = Vector<N, T, VecPacked>
        /// assert_eq!(
        ///     size_of::<VecNP<N, T>>(),
        ///     size_of::<T>() * N
        /// );
        /// assert_eq!(
        ///     align_of::<VecNP<N, T>>(),
        ///     align_of::<T>()
        /// );
        /// ```
        ///
        /// ## When to use
        ///
        /// Sometimes the ```VecAligned``` type-layout is required.
        /// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
        /// When ```VecAligned``` isn't required, choose based on performance advantages/disadvantages:
        ///
        /// - takes less space than ```VecAligned``` because of the minimal alignment and the lack of padding.
        /// - may result in slower computations than ```VecAligned``` because of the SIMD register's requirements.
        ///
        /// Only recommended when storing large arrays of vectors that you don't perform much computation on.
        pub struct VecPacked;
        impl alignment_seal::VecAlignment for VecPacked {}
        impl VecAlignment for VecPacked {}
        pub(super) mod alignment_seal {
            use super::*;
            pub trait VecAlignment: inner::VecAlignmentInnerVecs {}
        }
    }
    pub mod inner {
        //! Behaviour for selecting an inner-vector type based on a vector's length, scalar, and storage.
        //!
        //!
        use super::*;
        /// The type of the inner-value inside a vector
        pub type InnerVector<const N: usize, T, S> = <S as VecAlignmentInnerVecs>::InnerVec<
            N,
            T,
        >;
        /// Scalar supertrait that specifies inner-types for vectors that can't be declared generically.
        ///
        /// - Unsafe to implement manually because the implementation is expected to comply with type-layout guarentees.
        /// Instead, implement using [```aligned_vecs```].
        pub unsafe trait ScalarInnerVecs: Construct {
            /// Inner-type for ```VecAligned``` Vec2s.
            /// - Guarenteed: ```size = align = size_of::<T>().next_power_of_two() * 2```
            type InnerAlignedVec2: InnerConstruct;
            /// Inner-type for ```VecAligned``` Vec4s and Vec3s.
            /// - Guarenteed: ```size = align = size_of::<T>().next_power_of_two() * 4```
            type InnerAlignedVec4: InnerConstruct;
        }
        pub use ggmath_proc_macros::inner_vecs;
        #[doc(hidden)]
        #[allow(private_bounds)]
        pub trait VecLenInnerVec: Seal {
            type InnerAlignedVec<T: ScalarInnerVecs>: InnerConstruct;
        }
        impl Seal for ScalarCount<2> {}
        impl Seal for ScalarCount<4> {}
        impl Seal for ScalarCount<3> {}
        impl VecLenInnerVec for ScalarCount<2> {
            type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec2;
        }
        impl VecLenInnerVec for ScalarCount<3> {
            type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec4;
        }
        impl VecLenInnerVec for ScalarCount<4> {
            type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec4;
        }
        #[doc(hidden)]
        #[allow(private_bounds)]
        pub trait VecAlignmentInnerVecs: Seal {
            type InnerVec<const N: usize, T: ScalarInnerVecs>: InnerConstruct
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        impl Seal for VecPacked {}
        impl VecAlignmentInnerVecs for VecPacked {
            type InnerVec<const N: usize, T: ScalarInnerVecs> = [T; N]
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        impl Seal for VecAligned {}
        impl VecAlignmentInnerVecs for VecAligned {
            type InnerVec<const N: usize, T: ScalarInnerVecs> = <ScalarCount<
                N,
            > as VecLenInnerVec>::InnerAlignedVec<T>
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        trait Seal: Sized {}
    }
    pub mod length {
        use super::*;
        /// Sealed trait for ```ScalarCount```s that are valid as vector lengths.
        ///
        /// Vectors can only have lengths 2, 3, or 4 because internally vector fns have differently optimized implementations for each length.
        ///
        /// This trait is implemented by ```ScalarCount<2/3/4>``` and is used to validate that a generic vector length is either 2, 3, or 4 with ```where ScalarCount<N>: VecLen<N>```.
        ///
        /// # Examples
        /// ```
        /// // Line is generic over dimension count.
        /// use ggmath::vec::*;
        ///
        /// struct Line<const N: usize>
        /// where
        ///     ScalarCount<N>: VecLen<N>,
        /// {
        ///     start: VecN<N, f32>,
        ///     end: VecN<N, f32>,
        /// }
        ///
        /// type Line2D = Line<2>;
        /// type Line3D = Line<3>;
        /// type Line4D = Line<4>;
        ///
        /// // Using an N that isn't 2, 3, or 4. Wont work.
        /// struct InvalidStruct {
        ///     line: Line<6>, // ERROR: the trait bound `ScalarCount<6>: VecLen<6>` is not satisfied
        /// }
        /// ```
        #[allow(private_bounds)]
        pub trait VecLen<
            const N: usize,
        >: Seal + inner::VecLenInnerVec + interfaces::VecLenInterfaces<N>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        /// Count of scalars that may or may not be a [```VecLen```].
        ///
        /// Vectors can only have lengths 2, 3, or 4 because internally vector fns have differently optimized implementations for each length.
        ///
        /// Only ```ScalarCount<2/3/4>``` implements ```VecLen```.
        /// this is used to validate that a generic vector length is either 2, 3, or 4 with ```where ScalarCount<N>: VecLen<N>```.
        ///
        /// # Examples
        /// ```
        /// // Line is generic over dimension count.
        /// struct Line<const N: usize>
        /// where
        ///     ScalarCount<N>: VecLen<N>,
        /// {
        ///     start: FVecN<N>,
        ///     end: FVecN<N>,
        /// }
        ///
        /// type Line2D = Line<2>;
        /// type Line3D = Line<3>;
        /// type Line4D = Line<4>;
        ///
        /// // Using an N that isn't 2, 3, or 4. Wont work.
        /// struct InvalidStruct {
        ///     line: Line<6>, // ERROR: the trait bound `ScalarCount<6>: VecLen<6>` is not satisfied
        /// }
        /// ```
        pub struct ScalarCount<const VALUE: usize>;
        impl Seal for ScalarCount<2> {}
        impl Seal for ScalarCount<4> {}
        impl Seal for ScalarCount<3> {}
        impl VecLen<2> for ScalarCount<2> {}
        impl VecLen<3> for ScalarCount<3> {}
        impl VecLen<4> for ScalarCount<4> {}
        trait Seal {}
    }
    pub mod into_vec {
        use super::*;
        pub fn vector<const N: usize, T: Scalar, A: VecAlignment>(
            value: impl IntoVector<N, T>,
        ) -> Vector<N, T, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            value.into_vec()
        }
        pub fn vec2<T: Scalar>(value: impl IntoVector<2, T>) -> Vec2<T> {
            value.into_vec()
        }
        pub fn vec3<T: Scalar>(value: impl IntoVector<3, T>) -> Vec3<T> {
            value.into_vec()
        }
        pub fn vec4<T: Scalar>(value: impl IntoVector<4, T>) -> Vec4<T> {
            value.into_vec()
        }
        pub fn vec2p<T: Scalar>(value: impl IntoVector<2, T>) -> Vec2P<T> {
            value.into_vec()
        }
        pub fn vec3p<T: Scalar>(value: impl IntoVector<3, T>) -> Vec3P<T> {
            value.into_vec()
        }
        pub fn vec4p<T: Scalar>(value: impl IntoVector<4, T>) -> Vec4P<T> {
            value.into_vec()
        }
        pub trait IntoVector<const N: usize, T: Scalar>: Sized
        where
            ScalarCount<N>: VecLen<N>,
        {
            fn into_vec_array(self) -> [T; N];
            fn into_vec<A: VecAlignment>(self) -> Vector<N, T, A> {
                Vector::from_array(self.into_vec_array())
            }
        }
        impl<const N: usize, T: Scalar, A: VecAlignment> IntoVector<N, T>
        for Vector<N, T, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            fn into_vec_array(self) -> [T; N] {
                self.into_array()
            }
        }
        impl<T: Scalar> IntoVector<2, T> for (T, T) {
            fn into_vec_array(self) -> [T; 2] {
                [self.0, self.1]
            }
        }
        impl<T: Scalar> IntoVector<3, T> for (T, T, T) {
            fn into_vec_array(self) -> [T; 3] {
                [self.0, self.1, self.2]
            }
        }
        impl<T: Scalar, A: VecAlignment> IntoVector<3, T> for (Vector<2, T, A>, T) {
            fn into_vec_array(self) -> [T; 3] {
                [self.0.x(), self.0.y(), self.1]
            }
        }
        impl<T: Scalar, A: VecAlignment> IntoVector<3, T> for (T, Vector<2, T, A>) {
            fn into_vec_array(self) -> [T; 3] {
                [self.0, self.1.x(), self.1.y()]
            }
        }
        impl<T: Scalar> IntoVector<4, T> for (T, T, T, T) {
            fn into_vec_array(self) -> [T; 4] {
                [self.0, self.1, self.2, self.3]
            }
        }
        impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (Vector<2, T, A>, T, T) {
            fn into_vec_array(self) -> [T; 4] {
                [self.0.x(), self.0.y(), self.1, self.2]
            }
        }
        impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, Vector<2, T, A>, T) {
            fn into_vec_array(self) -> [T; 4] {
                [self.0, self.1.x(), self.1.y(), self.2]
            }
        }
        impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, T, Vector<2, T, A>) {
            fn into_vec_array(self) -> [T; 4] {
                [self.0, self.1, self.2.x(), self.2.y()]
            }
        }
        impl<T: Scalar, A0: VecAlignment, A1: VecAlignment> IntoVector<4, T>
        for (Vector<2, T, A0>, Vector<2, T, A1>) {
            fn into_vec_array(self) -> [T; 4] {
                [self.0.x(), self.0.y(), self.1.x(), self.1.y()]
            }
        }
        impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (Vector<3, T, A>, T) {
            fn into_vec_array(self) -> [T; 4] {
                [self.0.x(), self.0.y(), self.0.z(), self.1]
            }
        }
        impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, Vector<3, T, A>) {
            fn into_vec_array(self) -> [T; 4] {
                [self.0, self.1.x(), self.1.y(), self.1.z()]
            }
        }
    }
    pub mod swizzle_wrappers {
        use std::mem::transmute;
        use ggmath_proc_macros::swizzles;
        use super::*;
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn x(self) -> T {
                unsafe { self.get_unchecked(0) }
            }
            #[inline(always)]
            pub fn y(self) -> T {
                unsafe { self.get_unchecked(1) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn xx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 0]) }
            }
            #[inline(always)]
            pub fn xy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 1]) }
            }
            #[inline(always)]
            pub fn yx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 0]) }
            }
            #[inline(always)]
            pub fn yy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 1]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn xxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 0]) }
            }
            #[inline(always)]
            pub fn xxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 1]) }
            }
            #[inline(always)]
            pub fn xyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 0]) }
            }
            #[inline(always)]
            pub fn xyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 1]) }
            }
            #[inline(always)]
            pub fn yxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 0]) }
            }
            #[inline(always)]
            pub fn yxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 1]) }
            }
            #[inline(always)]
            pub fn yyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 0]) }
            }
            #[inline(always)]
            pub fn yyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 1]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn xxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn xxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn xxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn xxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn xyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn xyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn xyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn xyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 1]) }
            }
            #[inline(always)]
            pub fn yxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn yxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn yxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn yxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn yyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn yyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn yyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn yyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 1]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn x(self) -> T {
                unsafe { self.get_unchecked(0) }
            }
            #[inline(always)]
            pub fn y(self) -> T {
                unsafe { self.get_unchecked(1) }
            }
            #[inline(always)]
            pub fn z(self) -> T {
                unsafe { self.get_unchecked(2) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn xx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 0]) }
            }
            #[inline(always)]
            pub fn xy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 1]) }
            }
            #[inline(always)]
            pub fn xz(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 2]) }
            }
            #[inline(always)]
            pub fn yx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 0]) }
            }
            #[inline(always)]
            pub fn yy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 1]) }
            }
            #[inline(always)]
            pub fn yz(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 2]) }
            }
            #[inline(always)]
            pub fn zx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([2, 0]) }
            }
            #[inline(always)]
            pub fn zy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([2, 1]) }
            }
            #[inline(always)]
            pub fn zz(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([2, 2]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn xxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 0]) }
            }
            #[inline(always)]
            pub fn xxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 1]) }
            }
            #[inline(always)]
            pub fn xxz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 2]) }
            }
            #[inline(always)]
            pub fn xyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 0]) }
            }
            #[inline(always)]
            pub fn xyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 1]) }
            }
            #[inline(always)]
            pub fn xyz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 2]) }
            }
            #[inline(always)]
            pub fn xzx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 2, 0]) }
            }
            #[inline(always)]
            pub fn xzy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 2, 1]) }
            }
            #[inline(always)]
            pub fn xzz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 2, 2]) }
            }
            #[inline(always)]
            pub fn yxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 0]) }
            }
            #[inline(always)]
            pub fn yxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 1]) }
            }
            #[inline(always)]
            pub fn yxz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 2]) }
            }
            #[inline(always)]
            pub fn yyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 0]) }
            }
            #[inline(always)]
            pub fn yyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 1]) }
            }
            #[inline(always)]
            pub fn yyz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 2]) }
            }
            #[inline(always)]
            pub fn yzx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 2, 0]) }
            }
            #[inline(always)]
            pub fn yzy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 2, 1]) }
            }
            #[inline(always)]
            pub fn yzz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 2, 2]) }
            }
            #[inline(always)]
            pub fn zxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 0, 0]) }
            }
            #[inline(always)]
            pub fn zxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 0, 1]) }
            }
            #[inline(always)]
            pub fn zxz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 0, 2]) }
            }
            #[inline(always)]
            pub fn zyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 1, 0]) }
            }
            #[inline(always)]
            pub fn zyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 1, 1]) }
            }
            #[inline(always)]
            pub fn zyz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 1, 2]) }
            }
            #[inline(always)]
            pub fn zzx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 2, 0]) }
            }
            #[inline(always)]
            pub fn zzy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 2, 1]) }
            }
            #[inline(always)]
            pub fn zzz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 2, 2]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn xxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn xxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn xxxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 2]) }
            }
            #[inline(always)]
            pub fn xxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn xxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn xxyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 2]) }
            }
            #[inline(always)]
            pub fn xxzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 2, 0]) }
            }
            #[inline(always)]
            pub fn xxzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 2, 1]) }
            }
            #[inline(always)]
            pub fn xxzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 2, 2]) }
            }
            #[inline(always)]
            pub fn xyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn xyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn xyxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 2]) }
            }
            #[inline(always)]
            pub fn xyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn xyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 1]) }
            }
            #[inline(always)]
            pub fn xyyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 2]) }
            }
            #[inline(always)]
            pub fn xyzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 2, 0]) }
            }
            #[inline(always)]
            pub fn xyzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 2, 1]) }
            }
            #[inline(always)]
            pub fn xyzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 2, 2]) }
            }
            #[inline(always)]
            pub fn xzxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 0, 0]) }
            }
            #[inline(always)]
            pub fn xzxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 0, 1]) }
            }
            #[inline(always)]
            pub fn xzxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 0, 2]) }
            }
            #[inline(always)]
            pub fn xzyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 1, 0]) }
            }
            #[inline(always)]
            pub fn xzyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 1, 1]) }
            }
            #[inline(always)]
            pub fn xzyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 1, 2]) }
            }
            #[inline(always)]
            pub fn xzzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 2, 0]) }
            }
            #[inline(always)]
            pub fn xzzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 2, 1]) }
            }
            #[inline(always)]
            pub fn xzzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 2, 2]) }
            }
            #[inline(always)]
            pub fn yxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn yxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn yxxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 2]) }
            }
            #[inline(always)]
            pub fn yxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn yxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn yxyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 2]) }
            }
            #[inline(always)]
            pub fn yxzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 2, 0]) }
            }
            #[inline(always)]
            pub fn yxzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 2, 1]) }
            }
            #[inline(always)]
            pub fn yxzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 2, 2]) }
            }
            #[inline(always)]
            pub fn yyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn yyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn yyxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 2]) }
            }
            #[inline(always)]
            pub fn yyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn yyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 1]) }
            }
            #[inline(always)]
            pub fn yyyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 2]) }
            }
            #[inline(always)]
            pub fn yyzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 2, 0]) }
            }
            #[inline(always)]
            pub fn yyzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 2, 1]) }
            }
            #[inline(always)]
            pub fn yyzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 2, 2]) }
            }
            #[inline(always)]
            pub fn yzxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 0, 0]) }
            }
            #[inline(always)]
            pub fn yzxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 0, 1]) }
            }
            #[inline(always)]
            pub fn yzxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 0, 2]) }
            }
            #[inline(always)]
            pub fn yzyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 1, 0]) }
            }
            #[inline(always)]
            pub fn yzyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 1, 1]) }
            }
            #[inline(always)]
            pub fn yzyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 1, 2]) }
            }
            #[inline(always)]
            pub fn yzzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 2, 0]) }
            }
            #[inline(always)]
            pub fn yzzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 2, 1]) }
            }
            #[inline(always)]
            pub fn yzzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 2, 2]) }
            }
            #[inline(always)]
            pub fn zxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn zxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn zxxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 0, 2]) }
            }
            #[inline(always)]
            pub fn zxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn zxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn zxyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 1, 2]) }
            }
            #[inline(always)]
            pub fn zxzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 2, 0]) }
            }
            #[inline(always)]
            pub fn zxzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 2, 1]) }
            }
            #[inline(always)]
            pub fn zxzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 2, 2]) }
            }
            #[inline(always)]
            pub fn zyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn zyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn zyxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 0, 2]) }
            }
            #[inline(always)]
            pub fn zyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn zyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 1, 1]) }
            }
            #[inline(always)]
            pub fn zyyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 1, 2]) }
            }
            #[inline(always)]
            pub fn zyzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 2, 0]) }
            }
            #[inline(always)]
            pub fn zyzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 2, 1]) }
            }
            #[inline(always)]
            pub fn zyzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 2, 2]) }
            }
            #[inline(always)]
            pub fn zzxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 0, 0]) }
            }
            #[inline(always)]
            pub fn zzxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 0, 1]) }
            }
            #[inline(always)]
            pub fn zzxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 0, 2]) }
            }
            #[inline(always)]
            pub fn zzyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 1, 0]) }
            }
            #[inline(always)]
            pub fn zzyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 1, 1]) }
            }
            #[inline(always)]
            pub fn zzyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 1, 2]) }
            }
            #[inline(always)]
            pub fn zzzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 2, 0]) }
            }
            #[inline(always)]
            pub fn zzzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 2, 1]) }
            }
            #[inline(always)]
            pub fn zzzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 2, 2]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn x(self) -> T {
                unsafe { self.get_unchecked(0) }
            }
            #[inline(always)]
            pub fn y(self) -> T {
                unsafe { self.get_unchecked(1) }
            }
            #[inline(always)]
            pub fn z(self) -> T {
                unsafe { self.get_unchecked(2) }
            }
            #[inline(always)]
            pub fn w(self) -> T {
                unsafe { self.get_unchecked(3) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn xx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 0]) }
            }
            #[inline(always)]
            pub fn xy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 1]) }
            }
            #[inline(always)]
            pub fn xz(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 2]) }
            }
            #[inline(always)]
            pub fn xw(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([0, 3]) }
            }
            #[inline(always)]
            pub fn yx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 0]) }
            }
            #[inline(always)]
            pub fn yy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 1]) }
            }
            #[inline(always)]
            pub fn yz(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 2]) }
            }
            #[inline(always)]
            pub fn yw(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([1, 3]) }
            }
            #[inline(always)]
            pub fn zx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([2, 0]) }
            }
            #[inline(always)]
            pub fn zy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([2, 1]) }
            }
            #[inline(always)]
            pub fn zz(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([2, 2]) }
            }
            #[inline(always)]
            pub fn zw(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([2, 3]) }
            }
            #[inline(always)]
            pub fn wx(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([3, 0]) }
            }
            #[inline(always)]
            pub fn wy(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([3, 1]) }
            }
            #[inline(always)]
            pub fn wz(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([3, 2]) }
            }
            #[inline(always)]
            pub fn ww(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([3, 3]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn xxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 0]) }
            }
            #[inline(always)]
            pub fn xxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 1]) }
            }
            #[inline(always)]
            pub fn xxz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 2]) }
            }
            #[inline(always)]
            pub fn xxw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 0, 3]) }
            }
            #[inline(always)]
            pub fn xyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 0]) }
            }
            #[inline(always)]
            pub fn xyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 1]) }
            }
            #[inline(always)]
            pub fn xyz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 2]) }
            }
            #[inline(always)]
            pub fn xyw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 1, 3]) }
            }
            #[inline(always)]
            pub fn xzx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 2, 0]) }
            }
            #[inline(always)]
            pub fn xzy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 2, 1]) }
            }
            #[inline(always)]
            pub fn xzz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 2, 2]) }
            }
            #[inline(always)]
            pub fn xzw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 2, 3]) }
            }
            #[inline(always)]
            pub fn xwx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 3, 0]) }
            }
            #[inline(always)]
            pub fn xwy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 3, 1]) }
            }
            #[inline(always)]
            pub fn xwz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 3, 2]) }
            }
            #[inline(always)]
            pub fn xww(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([0, 3, 3]) }
            }
            #[inline(always)]
            pub fn yxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 0]) }
            }
            #[inline(always)]
            pub fn yxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 1]) }
            }
            #[inline(always)]
            pub fn yxz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 2]) }
            }
            #[inline(always)]
            pub fn yxw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 0, 3]) }
            }
            #[inline(always)]
            pub fn yyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 0]) }
            }
            #[inline(always)]
            pub fn yyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 1]) }
            }
            #[inline(always)]
            pub fn yyz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 2]) }
            }
            #[inline(always)]
            pub fn yyw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 1, 3]) }
            }
            #[inline(always)]
            pub fn yzx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 2, 0]) }
            }
            #[inline(always)]
            pub fn yzy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 2, 1]) }
            }
            #[inline(always)]
            pub fn yzz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 2, 2]) }
            }
            #[inline(always)]
            pub fn yzw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 2, 3]) }
            }
            #[inline(always)]
            pub fn ywx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 3, 0]) }
            }
            #[inline(always)]
            pub fn ywy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 3, 1]) }
            }
            #[inline(always)]
            pub fn ywz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 3, 2]) }
            }
            #[inline(always)]
            pub fn yww(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([1, 3, 3]) }
            }
            #[inline(always)]
            pub fn zxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 0, 0]) }
            }
            #[inline(always)]
            pub fn zxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 0, 1]) }
            }
            #[inline(always)]
            pub fn zxz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 0, 2]) }
            }
            #[inline(always)]
            pub fn zxw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 0, 3]) }
            }
            #[inline(always)]
            pub fn zyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 1, 0]) }
            }
            #[inline(always)]
            pub fn zyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 1, 1]) }
            }
            #[inline(always)]
            pub fn zyz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 1, 2]) }
            }
            #[inline(always)]
            pub fn zyw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 1, 3]) }
            }
            #[inline(always)]
            pub fn zzx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 2, 0]) }
            }
            #[inline(always)]
            pub fn zzy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 2, 1]) }
            }
            #[inline(always)]
            pub fn zzz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 2, 2]) }
            }
            #[inline(always)]
            pub fn zzw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 2, 3]) }
            }
            #[inline(always)]
            pub fn zwx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 3, 0]) }
            }
            #[inline(always)]
            pub fn zwy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 3, 1]) }
            }
            #[inline(always)]
            pub fn zwz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 3, 2]) }
            }
            #[inline(always)]
            pub fn zww(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([2, 3, 3]) }
            }
            #[inline(always)]
            pub fn wxx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 0, 0]) }
            }
            #[inline(always)]
            pub fn wxy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 0, 1]) }
            }
            #[inline(always)]
            pub fn wxz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 0, 2]) }
            }
            #[inline(always)]
            pub fn wxw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 0, 3]) }
            }
            #[inline(always)]
            pub fn wyx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 1, 0]) }
            }
            #[inline(always)]
            pub fn wyy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 1, 1]) }
            }
            #[inline(always)]
            pub fn wyz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 1, 2]) }
            }
            #[inline(always)]
            pub fn wyw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 1, 3]) }
            }
            #[inline(always)]
            pub fn wzx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 2, 0]) }
            }
            #[inline(always)]
            pub fn wzy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 2, 1]) }
            }
            #[inline(always)]
            pub fn wzz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 2, 2]) }
            }
            #[inline(always)]
            pub fn wzw(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 2, 3]) }
            }
            #[inline(always)]
            pub fn wwx(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 3, 0]) }
            }
            #[inline(always)]
            pub fn wwy(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 3, 1]) }
            }
            #[inline(always)]
            pub fn wwz(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 3, 2]) }
            }
            #[inline(always)]
            pub fn www(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([3, 3, 3]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn xxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn xxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn xxxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 2]) }
            }
            #[inline(always)]
            pub fn xxxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 0, 3]) }
            }
            #[inline(always)]
            pub fn xxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn xxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn xxyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 2]) }
            }
            #[inline(always)]
            pub fn xxyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 1, 3]) }
            }
            #[inline(always)]
            pub fn xxzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 2, 0]) }
            }
            #[inline(always)]
            pub fn xxzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 2, 1]) }
            }
            #[inline(always)]
            pub fn xxzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 2, 2]) }
            }
            #[inline(always)]
            pub fn xxzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 2, 3]) }
            }
            #[inline(always)]
            pub fn xxwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 3, 0]) }
            }
            #[inline(always)]
            pub fn xxwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 3, 1]) }
            }
            #[inline(always)]
            pub fn xxwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 3, 2]) }
            }
            #[inline(always)]
            pub fn xxww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 0, 3, 3]) }
            }
            #[inline(always)]
            pub fn xyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn xyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn xyxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 2]) }
            }
            #[inline(always)]
            pub fn xyxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 0, 3]) }
            }
            #[inline(always)]
            pub fn xyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn xyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 1]) }
            }
            #[inline(always)]
            pub fn xyyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 2]) }
            }
            #[inline(always)]
            pub fn xyyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 1, 3]) }
            }
            #[inline(always)]
            pub fn xyzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 2, 0]) }
            }
            #[inline(always)]
            pub fn xyzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 2, 1]) }
            }
            #[inline(always)]
            pub fn xyzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 2, 2]) }
            }
            #[inline(always)]
            pub fn xyzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 2, 3]) }
            }
            #[inline(always)]
            pub fn xywx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 3, 0]) }
            }
            #[inline(always)]
            pub fn xywy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 3, 1]) }
            }
            #[inline(always)]
            pub fn xywz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 3, 2]) }
            }
            #[inline(always)]
            pub fn xyww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 1, 3, 3]) }
            }
            #[inline(always)]
            pub fn xzxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 0, 0]) }
            }
            #[inline(always)]
            pub fn xzxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 0, 1]) }
            }
            #[inline(always)]
            pub fn xzxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 0, 2]) }
            }
            #[inline(always)]
            pub fn xzxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 0, 3]) }
            }
            #[inline(always)]
            pub fn xzyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 1, 0]) }
            }
            #[inline(always)]
            pub fn xzyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 1, 1]) }
            }
            #[inline(always)]
            pub fn xzyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 1, 2]) }
            }
            #[inline(always)]
            pub fn xzyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 1, 3]) }
            }
            #[inline(always)]
            pub fn xzzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 2, 0]) }
            }
            #[inline(always)]
            pub fn xzzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 2, 1]) }
            }
            #[inline(always)]
            pub fn xzzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 2, 2]) }
            }
            #[inline(always)]
            pub fn xzzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 2, 3]) }
            }
            #[inline(always)]
            pub fn xzwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 3, 0]) }
            }
            #[inline(always)]
            pub fn xzwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 3, 1]) }
            }
            #[inline(always)]
            pub fn xzwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 3, 2]) }
            }
            #[inline(always)]
            pub fn xzww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 2, 3, 3]) }
            }
            #[inline(always)]
            pub fn xwxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 0, 0]) }
            }
            #[inline(always)]
            pub fn xwxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 0, 1]) }
            }
            #[inline(always)]
            pub fn xwxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 0, 2]) }
            }
            #[inline(always)]
            pub fn xwxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 0, 3]) }
            }
            #[inline(always)]
            pub fn xwyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 1, 0]) }
            }
            #[inline(always)]
            pub fn xwyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 1, 1]) }
            }
            #[inline(always)]
            pub fn xwyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 1, 2]) }
            }
            #[inline(always)]
            pub fn xwyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 1, 3]) }
            }
            #[inline(always)]
            pub fn xwzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 2, 0]) }
            }
            #[inline(always)]
            pub fn xwzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 2, 1]) }
            }
            #[inline(always)]
            pub fn xwzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 2, 2]) }
            }
            #[inline(always)]
            pub fn xwzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 2, 3]) }
            }
            #[inline(always)]
            pub fn xwwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 3, 0]) }
            }
            #[inline(always)]
            pub fn xwwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 3, 1]) }
            }
            #[inline(always)]
            pub fn xwwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 3, 2]) }
            }
            #[inline(always)]
            pub fn xwww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([0, 3, 3, 3]) }
            }
            #[inline(always)]
            pub fn yxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn yxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn yxxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 2]) }
            }
            #[inline(always)]
            pub fn yxxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 0, 3]) }
            }
            #[inline(always)]
            pub fn yxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn yxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn yxyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 2]) }
            }
            #[inline(always)]
            pub fn yxyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 1, 3]) }
            }
            #[inline(always)]
            pub fn yxzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 2, 0]) }
            }
            #[inline(always)]
            pub fn yxzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 2, 1]) }
            }
            #[inline(always)]
            pub fn yxzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 2, 2]) }
            }
            #[inline(always)]
            pub fn yxzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 2, 3]) }
            }
            #[inline(always)]
            pub fn yxwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 3, 0]) }
            }
            #[inline(always)]
            pub fn yxwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 3, 1]) }
            }
            #[inline(always)]
            pub fn yxwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 3, 2]) }
            }
            #[inline(always)]
            pub fn yxww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 0, 3, 3]) }
            }
            #[inline(always)]
            pub fn yyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn yyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn yyxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 2]) }
            }
            #[inline(always)]
            pub fn yyxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 0, 3]) }
            }
            #[inline(always)]
            pub fn yyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn yyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 1]) }
            }
            #[inline(always)]
            pub fn yyyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 2]) }
            }
            #[inline(always)]
            pub fn yyyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 1, 3]) }
            }
            #[inline(always)]
            pub fn yyzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 2, 0]) }
            }
            #[inline(always)]
            pub fn yyzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 2, 1]) }
            }
            #[inline(always)]
            pub fn yyzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 2, 2]) }
            }
            #[inline(always)]
            pub fn yyzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 2, 3]) }
            }
            #[inline(always)]
            pub fn yywx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 3, 0]) }
            }
            #[inline(always)]
            pub fn yywy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 3, 1]) }
            }
            #[inline(always)]
            pub fn yywz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 3, 2]) }
            }
            #[inline(always)]
            pub fn yyww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 1, 3, 3]) }
            }
            #[inline(always)]
            pub fn yzxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 0, 0]) }
            }
            #[inline(always)]
            pub fn yzxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 0, 1]) }
            }
            #[inline(always)]
            pub fn yzxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 0, 2]) }
            }
            #[inline(always)]
            pub fn yzxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 0, 3]) }
            }
            #[inline(always)]
            pub fn yzyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 1, 0]) }
            }
            #[inline(always)]
            pub fn yzyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 1, 1]) }
            }
            #[inline(always)]
            pub fn yzyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 1, 2]) }
            }
            #[inline(always)]
            pub fn yzyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 1, 3]) }
            }
            #[inline(always)]
            pub fn yzzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 2, 0]) }
            }
            #[inline(always)]
            pub fn yzzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 2, 1]) }
            }
            #[inline(always)]
            pub fn yzzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 2, 2]) }
            }
            #[inline(always)]
            pub fn yzzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 2, 3]) }
            }
            #[inline(always)]
            pub fn yzwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 3, 0]) }
            }
            #[inline(always)]
            pub fn yzwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 3, 1]) }
            }
            #[inline(always)]
            pub fn yzwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 3, 2]) }
            }
            #[inline(always)]
            pub fn yzww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 2, 3, 3]) }
            }
            #[inline(always)]
            pub fn ywxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 0, 0]) }
            }
            #[inline(always)]
            pub fn ywxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 0, 1]) }
            }
            #[inline(always)]
            pub fn ywxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 0, 2]) }
            }
            #[inline(always)]
            pub fn ywxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 0, 3]) }
            }
            #[inline(always)]
            pub fn ywyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 1, 0]) }
            }
            #[inline(always)]
            pub fn ywyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 1, 1]) }
            }
            #[inline(always)]
            pub fn ywyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 1, 2]) }
            }
            #[inline(always)]
            pub fn ywyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 1, 3]) }
            }
            #[inline(always)]
            pub fn ywzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 2, 0]) }
            }
            #[inline(always)]
            pub fn ywzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 2, 1]) }
            }
            #[inline(always)]
            pub fn ywzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 2, 2]) }
            }
            #[inline(always)]
            pub fn ywzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 2, 3]) }
            }
            #[inline(always)]
            pub fn ywwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 3, 0]) }
            }
            #[inline(always)]
            pub fn ywwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 3, 1]) }
            }
            #[inline(always)]
            pub fn ywwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 3, 2]) }
            }
            #[inline(always)]
            pub fn ywww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([1, 3, 3, 3]) }
            }
            #[inline(always)]
            pub fn zxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn zxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn zxxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 0, 2]) }
            }
            #[inline(always)]
            pub fn zxxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 0, 3]) }
            }
            #[inline(always)]
            pub fn zxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn zxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn zxyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 1, 2]) }
            }
            #[inline(always)]
            pub fn zxyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 1, 3]) }
            }
            #[inline(always)]
            pub fn zxzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 2, 0]) }
            }
            #[inline(always)]
            pub fn zxzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 2, 1]) }
            }
            #[inline(always)]
            pub fn zxzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 2, 2]) }
            }
            #[inline(always)]
            pub fn zxzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 2, 3]) }
            }
            #[inline(always)]
            pub fn zxwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 3, 0]) }
            }
            #[inline(always)]
            pub fn zxwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 3, 1]) }
            }
            #[inline(always)]
            pub fn zxwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 3, 2]) }
            }
            #[inline(always)]
            pub fn zxww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 0, 3, 3]) }
            }
            #[inline(always)]
            pub fn zyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn zyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn zyxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 0, 2]) }
            }
            #[inline(always)]
            pub fn zyxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 0, 3]) }
            }
            #[inline(always)]
            pub fn zyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn zyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 1, 1]) }
            }
            #[inline(always)]
            pub fn zyyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 1, 2]) }
            }
            #[inline(always)]
            pub fn zyyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 1, 3]) }
            }
            #[inline(always)]
            pub fn zyzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 2, 0]) }
            }
            #[inline(always)]
            pub fn zyzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 2, 1]) }
            }
            #[inline(always)]
            pub fn zyzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 2, 2]) }
            }
            #[inline(always)]
            pub fn zyzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 2, 3]) }
            }
            #[inline(always)]
            pub fn zywx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 3, 0]) }
            }
            #[inline(always)]
            pub fn zywy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 3, 1]) }
            }
            #[inline(always)]
            pub fn zywz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 3, 2]) }
            }
            #[inline(always)]
            pub fn zyww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 1, 3, 3]) }
            }
            #[inline(always)]
            pub fn zzxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 0, 0]) }
            }
            #[inline(always)]
            pub fn zzxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 0, 1]) }
            }
            #[inline(always)]
            pub fn zzxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 0, 2]) }
            }
            #[inline(always)]
            pub fn zzxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 0, 3]) }
            }
            #[inline(always)]
            pub fn zzyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 1, 0]) }
            }
            #[inline(always)]
            pub fn zzyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 1, 1]) }
            }
            #[inline(always)]
            pub fn zzyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 1, 2]) }
            }
            #[inline(always)]
            pub fn zzyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 1, 3]) }
            }
            #[inline(always)]
            pub fn zzzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 2, 0]) }
            }
            #[inline(always)]
            pub fn zzzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 2, 1]) }
            }
            #[inline(always)]
            pub fn zzzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 2, 2]) }
            }
            #[inline(always)]
            pub fn zzzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 2, 3]) }
            }
            #[inline(always)]
            pub fn zzwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 3, 0]) }
            }
            #[inline(always)]
            pub fn zzwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 3, 1]) }
            }
            #[inline(always)]
            pub fn zzwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 3, 2]) }
            }
            #[inline(always)]
            pub fn zzww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 2, 3, 3]) }
            }
            #[inline(always)]
            pub fn zwxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 0, 0]) }
            }
            #[inline(always)]
            pub fn zwxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 0, 1]) }
            }
            #[inline(always)]
            pub fn zwxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 0, 2]) }
            }
            #[inline(always)]
            pub fn zwxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 0, 3]) }
            }
            #[inline(always)]
            pub fn zwyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 1, 0]) }
            }
            #[inline(always)]
            pub fn zwyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 1, 1]) }
            }
            #[inline(always)]
            pub fn zwyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 1, 2]) }
            }
            #[inline(always)]
            pub fn zwyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 1, 3]) }
            }
            #[inline(always)]
            pub fn zwzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 2, 0]) }
            }
            #[inline(always)]
            pub fn zwzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 2, 1]) }
            }
            #[inline(always)]
            pub fn zwzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 2, 2]) }
            }
            #[inline(always)]
            pub fn zwzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 2, 3]) }
            }
            #[inline(always)]
            pub fn zwwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 3, 0]) }
            }
            #[inline(always)]
            pub fn zwwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 3, 1]) }
            }
            #[inline(always)]
            pub fn zwwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 3, 2]) }
            }
            #[inline(always)]
            pub fn zwww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([2, 3, 3, 3]) }
            }
            #[inline(always)]
            pub fn wxxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 0, 0]) }
            }
            #[inline(always)]
            pub fn wxxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 0, 1]) }
            }
            #[inline(always)]
            pub fn wxxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 0, 2]) }
            }
            #[inline(always)]
            pub fn wxxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 0, 3]) }
            }
            #[inline(always)]
            pub fn wxyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 1, 0]) }
            }
            #[inline(always)]
            pub fn wxyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 1, 1]) }
            }
            #[inline(always)]
            pub fn wxyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 1, 2]) }
            }
            #[inline(always)]
            pub fn wxyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 1, 3]) }
            }
            #[inline(always)]
            pub fn wxzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 2, 0]) }
            }
            #[inline(always)]
            pub fn wxzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 2, 1]) }
            }
            #[inline(always)]
            pub fn wxzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 2, 2]) }
            }
            #[inline(always)]
            pub fn wxzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 2, 3]) }
            }
            #[inline(always)]
            pub fn wxwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 3, 0]) }
            }
            #[inline(always)]
            pub fn wxwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 3, 1]) }
            }
            #[inline(always)]
            pub fn wxwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 3, 2]) }
            }
            #[inline(always)]
            pub fn wxww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 0, 3, 3]) }
            }
            #[inline(always)]
            pub fn wyxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 0, 0]) }
            }
            #[inline(always)]
            pub fn wyxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 0, 1]) }
            }
            #[inline(always)]
            pub fn wyxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 0, 2]) }
            }
            #[inline(always)]
            pub fn wyxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 0, 3]) }
            }
            #[inline(always)]
            pub fn wyyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 1, 0]) }
            }
            #[inline(always)]
            pub fn wyyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 1, 1]) }
            }
            #[inline(always)]
            pub fn wyyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 1, 2]) }
            }
            #[inline(always)]
            pub fn wyyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 1, 3]) }
            }
            #[inline(always)]
            pub fn wyzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 2, 0]) }
            }
            #[inline(always)]
            pub fn wyzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 2, 1]) }
            }
            #[inline(always)]
            pub fn wyzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 2, 2]) }
            }
            #[inline(always)]
            pub fn wyzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 2, 3]) }
            }
            #[inline(always)]
            pub fn wywx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 3, 0]) }
            }
            #[inline(always)]
            pub fn wywy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 3, 1]) }
            }
            #[inline(always)]
            pub fn wywz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 3, 2]) }
            }
            #[inline(always)]
            pub fn wyww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 1, 3, 3]) }
            }
            #[inline(always)]
            pub fn wzxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 0, 0]) }
            }
            #[inline(always)]
            pub fn wzxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 0, 1]) }
            }
            #[inline(always)]
            pub fn wzxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 0, 2]) }
            }
            #[inline(always)]
            pub fn wzxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 0, 3]) }
            }
            #[inline(always)]
            pub fn wzyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 1, 0]) }
            }
            #[inline(always)]
            pub fn wzyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 1, 1]) }
            }
            #[inline(always)]
            pub fn wzyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 1, 2]) }
            }
            #[inline(always)]
            pub fn wzyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 1, 3]) }
            }
            #[inline(always)]
            pub fn wzzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 2, 0]) }
            }
            #[inline(always)]
            pub fn wzzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 2, 1]) }
            }
            #[inline(always)]
            pub fn wzzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 2, 2]) }
            }
            #[inline(always)]
            pub fn wzzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 2, 3]) }
            }
            #[inline(always)]
            pub fn wzwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 3, 0]) }
            }
            #[inline(always)]
            pub fn wzwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 3, 1]) }
            }
            #[inline(always)]
            pub fn wzwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 3, 2]) }
            }
            #[inline(always)]
            pub fn wzww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 2, 3, 3]) }
            }
            #[inline(always)]
            pub fn wwxx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 0, 0]) }
            }
            #[inline(always)]
            pub fn wwxy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 0, 1]) }
            }
            #[inline(always)]
            pub fn wwxz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 0, 2]) }
            }
            #[inline(always)]
            pub fn wwxw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 0, 3]) }
            }
            #[inline(always)]
            pub fn wwyx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 1, 0]) }
            }
            #[inline(always)]
            pub fn wwyy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 1, 1]) }
            }
            #[inline(always)]
            pub fn wwyz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 1, 2]) }
            }
            #[inline(always)]
            pub fn wwyw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 1, 3]) }
            }
            #[inline(always)]
            pub fn wwzx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 2, 0]) }
            }
            #[inline(always)]
            pub fn wwzy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 2, 1]) }
            }
            #[inline(always)]
            pub fn wwzz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 2, 2]) }
            }
            #[inline(always)]
            pub fn wwzw(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 2, 3]) }
            }
            #[inline(always)]
            pub fn wwwx(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 3, 0]) }
            }
            #[inline(always)]
            pub fn wwwy(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 3, 1]) }
            }
            #[inline(always)]
            pub fn wwwz(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 3, 2]) }
            }
            #[inline(always)]
            pub fn wwww(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([3, 3, 3, 3]) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn x_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
            #[inline(always)]
            pub fn y_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(1)) })
            }
            #[inline(always)]
            pub fn xy_mut(&mut self) -> &mut Vec2P<T> {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                )
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn x_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
            #[inline(always)]
            pub fn y_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(1)) })
            }
            #[inline(always)]
            pub fn z_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(2)) })
            }
            #[inline(always)]
            pub fn xy_mut(&mut self) -> &mut Vec2P<T> {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
            #[inline(always)]
            pub fn yz_mut(&mut self) -> &mut Vec2P<T> {
                (unsafe { transmute(self.get_mut_unchecked(1)) })
            }
            #[inline(always)]
            pub fn xyz_mut(&mut self) -> &mut Vec3P<T> {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                )
            }
            #[inline(always)]
            pub fn x_z_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn x_yz_mut(&mut self) -> (&mut T, &mut Vec2P<T>) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                )
            }
            #[inline(always)]
            pub fn y_z_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn xy_z_mut(&mut self) -> (&mut Vec2P<T>, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn x_y_z_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn x_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
            #[inline(always)]
            pub fn y_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(1)) })
            }
            #[inline(always)]
            pub fn z_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(2)) })
            }
            #[inline(always)]
            pub fn w_mut(&mut self) -> &mut T {
                (unsafe { transmute(self.get_mut_unchecked(3)) })
            }
            #[inline(always)]
            pub fn xy_mut(&mut self) -> &mut Vec2P<T> {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
            #[inline(always)]
            pub fn yz_mut(&mut self) -> &mut Vec2P<T> {
                (unsafe { transmute(self.get_mut_unchecked(1)) })
            }
            #[inline(always)]
            pub fn zw_mut(&mut self) -> &mut Vec2P<T> {
                (unsafe { transmute(self.get_mut_unchecked(2)) })
            }
            #[inline(always)]
            pub fn xyz_mut(&mut self) -> &mut Vec3P<T> {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
            #[inline(always)]
            pub fn yzw_mut(&mut self) -> &mut Vec3P<T> {
                (unsafe { transmute(self.get_mut_unchecked(1)) })
            }
            #[inline(always)]
            pub fn xyzw_mut(&mut self) -> &mut Vec4P<T> {
                (unsafe { transmute(self.get_mut_unchecked(0)) })
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                )
            }
            #[inline(always)]
            pub fn x_z_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn x_w_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn x_yz_mut(&mut self) -> (&mut T, &mut Vec2P<T>) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                )
            }
            #[inline(always)]
            pub fn x_zw_mut(&mut self) -> (&mut T, &mut Vec2P<T>) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn x_yzw_mut(&mut self) -> (&mut T, &mut Vec3P<T>) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                )
            }
            #[inline(always)]
            pub fn y_z_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn y_w_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn y_zw_mut(&mut self) -> (&mut T, &mut Vec2P<T>) {
                (
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn z_w_mut(&mut self) -> (&mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn xy_z_mut(&mut self) -> (&mut Vec2P<T>, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn xy_w_mut(&mut self) -> (&mut Vec2P<T>, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn xy_zw_mut(&mut self) -> (&mut Vec2P<T>, &mut Vec2P<T>) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn yz_w_mut(&mut self) -> (&mut Vec2P<T>, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn xyz_w_mut(&mut self) -> (&mut Vec3P<T>, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn x_y_z_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn x_y_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn x_y_zw_mut(&mut self) -> (&mut T, &mut T, &mut Vec2P<T>) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                )
            }
            #[inline(always)]
            pub fn x_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn x_yz_w_mut(&mut self) -> (&mut T, &mut Vec2P<T>, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn y_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
            #[inline(always)]
            pub fn xy_z_w_mut(&mut self) -> (&mut Vec2P<T>, &mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn x_y_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
                (
                    unsafe { transmute(self.get_mut_unchecked(0)) },
                    unsafe { transmute(self.get_mut_unchecked(1)) },
                    unsafe { transmute(self.get_mut_unchecked(2)) },
                    unsafe { transmute(self.get_mut_unchecked(3)) },
                )
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn with_x(self, value: T) -> Self {
                unsafe { self.with_unchecked(0, value) }
            }
            #[inline(always)]
            pub fn with_y(self, value: T) -> Self {
                unsafe { self.with_unchecked(1, value) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn with_xy(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([0, 1], values) }
            }
            #[inline(always)]
            pub fn with_yx(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([1, 0], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn with_x(self, value: T) -> Self {
                unsafe { self.with_unchecked(0, value) }
            }
            #[inline(always)]
            pub fn with_y(self, value: T) -> Self {
                unsafe { self.with_unchecked(1, value) }
            }
            #[inline(always)]
            pub fn with_z(self, value: T) -> Self {
                unsafe { self.with_unchecked(2, value) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn with_xy(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([0, 1], values) }
            }
            #[inline(always)]
            pub fn with_xz(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([0, 2], values) }
            }
            #[inline(always)]
            pub fn with_yx(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([1, 0], values) }
            }
            #[inline(always)]
            pub fn with_yz(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([1, 2], values) }
            }
            #[inline(always)]
            pub fn with_zx(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([2, 0], values) }
            }
            #[inline(always)]
            pub fn with_zy(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([2, 1], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn with_xyz(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([0, 1, 2], values) }
            }
            #[inline(always)]
            pub fn with_xzy(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([0, 2, 1], values) }
            }
            #[inline(always)]
            pub fn with_yxz(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([1, 0, 2], values) }
            }
            #[inline(always)]
            pub fn with_yzx(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([1, 2, 0], values) }
            }
            #[inline(always)]
            pub fn with_zxy(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([2, 0, 1], values) }
            }
            #[inline(always)]
            pub fn with_zyx(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([2, 1, 0], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn with_x(self, value: T) -> Self {
                unsafe { self.with_unchecked(0, value) }
            }
            #[inline(always)]
            pub fn with_y(self, value: T) -> Self {
                unsafe { self.with_unchecked(1, value) }
            }
            #[inline(always)]
            pub fn with_z(self, value: T) -> Self {
                unsafe { self.with_unchecked(2, value) }
            }
            #[inline(always)]
            pub fn with_w(self, value: T) -> Self {
                unsafe { self.with_unchecked(3, value) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn with_xy(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([0, 1], values) }
            }
            #[inline(always)]
            pub fn with_xz(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([0, 2], values) }
            }
            #[inline(always)]
            pub fn with_xw(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([0, 3], values) }
            }
            #[inline(always)]
            pub fn with_yx(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([1, 0], values) }
            }
            #[inline(always)]
            pub fn with_yz(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([1, 2], values) }
            }
            #[inline(always)]
            pub fn with_yw(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([1, 3], values) }
            }
            #[inline(always)]
            pub fn with_zx(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([2, 0], values) }
            }
            #[inline(always)]
            pub fn with_zy(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([2, 1], values) }
            }
            #[inline(always)]
            pub fn with_zw(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([2, 3], values) }
            }
            #[inline(always)]
            pub fn with_wx(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([3, 0], values) }
            }
            #[inline(always)]
            pub fn with_wy(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([3, 1], values) }
            }
            #[inline(always)]
            pub fn with_wz(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([3, 2], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn with_xyz(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([0, 1, 2], values) }
            }
            #[inline(always)]
            pub fn with_xyw(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([0, 1, 3], values) }
            }
            #[inline(always)]
            pub fn with_xzy(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([0, 2, 1], values) }
            }
            #[inline(always)]
            pub fn with_xzw(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([0, 2, 3], values) }
            }
            #[inline(always)]
            pub fn with_xwy(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([0, 3, 1], values) }
            }
            #[inline(always)]
            pub fn with_xwz(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([0, 3, 2], values) }
            }
            #[inline(always)]
            pub fn with_yxz(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([1, 0, 2], values) }
            }
            #[inline(always)]
            pub fn with_yxw(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([1, 0, 3], values) }
            }
            #[inline(always)]
            pub fn with_yzx(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([1, 2, 0], values) }
            }
            #[inline(always)]
            pub fn with_yzw(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([1, 2, 3], values) }
            }
            #[inline(always)]
            pub fn with_ywx(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([1, 3, 0], values) }
            }
            #[inline(always)]
            pub fn with_ywz(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([1, 3, 2], values) }
            }
            #[inline(always)]
            pub fn with_zxy(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([2, 0, 1], values) }
            }
            #[inline(always)]
            pub fn with_zxw(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([2, 0, 3], values) }
            }
            #[inline(always)]
            pub fn with_zyx(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([2, 1, 0], values) }
            }
            #[inline(always)]
            pub fn with_zyw(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([2, 1, 3], values) }
            }
            #[inline(always)]
            pub fn with_zwx(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([2, 3, 0], values) }
            }
            #[inline(always)]
            pub fn with_zwy(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([2, 3, 1], values) }
            }
            #[inline(always)]
            pub fn with_wxy(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([3, 0, 1], values) }
            }
            #[inline(always)]
            pub fn with_wxz(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([3, 0, 2], values) }
            }
            #[inline(always)]
            pub fn with_wyx(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([3, 1, 0], values) }
            }
            #[inline(always)]
            pub fn with_wyz(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([3, 1, 2], values) }
            }
            #[inline(always)]
            pub fn with_wzx(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([3, 2, 0], values) }
            }
            #[inline(always)]
            pub fn with_wzy(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([3, 2, 1], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn with_xyzw(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([0, 1, 2, 3], values) }
            }
            #[inline(always)]
            pub fn with_xywz(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([0, 1, 3, 2], values) }
            }
            #[inline(always)]
            pub fn with_xzyw(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([0, 2, 1, 3], values) }
            }
            #[inline(always)]
            pub fn with_xzwy(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([0, 2, 3, 1], values) }
            }
            #[inline(always)]
            pub fn with_xwyz(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([0, 3, 1, 2], values) }
            }
            #[inline(always)]
            pub fn with_xwzy(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([0, 3, 2, 1], values) }
            }
            #[inline(always)]
            pub fn with_yxzw(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([1, 0, 2, 3], values) }
            }
            #[inline(always)]
            pub fn with_yxwz(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([1, 0, 3, 2], values) }
            }
            #[inline(always)]
            pub fn with_yzxw(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([1, 2, 0, 3], values) }
            }
            #[inline(always)]
            pub fn with_yzwx(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([1, 2, 3, 0], values) }
            }
            #[inline(always)]
            pub fn with_ywxz(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([1, 3, 0, 2], values) }
            }
            #[inline(always)]
            pub fn with_ywzx(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([1, 3, 2, 0], values) }
            }
            #[inline(always)]
            pub fn with_zxyw(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([2, 0, 1, 3], values) }
            }
            #[inline(always)]
            pub fn with_zxwy(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([2, 0, 3, 1], values) }
            }
            #[inline(always)]
            pub fn with_zyxw(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([2, 1, 0, 3], values) }
            }
            #[inline(always)]
            pub fn with_zywx(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([2, 1, 3, 0], values) }
            }
            #[inline(always)]
            pub fn with_zwxy(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([2, 3, 0, 1], values) }
            }
            #[inline(always)]
            pub fn with_zwyx(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([2, 3, 1, 0], values) }
            }
            #[inline(always)]
            pub fn with_wxyz(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([3, 0, 1, 2], values) }
            }
            #[inline(always)]
            pub fn with_wxzy(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([3, 0, 2, 1], values) }
            }
            #[inline(always)]
            pub fn with_wyxz(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([3, 1, 0, 2], values) }
            }
            #[inline(always)]
            pub fn with_wyzx(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([3, 1, 2, 0], values) }
            }
            #[inline(always)]
            pub fn with_wzxy(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([3, 2, 0, 1], values) }
            }
            #[inline(always)]
            pub fn with_wzyx(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([3, 2, 1, 0], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn set_x(&mut self, value: T) {
                unsafe { self.set_unchecked(0, value) }
            }
            #[inline(always)]
            pub fn set_y(&mut self, value: T) {
                unsafe { self.set_unchecked(1, value) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            #[inline(always)]
            pub fn set_xy(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([0, 1], values) }
            }
            #[inline(always)]
            pub fn set_yx(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([1, 0], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn set_x(&mut self, value: T) {
                unsafe { self.set_unchecked(0, value) }
            }
            #[inline(always)]
            pub fn set_y(&mut self, value: T) {
                unsafe { self.set_unchecked(1, value) }
            }
            #[inline(always)]
            pub fn set_z(&mut self, value: T) {
                unsafe { self.set_unchecked(2, value) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn set_xy(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([0, 1], values) }
            }
            #[inline(always)]
            pub fn set_xz(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([0, 2], values) }
            }
            #[inline(always)]
            pub fn set_yx(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([1, 0], values) }
            }
            #[inline(always)]
            pub fn set_yz(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([1, 2], values) }
            }
            #[inline(always)]
            pub fn set_zx(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([2, 0], values) }
            }
            #[inline(always)]
            pub fn set_zy(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([2, 1], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            #[inline(always)]
            pub fn set_xyz(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([0, 1, 2], values) }
            }
            #[inline(always)]
            pub fn set_xzy(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([0, 2, 1], values) }
            }
            #[inline(always)]
            pub fn set_yxz(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([1, 0, 2], values) }
            }
            #[inline(always)]
            pub fn set_yzx(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([1, 2, 0], values) }
            }
            #[inline(always)]
            pub fn set_zxy(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([2, 0, 1], values) }
            }
            #[inline(always)]
            pub fn set_zyx(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([2, 1, 0], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn set_x(&mut self, value: T) {
                unsafe { self.set_unchecked(0, value) }
            }
            #[inline(always)]
            pub fn set_y(&mut self, value: T) {
                unsafe { self.set_unchecked(1, value) }
            }
            #[inline(always)]
            pub fn set_z(&mut self, value: T) {
                unsafe { self.set_unchecked(2, value) }
            }
            #[inline(always)]
            pub fn set_w(&mut self, value: T) {
                unsafe { self.set_unchecked(3, value) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn set_xy(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([0, 1], values) }
            }
            #[inline(always)]
            pub fn set_xz(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([0, 2], values) }
            }
            #[inline(always)]
            pub fn set_xw(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([0, 3], values) }
            }
            #[inline(always)]
            pub fn set_yx(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([1, 0], values) }
            }
            #[inline(always)]
            pub fn set_yz(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([1, 2], values) }
            }
            #[inline(always)]
            pub fn set_yw(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([1, 3], values) }
            }
            #[inline(always)]
            pub fn set_zx(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([2, 0], values) }
            }
            #[inline(always)]
            pub fn set_zy(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([2, 1], values) }
            }
            #[inline(always)]
            pub fn set_zw(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([2, 3], values) }
            }
            #[inline(always)]
            pub fn set_wx(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([3, 0], values) }
            }
            #[inline(always)]
            pub fn set_wy(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([3, 1], values) }
            }
            #[inline(always)]
            pub fn set_wz(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([3, 2], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn set_xyz(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([0, 1, 2], values) }
            }
            #[inline(always)]
            pub fn set_xyw(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([0, 1, 3], values) }
            }
            #[inline(always)]
            pub fn set_xzy(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([0, 2, 1], values) }
            }
            #[inline(always)]
            pub fn set_xzw(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([0, 2, 3], values) }
            }
            #[inline(always)]
            pub fn set_xwy(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([0, 3, 1], values) }
            }
            #[inline(always)]
            pub fn set_xwz(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([0, 3, 2], values) }
            }
            #[inline(always)]
            pub fn set_yxz(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([1, 0, 2], values) }
            }
            #[inline(always)]
            pub fn set_yxw(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([1, 0, 3], values) }
            }
            #[inline(always)]
            pub fn set_yzx(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([1, 2, 0], values) }
            }
            #[inline(always)]
            pub fn set_yzw(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([1, 2, 3], values) }
            }
            #[inline(always)]
            pub fn set_ywx(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([1, 3, 0], values) }
            }
            #[inline(always)]
            pub fn set_ywz(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([1, 3, 2], values) }
            }
            #[inline(always)]
            pub fn set_zxy(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([2, 0, 1], values) }
            }
            #[inline(always)]
            pub fn set_zxw(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([2, 0, 3], values) }
            }
            #[inline(always)]
            pub fn set_zyx(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([2, 1, 0], values) }
            }
            #[inline(always)]
            pub fn set_zyw(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([2, 1, 3], values) }
            }
            #[inline(always)]
            pub fn set_zwx(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([2, 3, 0], values) }
            }
            #[inline(always)]
            pub fn set_zwy(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([2, 3, 1], values) }
            }
            #[inline(always)]
            pub fn set_wxy(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([3, 0, 1], values) }
            }
            #[inline(always)]
            pub fn set_wxz(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([3, 0, 2], values) }
            }
            #[inline(always)]
            pub fn set_wyx(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([3, 1, 0], values) }
            }
            #[inline(always)]
            pub fn set_wyz(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([3, 1, 2], values) }
            }
            #[inline(always)]
            pub fn set_wzx(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([3, 2, 0], values) }
            }
            #[inline(always)]
            pub fn set_wzy(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([3, 2, 1], values) }
            }
        }
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            #[inline(always)]
            pub fn set_xyzw(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([0, 1, 2, 3], values) }
            }
            #[inline(always)]
            pub fn set_xywz(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([0, 1, 3, 2], values) }
            }
            #[inline(always)]
            pub fn set_xzyw(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([0, 2, 1, 3], values) }
            }
            #[inline(always)]
            pub fn set_xzwy(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([0, 2, 3, 1], values) }
            }
            #[inline(always)]
            pub fn set_xwyz(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([0, 3, 1, 2], values) }
            }
            #[inline(always)]
            pub fn set_xwzy(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([0, 3, 2, 1], values) }
            }
            #[inline(always)]
            pub fn set_yxzw(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([1, 0, 2, 3], values) }
            }
            #[inline(always)]
            pub fn set_yxwz(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([1, 0, 3, 2], values) }
            }
            #[inline(always)]
            pub fn set_yzxw(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([1, 2, 0, 3], values) }
            }
            #[inline(always)]
            pub fn set_yzwx(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([1, 2, 3, 0], values) }
            }
            #[inline(always)]
            pub fn set_ywxz(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([1, 3, 0, 2], values) }
            }
            #[inline(always)]
            pub fn set_ywzx(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([1, 3, 2, 0], values) }
            }
            #[inline(always)]
            pub fn set_zxyw(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([2, 0, 1, 3], values) }
            }
            #[inline(always)]
            pub fn set_zxwy(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([2, 0, 3, 1], values) }
            }
            #[inline(always)]
            pub fn set_zyxw(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([2, 1, 0, 3], values) }
            }
            #[inline(always)]
            pub fn set_zywx(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([2, 1, 3, 0], values) }
            }
            #[inline(always)]
            pub fn set_zwxy(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([2, 3, 0, 1], values) }
            }
            #[inline(always)]
            pub fn set_zwyx(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([2, 3, 1, 0], values) }
            }
            #[inline(always)]
            pub fn set_wxyz(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([3, 0, 1, 2], values) }
            }
            #[inline(always)]
            pub fn set_wxzy(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([3, 0, 2, 1], values) }
            }
            #[inline(always)]
            pub fn set_wyxz(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([3, 1, 0, 2], values) }
            }
            #[inline(always)]
            pub fn set_wyzx(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([3, 1, 2, 0], values) }
            }
            #[inline(always)]
            pub fn set_wzxy(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([3, 2, 0, 1], values) }
            }
            #[inline(always)]
            pub fn set_wzyx(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([3, 2, 1, 0], values) }
            }
        }
    }
    pub(crate) mod interfaces {
        mod core {
            use std::mem::{transmute, transmute_copy};
            #[allow(unused_imports)]
            use crate::vector::{alignment::*, inner::*, length::*, *};
            const _: () = {};
            impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn from_array(array: [T; N]) -> Self {
                    <ScalarCount<N> as VecLenCore<N>>::from_array::<T, A>(array)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn into_array(self) -> [T; N] {
                    <ScalarCount<N> as VecLenCore<N>>::into_array::<T, A>(self)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn as_array(&self) -> &[T; N] {
                    <ScalarCount<N> as VecLenCore<N>>::as_array::<T, A>(self)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn as_array_mut(&mut self) -> &mut [T; N] {
                    <ScalarCount<N> as VecLenCore<N>>::as_array_mut::<T, A>(self)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get(self, index: usize) -> Option<T> {
                    <ScalarCount<N> as VecLenCore<N>>::get::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_2(self, indicies: [usize; 2]) -> Option<Vector<2, T, A>> {
                    <ScalarCount<N> as VecLenCore<N>>::get_2::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_3(self, indicies: [usize; 3]) -> Option<Vector<3, T, A>> {
                    <ScalarCount<N> as VecLenCore<N>>::get_3::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_4(self, indicies: [usize; 4]) -> Option<Vector<4, T, A>> {
                    <ScalarCount<N> as VecLenCore<N>>::get_4::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn get_unchecked(self, index: usize) -> T {
                    <ScalarCount<N> as VecLenCore<N>>::get_unchecked::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn get_2_unchecked(
                    self,
                    indicies: [usize; 2],
                ) -> Vector<2, T, A> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_2_unchecked::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn get_3_unchecked(
                    self,
                    indicies: [usize; 3],
                ) -> Vector<3, T, A> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_3_unchecked::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn get_4_unchecked(
                    self,
                    indicies: [usize; 4],
                ) -> Vector<4, T, A> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_4_unchecked::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
                    <ScalarCount<N> as VecLenCore<N>>::get_mut::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_2(&mut self, index: usize) -> Option<&mut Vec2P<T>> {
                    <ScalarCount<N> as VecLenCore<N>>::get_mut_2::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_3(&mut self, index: usize) -> Option<&mut Vec3P<T>> {
                    <ScalarCount<N> as VecLenCore<N>>::get_mut_3::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_4(&mut self, index: usize) -> Option<&mut Vec4P<T>> {
                    <ScalarCount<N> as VecLenCore<N>>::get_mut_4::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_unchecked::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn get_mut_2_unchecked(
                    &mut self,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_2_unchecked::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn get_mut_3_unchecked(
                    &mut self,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_3_unchecked::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn get_mut_4_unchecked(
                    &mut self,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_4_unchecked::<T, A>(self, index)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_1_1(
                    &mut self,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_1_1::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_1_2(
                    &mut self,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_1_2::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_1_3(
                    &mut self,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_1_3::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_2_1(
                    &mut self,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_2_1::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_2_2(
                    &mut self,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_2_2::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_3_1(
                    &mut self,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_3_1::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_1_1_1(
                    &mut self,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_1_1_1::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_1_1_2(
                    &mut self,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_1_1_2::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_1_2_1(
                    &mut self,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_1_2_1::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_2_1_1(
                    &mut self,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_2_1_1::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn get_mut_1_1_1_1(
                    &mut self,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::get_mut_1_1_1_1::<T, A>(self, indicies)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn with(self, index: usize, value: T) -> Option<Self> {
                    <ScalarCount<N> as VecLenCore<N>>::with::<T, A>(self, index, value)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn with_2(
                    self,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Option<Self> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::with_2::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn with_3(
                    self,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Option<Self> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::with_3::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn with_4(
                    self,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Option<Self> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::with_4::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn with_unchecked(mut self, index: usize, value: T) -> Self {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::with_unchecked::<T, A>(self, index, value)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn with_2_unchecked(
                    mut self,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Self {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::with_2_unchecked::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn with_3_unchecked(
                    mut self,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Self {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::with_3_unchecked::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn with_4_unchecked(
                    mut self,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Self {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::with_4_unchecked::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn set(&mut self, index: usize, value: T) -> Result<(), ()> {
                    <ScalarCount<N> as VecLenCore<N>>::set::<T, A>(self, index, value)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn set_2(
                    &mut self,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Result<(), ()> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::set_2::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn set_3(
                    &mut self,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Result<(), ()> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::set_3::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn set_4(
                    &mut self,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Result<(), ()> {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::set_4::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::set_unchecked::<T, A>(self, index, value)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn set_2_unchecked(
                    &mut self,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::set_2_unchecked::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn set_3_unchecked(
                    &mut self,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::set_3_unchecked::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub unsafe fn set_4_unchecked(
                    &mut self,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) {
                    <ScalarCount<
                        N,
                    > as VecLenCore<N>>::set_4_unchecked::<T, A>(self, indicies, values)
                }
                #[inline(always)]
                #[allow(unused_mut)]
                pub fn splat(value: T) -> Self {
                    <ScalarCount<N> as VecLenCore<N>>::splat::<T, A>(value)
                }
            }
            /// trait for types that can be put inside mathamatical types like [vectors](crate::vec::Vector) and [matricies](crate::mat::Matrix).
            ///
            /// useful when using mathamatical types while being generic over the scalar type.
            /// # Examples
            /// ```
            /// fn print_x<T: Scalar>(vec: Vec2<T>) {
            ///     println!("x is equal to {}", vec.x())
            /// }
            /// ```
            ///
            /// # Implementing [Scalar]
            /// To implement [Scalar] you need to implement all [Vector](crate::vec::Vector) fns for the scalar type.
            /// This is so that each vector fn can be optimized differently for each scalar.
            /// for example, [f32] uses SIMD to implement fns on most targets.
            ///
            /// To make an unoptimized scalar type use [scalar_default_impl](default_impl::scalar_default_impl).
            ///
            /// To make a wrapper scaler type for an existing scalar (for example Meters(f32)) use ```todo!()```
            pub trait Scalar: Construct + ScalarInnerVecs {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_from_array(
                    array: [Self; 2],
                ) -> Vector<2, Self, VecAligned> {
                    unsafe { transmute_copy(&array) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_from_array(
                    array: [Self; 3],
                ) -> Vector<3, Self, VecAligned> {
                    unsafe { transmute_copy(&[array[0], array[1], array[2], array[2]]) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_from_array(
                    array: [Self; 4],
                ) -> Vector<4, Self, VecAligned> {
                    unsafe { transmute_copy(&array) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_from_array(
                    array: [Self; 2],
                ) -> Vector<2, Self, VecPacked> {
                    Vector { inner: array }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_from_array(
                    array: [Self; 3],
                ) -> Vector<3, Self, VecPacked> {
                    Vector { inner: array }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_from_array(
                    array: [Self; 4],
                ) -> Vector<4, Self, VecPacked> {
                    Vector { inner: array }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_into_array(
                    vec: Vector<2, Self, VecAligned>,
                ) -> [Self; 2] {
                    unsafe { transmute_copy(&vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_into_array(
                    vec: Vector<3, Self, VecAligned>,
                ) -> [Self; 3] {
                    unsafe { transmute_copy(&vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_into_array(
                    vec: Vector<4, Self, VecAligned>,
                ) -> [Self; 4] {
                    unsafe { transmute_copy(&vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_into_array(vec: Vector<2, Self, VecPacked>) -> [Self; 2] {
                    unsafe { transmute_copy(&vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_into_array(vec: Vector<3, Self, VecPacked>) -> [Self; 3] {
                    unsafe { transmute_copy(&vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_into_array(vec: Vector<4, Self, VecPacked>) -> [Self; 4] {
                    unsafe { transmute_copy(&vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_as_array(
                    vec: &Vector<2, Self, VecAligned>,
                ) -> &[Self; 2] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_as_array(
                    vec: &Vector<3, Self, VecAligned>,
                ) -> &[Self; 3] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_as_array(
                    vec: &Vector<4, Self, VecAligned>,
                ) -> &[Self; 4] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_as_array(vec: &Vector<2, Self, VecPacked>) -> &[Self; 2] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_as_array(vec: &Vector<3, Self, VecPacked>) -> &[Self; 3] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_as_array(vec: &Vector<4, Self, VecPacked>) -> &[Self; 4] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_as_array_mut(
                    vec: &mut Vector<2, Self, VecAligned>,
                ) -> &mut [Self; 2] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_as_array_mut(
                    vec: &mut Vector<3, Self, VecAligned>,
                ) -> &mut [Self; 3] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_as_array_mut(
                    vec: &mut Vector<4, Self, VecAligned>,
                ) -> &mut [Self; 4] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_as_array_mut(
                    vec: &mut Vector<2, Self, VecPacked>,
                ) -> &mut [Self; 2] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_as_array_mut(
                    vec: &mut Vector<3, Self, VecPacked>,
                ) -> &mut [Self; 3] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_as_array_mut(
                    vec: &mut Vector<4, Self, VecPacked>,
                ) -> &mut [Self; 4] {
                    unsafe { transmute(vec) }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get(
                    vec: Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> Option<Self> {
                    if index >= 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get(
                    vec: Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> Option<Self> {
                    if index >= 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get(
                    vec: Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> Option<Self> {
                    if index >= 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get(
                    vec: Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> Option<Self> {
                    if index >= 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get(
                    vec: Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> Option<Self> {
                    if index >= 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get(
                    vec: Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> Option<Self> {
                    if index >= 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_2(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else {
                        Some(unsafe { vec.get_2_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_2(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else {
                        Some(unsafe { vec.get_2_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_2(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else {
                        Some(unsafe { vec.get_2_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_2(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else {
                        Some(unsafe { vec.get_2_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_2(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else {
                        Some(unsafe { vec.get_2_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_2(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else {
                        Some(unsafe { vec.get_2_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_3(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else {
                        Some(unsafe { vec.get_3_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_3(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else {
                        Some(unsafe { vec.get_3_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_3(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else {
                        Some(unsafe { vec.get_3_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_3(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else {
                        Some(unsafe { vec.get_3_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_3(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else {
                        Some(unsafe { vec.get_3_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_3(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else {
                        Some(unsafe { vec.get_3_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_4(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else {
                        Some(unsafe { vec.get_4_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_4(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else {
                        Some(unsafe { vec.get_4_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_4(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else {
                        Some(unsafe { vec.get_4_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_4(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else {
                        Some(unsafe { vec.get_4_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_4(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else {
                        Some(unsafe { vec.get_4_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_4(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else {
                        Some(unsafe { vec.get_4_unchecked(indicies) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_get_unchecked(
                    vec: Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> Self {
                    *vec.as_array().get_unchecked(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_get_unchecked(
                    vec: Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> Self {
                    *vec.as_array().get_unchecked(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_get_unchecked(
                    vec: Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> Self {
                    *vec.as_array().get_unchecked(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_get_unchecked(
                    vec: Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> Self {
                    *vec.as_array().get_unchecked(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_get_unchecked(
                    vec: Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> Self {
                    *vec.as_array().get_unchecked(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_get_unchecked(
                    vec: Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> Self {
                    *vec.as_array().get_unchecked(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_get_2_unchecked(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Vector<2, Self, VecAligned> {
                    Vector::<
                        2,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_get_2_unchecked(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Vector<2, Self, VecAligned> {
                    Vector::<
                        2,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_get_2_unchecked(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Vector<2, Self, VecAligned> {
                    Vector::<
                        2,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_get_2_unchecked(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Vector<2, Self, VecPacked> {
                    Vector::<
                        2,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_get_2_unchecked(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Vector<2, Self, VecPacked> {
                    Vector::<
                        2,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_get_2_unchecked(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Vector<2, Self, VecPacked> {
                    Vector::<
                        2,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_get_3_unchecked(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Vector<3, Self, VecAligned> {
                    Vector::<
                        3,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_get_3_unchecked(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Vector<3, Self, VecAligned> {
                    Vector::<
                        3,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_get_3_unchecked(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Vector<3, Self, VecAligned> {
                    Vector::<
                        3,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_get_3_unchecked(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Vector<3, Self, VecPacked> {
                    Vector::<
                        3,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_get_3_unchecked(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Vector<3, Self, VecPacked> {
                    Vector::<
                        3,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_get_3_unchecked(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Vector<3, Self, VecPacked> {
                    Vector::<
                        3,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_get_4_unchecked(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Vector<4, Self, VecAligned> {
                    Vector::<
                        4,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                        vec.get_unchecked(indicies[3]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_get_4_unchecked(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Vector<4, Self, VecAligned> {
                    Vector::<
                        4,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                        vec.get_unchecked(indicies[3]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_get_4_unchecked(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Vector<4, Self, VecAligned> {
                    Vector::<
                        4,
                        Self,
                        VecAligned,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                        vec.get_unchecked(indicies[3]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_get_4_unchecked(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Vector<4, Self, VecPacked> {
                    Vector::<
                        4,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                        vec.get_unchecked(indicies[3]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_get_4_unchecked(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Vector<4, Self, VecPacked> {
                    Vector::<
                        4,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                        vec.get_unchecked(indicies[3]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_get_4_unchecked(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Vector<4, Self, VecPacked> {
                    Vector::<
                        4,
                        Self,
                        VecPacked,
                    >::from_array([
                        vec.get_unchecked(indicies[0]),
                        vec.get_unchecked(indicies[1]),
                        vec.get_unchecked(indicies[2]),
                        vec.get_unchecked(indicies[3]),
                    ])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Self> {
                    if index + 1 > 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Self> {
                    if index + 1 > 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Self> {
                    if index + 1 > 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Self> {
                    if index + 1 > 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Self> {
                    if index + 1 > 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Self> {
                    if index + 1 > 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_2(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec2P<Self>> {
                    if index + 2 > 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_2_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_2(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec2P<Self>> {
                    if index + 2 > 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_2_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_2(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec2P<Self>> {
                    if index + 2 > 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_2_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_2(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec2P<Self>> {
                    if index + 2 > 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_2_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_2(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec2P<Self>> {
                    if index + 2 > 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_2_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_2(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec2P<Self>> {
                    if index + 2 > 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_2_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_3(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec3P<Self>> {
                    if index + 3 > 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_3_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_3(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec3P<Self>> {
                    if index + 3 > 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_3_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_3(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec3P<Self>> {
                    if index + 3 > 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_3_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_3(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec3P<Self>> {
                    if index + 3 > 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_3_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_3(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec3P<Self>> {
                    if index + 3 > 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_3_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_3(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec3P<Self>> {
                    if index + 3 > 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_3_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_4(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec4P<Self>> {
                    if index + 4 > 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_4_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_4(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec4P<Self>> {
                    if index + 4 > 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_4_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_4(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> Option<&mut Vec4P<Self>> {
                    if index + 4 > 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_4_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_4(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec4P<Self>> {
                    if index + 4 > 2 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_4_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_4(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec4P<Self>> {
                    if index + 4 > 3 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_4_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_4(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> Option<&mut Vec4P<Self>> {
                    if index + 4 > 4 {
                        None
                    } else {
                        Some(unsafe { vec.get_mut_4_unchecked(index) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_get_mut_unchecked(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> &mut Self {
                    vec.as_array_mut().get_unchecked_mut(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_get_mut_unchecked(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> &mut Self {
                    vec.as_array_mut().get_unchecked_mut(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_get_mut_unchecked(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> &mut Self {
                    vec.as_array_mut().get_unchecked_mut(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_get_mut_unchecked(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> &mut Self {
                    vec.as_array_mut().get_unchecked_mut(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_get_mut_unchecked(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> &mut Self {
                    vec.as_array_mut().get_unchecked_mut(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_get_mut_unchecked(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> &mut Self {
                    vec.as_array_mut().get_unchecked_mut(index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_get_mut_2_unchecked(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec2P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_get_mut_2_unchecked(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec2P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_get_mut_2_unchecked(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec2P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_get_mut_2_unchecked(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec2P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_get_mut_2_unchecked(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec2P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_get_mut_2_unchecked(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec2P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_get_mut_3_unchecked(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec3P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_get_mut_3_unchecked(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec3P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_get_mut_3_unchecked(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec3P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_get_mut_3_unchecked(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec3P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_get_mut_3_unchecked(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec3P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_get_mut_3_unchecked(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec3P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_get_mut_4_unchecked(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec4P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_get_mut_4_unchecked(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec4P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_get_mut_4_unchecked(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                ) -> &mut Vec4P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_get_mut_4_unchecked(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec4P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_get_mut_4_unchecked(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec4P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_get_mut_4_unchecked(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                ) -> &mut Vec4P<Self> {
                    transmute(vec.get_mut_unchecked(index))
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_1_1(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_1_1(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_1_1(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_1_1(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_1_1(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_1_1(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_1_2(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_1_2(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_1_2(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_1_2(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_1_2(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_1_2(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_1_3(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec3P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] == indicies[1] + 2
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_3(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_1_3(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec3P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] == indicies[1] + 2
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_3(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_1_3(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec3P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] == indicies[1] + 2
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_3(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_1_3(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec3P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] == indicies[1] + 2
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_3(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_1_3(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec3P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] == indicies[1] + 2
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_3(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_1_3(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Self, &mut Vec3P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] == indicies[1] + 2
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_3(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_2_1(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_2_1(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_2_1(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_2_1(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_2_1(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_2_1(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_2_2(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] + 1 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_2_2(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] + 1 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_2_2(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] + 1 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_2_2(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] + 1 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_2_2(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] + 1 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_2_2(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<Self>, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1
                        || indicies[0] + 1 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_3_1(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1]
                        || indicies[0] + 2 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_3(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_3_1(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1]
                        || indicies[0] + 2 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_3(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_3_1(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1]
                        || indicies[0] + 2 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_3(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_3_1(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1]
                        || indicies[0] + 2 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_3(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_3_1(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1]
                        || indicies[0] + 2 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_3(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_3_1(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1]
                        || indicies[0] + 2 == indicies[1]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_3(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_1_1_1(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_1_1_1(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_1_1_1(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_1_1_1(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_1_1_1(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_1_1_1(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_1_1_2(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] == indicies[2] + 1
                    {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] == indicies[2] + 1
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_1_1_2(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] == indicies[2] + 1
                    {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] == indicies[2] + 1
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_1_1_2(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] == indicies[2] + 1
                    {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] == indicies[2] + 1
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_1_1_2(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] == indicies[2] + 1
                    {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] == indicies[2] + 1
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_1_1_2(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] == indicies[2] + 1
                    {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] == indicies[2] + 1
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_1_1_2(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Self, &mut Vec2P<Self>)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] == indicies[2] + 1
                    {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] == indicies[2] + 1
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_1_2_1(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] + 1 == indicies[2]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_1_2_1(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] + 1 == indicies[2]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_1_2_1(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] + 1 == indicies[2]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_1_2_1(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] + 1 == indicies[2]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_1_2_1(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] + 1 == indicies[2]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_1_2_1(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Self, &mut Vec2P<Self>, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2]
                        || indicies[1] + 1 == indicies[2]
                    {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_2_1_1(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<Self>, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] + 1 == indicies[2]
                    {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_2_1_1(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<Self>, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] + 1 == indicies[2]
                    {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_2_1_1(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<Self>, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] + 1 == indicies[2]
                    {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_2_1_1(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<Self>, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] + 1 == indicies[2]
                    {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_2_1_1(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<Self>, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] + 1 == indicies[2]
                    {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_2_1_1(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<Self>, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2]
                        || indicies[0] + 1 == indicies[2]
                    {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut_2(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_get_mut_1_1_1_1(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Option<(&mut Self, &mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecAligned>,
                                    &mut Vector<2, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[3])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_get_mut_1_1_1_1(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Option<(&mut Self, &mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecAligned>,
                                    &mut Vector<3, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[3])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_get_mut_1_1_1_1(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 4],
                ) -> Option<(&mut Self, &mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecAligned>,
                                    &mut Vector<4, Self, VecAligned>,
                                >(vec)
                            }
                                .get_mut(indicies[3])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_get_mut_1_1_1_1(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Option<(&mut Self, &mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<2, Self, VecPacked>,
                                    &mut Vector<2, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[3])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_get_mut_1_1_1_1(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Option<(&mut Self, &mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<3, Self, VecPacked>,
                                    &mut Vector<3, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[3])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_get_mut_1_1_1_1(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 4],
                ) -> Option<(&mut Self, &mut Self, &mut Self, &mut Self)> {
                    if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some((
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[0])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[1])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[2])?,
                            unsafe {
                                transmute::<
                                    &mut Vector<4, Self, VecPacked>,
                                    &mut Vector<4, Self, VecPacked>,
                                >(vec)
                            }
                                .get_mut(indicies[3])?,
                        ))
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_with(
                    vec: Vector<2, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Option<Vector<2, Self, VecAligned>> {
                    if index >= 2 {
                        None
                    } else {
                        Some(unsafe { vec.with_unchecked(index, value) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_with(
                    vec: Vector<3, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Option<Vector<3, Self, VecAligned>> {
                    if index >= 3 {
                        None
                    } else {
                        Some(unsafe { vec.with_unchecked(index, value) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_with(
                    vec: Vector<4, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Option<Vector<4, Self, VecAligned>> {
                    if index >= 4 {
                        None
                    } else {
                        Some(unsafe { vec.with_unchecked(index, value) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_with(
                    vec: Vector<2, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Option<Vector<2, Self, VecPacked>> {
                    if index >= 2 {
                        None
                    } else {
                        Some(unsafe { vec.with_unchecked(index, value) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_with(
                    vec: Vector<3, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Option<Vector<3, Self, VecPacked>> {
                    if index >= 3 {
                        None
                    } else {
                        Some(unsafe { vec.with_unchecked(index, value) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_with(
                    vec: Vector<4, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Option<Vector<4, Self, VecPacked>> {
                    if index >= 4 {
                        None
                    } else {
                        Some(unsafe { vec.with_unchecked(index, value) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_with_2(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Option<Vector<2, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some(unsafe { vec.with_2_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_with_2(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Option<Vector<3, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some(unsafe { vec.with_2_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_with_2(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Option<Vector<4, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some(unsafe { vec.with_2_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_with_2(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Option<Vector<2, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some(unsafe { vec.with_2_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_with_2(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Option<Vector<3, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some(unsafe { vec.with_2_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_with_2(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Option<Vector<4, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else {
                        Some(unsafe { vec.with_2_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_with_3(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Option<Vector<2, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some(unsafe { vec.with_3_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_with_3(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Option<Vector<3, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some(unsafe { vec.with_3_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_with_3(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Option<Vector<4, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some(unsafe { vec.with_3_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_with_3(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Option<Vector<2, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some(unsafe { vec.with_3_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_with_3(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Option<Vector<3, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some(unsafe { vec.with_3_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_with_3(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Option<Vector<4, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else {
                        Some(unsafe { vec.with_3_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_with_4(
                    vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Option<Vector<2, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some(unsafe { vec.with_4_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_with_4(
                    vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Option<Vector<3, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some(unsafe { vec.with_4_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_with_4(
                    vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Option<Vector<4, Self, VecAligned>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some(unsafe { vec.with_4_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_with_4(
                    vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Option<Vector<2, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some(unsafe { vec.with_4_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_with_4(
                    vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Option<Vector<3, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some(unsafe { vec.with_4_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_with_4(
                    vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Option<Vector<4, Self, VecPacked>> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        None
                    } else if indicies[0] == indicies[1] {
                        None
                    } else if indicies[0] == indicies[2] {
                        None
                    } else if indicies[0] == indicies[3] {
                        None
                    } else if indicies[1] == indicies[2] {
                        None
                    } else if indicies[1] == indicies[3] {
                        None
                    } else if indicies[2] == indicies[3] {
                        None
                    } else {
                        Some(unsafe { vec.with_4_unchecked(indicies, values) })
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_with_unchecked(
                    mut vec: Vector<2, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Vector<2, Self, VecAligned> {
                    *vec.get_mut_unchecked(index) = value;
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_with_unchecked(
                    mut vec: Vector<3, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Vector<3, Self, VecAligned> {
                    *vec.get_mut_unchecked(index) = value;
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_with_unchecked(
                    mut vec: Vector<4, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Vector<4, Self, VecAligned> {
                    *vec.get_mut_unchecked(index) = value;
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_with_unchecked(
                    mut vec: Vector<2, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Vector<2, Self, VecPacked> {
                    *vec.get_mut_unchecked(index) = value;
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_with_unchecked(
                    mut vec: Vector<3, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Vector<3, Self, VecPacked> {
                    *vec.get_mut_unchecked(index) = value;
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_with_unchecked(
                    mut vec: Vector<4, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Vector<4, Self, VecPacked> {
                    *vec.get_mut_unchecked(index) = value;
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_with_2_unchecked(
                    mut vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Vector<2, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_with_2_unchecked(
                    mut vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Vector<3, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_with_2_unchecked(
                    mut vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Vector<4, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_with_2_unchecked(
                    mut vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Vector<2, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_with_2_unchecked(
                    mut vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Vector<3, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_with_2_unchecked(
                    mut vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Vector<4, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_with_3_unchecked(
                    mut vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Vector<2, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_with_3_unchecked(
                    mut vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Vector<3, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_with_3_unchecked(
                    mut vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Vector<4, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_with_3_unchecked(
                    mut vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Vector<2, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_with_3_unchecked(
                    mut vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Vector<3, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_with_3_unchecked(
                    mut vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Vector<4, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_with_4_unchecked(
                    mut vec: Vector<2, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Vector<2, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    *vec.get_mut_unchecked(indicies[3]) = values.get_unchecked(3);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_with_4_unchecked(
                    mut vec: Vector<3, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Vector<3, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    *vec.get_mut_unchecked(indicies[3]) = values.get_unchecked(3);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_with_4_unchecked(
                    mut vec: Vector<4, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Vector<4, Self, VecAligned> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    *vec.get_mut_unchecked(indicies[3]) = values.get_unchecked(3);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_with_4_unchecked(
                    mut vec: Vector<2, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Vector<2, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    *vec.get_mut_unchecked(indicies[3]) = values.get_unchecked(3);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_with_4_unchecked(
                    mut vec: Vector<3, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Vector<3, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    *vec.get_mut_unchecked(indicies[3]) = values.get_unchecked(3);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_with_4_unchecked(
                    mut vec: Vector<4, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Vector<4, Self, VecPacked> {
                    *vec.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
                    *vec.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
                    *vec.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
                    *vec.get_mut_unchecked(indicies[3]) = values.get_unchecked(3);
                    vec
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_set(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Result<(), ()> {
                    if index >= 2 {
                        Err(())
                    } else {
                        unsafe { vec.set_unchecked(index, value) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_set(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Result<(), ()> {
                    if index >= 3 {
                        Err(())
                    } else {
                        unsafe { vec.set_unchecked(index, value) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_set(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) -> Result<(), ()> {
                    if index >= 4 {
                        Err(())
                    } else {
                        unsafe { vec.set_unchecked(index, value) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_set(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Result<(), ()> {
                    if index >= 2 {
                        Err(())
                    } else {
                        unsafe { vec.set_unchecked(index, value) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_set(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Result<(), ()> {
                    if index >= 3 {
                        Err(())
                    } else {
                        unsafe { vec.set_unchecked(index, value) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_set(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) -> Result<(), ()> {
                    if index >= 4 {
                        Err(())
                    } else {
                        unsafe { vec.set_unchecked(index, value) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_set_2(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else {
                        unsafe { vec.set_2_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_set_2(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else {
                        unsafe { vec.set_2_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_set_2(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else {
                        unsafe { vec.set_2_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_set_2(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else {
                        unsafe { vec.set_2_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_set_2(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else {
                        unsafe { vec.set_2_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_set_2(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else {
                        unsafe { vec.set_2_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_set_3(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else {
                        unsafe { vec.set_3_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_set_3(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else {
                        unsafe { vec.set_3_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_set_3(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else {
                        unsafe { vec.set_3_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_set_3(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else {
                        unsafe { vec.set_3_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_set_3(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else {
                        unsafe { vec.set_3_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_set_3(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else {
                        unsafe { vec.set_3_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_set_4(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[0] == indicies[3] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[3] {
                        Err(())
                    } else if indicies[2] == indicies[3] {
                        Err(())
                    } else {
                        unsafe { vec.set_4_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_set_4(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[0] == indicies[3] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[3] {
                        Err(())
                    } else if indicies[2] == indicies[3] {
                        Err(())
                    } else {
                        unsafe { vec.set_4_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_set_4(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[0] == indicies[3] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[3] {
                        Err(())
                    } else if indicies[2] == indicies[3] {
                        Err(())
                    } else {
                        unsafe { vec.set_4_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_set_4(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 2) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[0] == indicies[3] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[3] {
                        Err(())
                    } else if indicies[2] == indicies[3] {
                        Err(())
                    } else {
                        unsafe { vec.set_4_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_set_4(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 3) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[0] == indicies[3] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[3] {
                        Err(())
                    } else if indicies[2] == indicies[3] {
                        Err(())
                    } else {
                        unsafe { vec.set_4_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_set_4(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) -> Result<(), ()> {
                    if indicies.into_iter().any(|index| index >= 4) {
                        Err(())
                    } else if indicies[0] == indicies[1] {
                        Err(())
                    } else if indicies[0] == indicies[2] {
                        Err(())
                    } else if indicies[0] == indicies[3] {
                        Err(())
                    } else if indicies[1] == indicies[2] {
                        Err(())
                    } else if indicies[1] == indicies[3] {
                        Err(())
                    } else if indicies[2] == indicies[3] {
                        Err(())
                    } else {
                        unsafe { vec.set_4_unchecked(indicies, values) }
                        Ok(())
                    }
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_set_unchecked(
                    vec: &mut Vector<2, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) {
                    *vec = vec.with_unchecked(index, value);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_set_unchecked(
                    vec: &mut Vector<3, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) {
                    *vec = vec.with_unchecked(index, value);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_set_unchecked(
                    vec: &mut Vector<4, Self, VecAligned>,
                    index: usize,
                    value: Self,
                ) {
                    *vec = vec.with_unchecked(index, value);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_set_unchecked(
                    vec: &mut Vector<2, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) {
                    *vec = vec.with_unchecked(index, value);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_set_unchecked(
                    vec: &mut Vector<3, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) {
                    *vec = vec.with_unchecked(index, value);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_set_unchecked(
                    vec: &mut Vector<4, Self, VecPacked>,
                    index: usize,
                    value: Self,
                ) {
                    *vec = vec.with_unchecked(index, value);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_set_2_unchecked(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) {
                    *vec = vec.with_2_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_set_2_unchecked(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) {
                    *vec = vec.with_2_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_set_2_unchecked(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecAligned>,
                ) {
                    *vec = vec.with_2_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_set_2_unchecked(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) {
                    *vec = vec.with_2_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_set_2_unchecked(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) {
                    *vec = vec.with_2_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_set_2_unchecked(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 2],
                    values: Vector<2, Self, VecPacked>,
                ) {
                    *vec = vec.with_2_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_set_3_unchecked(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) {
                    *vec = vec.with_3_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_set_3_unchecked(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) {
                    *vec = vec.with_3_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_set_3_unchecked(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecAligned>,
                ) {
                    *vec = vec.with_3_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_set_3_unchecked(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) {
                    *vec = vec.with_3_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_set_3_unchecked(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) {
                    *vec = vec.with_3_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_set_3_unchecked(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 3],
                    values: Vector<3, Self, VecPacked>,
                ) {
                    *vec = vec.with_3_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec2_set_4_unchecked(
                    vec: &mut Vector<2, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) {
                    *vec = vec.with_4_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec3_set_4_unchecked(
                    vec: &mut Vector<3, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) {
                    *vec = vec.with_4_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn aligned_vec4_set_4_unchecked(
                    vec: &mut Vector<4, Self, VecAligned>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecAligned>,
                ) {
                    *vec = vec.with_4_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec2_set_4_unchecked(
                    vec: &mut Vector<2, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) {
                    *vec = vec.with_4_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec3_set_4_unchecked(
                    vec: &mut Vector<3, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) {
                    *vec = vec.with_4_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn packed_vec4_set_4_unchecked(
                    vec: &mut Vector<4, Self, VecPacked>,
                    indicies: [usize; 4],
                    values: Vector<4, Self, VecPacked>,
                ) {
                    *vec = vec.with_4_unchecked(indicies, values);
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_splat(value: Self) -> Vector<2, Self, VecAligned> {
                    Vector::from_array([value; 2])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_splat(value: Self) -> Vector<3, Self, VecAligned> {
                    Vector::from_array([value; 3])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_splat(value: Self) -> Vector<4, Self, VecAligned> {
                    Vector::from_array([value; 4])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_splat(value: Self) -> Vector<2, Self, VecPacked> {
                    Vector::from_array([value; 2])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_splat(value: Self) -> Vector<3, Self, VecPacked> {
                    Vector::from_array([value; 3])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_splat(value: Self) -> Vector<4, Self, VecPacked> {
                    Vector::from_array([value; 4])
                }
            }
            pub(super) trait VecLenCore<const N: usize>: VecLenInnerVec
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[allow(missing_docs)]
                fn from_array<T: Scalar, A: VecAlignment>(
                    array: [T; N],
                ) -> Vector<N, T, A>;
                #[allow(missing_docs)]
                fn into_array<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                ) -> [T; N];
                #[allow(missing_docs)]
                fn as_array<T: Scalar, A: VecAlignment>(
                    vec: &Vector<N, T, A>,
                ) -> &[T; N];
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                ) -> &mut [T; N];
                #[allow(missing_docs)]
                fn get<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    index: usize,
                ) -> Option<T>;
                #[allow(missing_docs)]
                fn get_2<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, A>>;
                #[allow(missing_docs)]
                fn get_3<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, A>>;
                #[allow(missing_docs)]
                fn get_4<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, A>>;
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    index: usize,
                ) -> T;
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, A>;
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, A>;
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, A>;
                #[allow(missing_docs)]
                fn get_mut<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                ) -> Option<&mut T>;
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>>;
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>>;
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>>;
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                ) -> &mut T;
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                ) -> &mut Vec2P<T>;
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                ) -> &mut Vec3P<T>;
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                ) -> &mut Vec4P<T>;
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)>;
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)>;
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)>;
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)>;
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)>;
                #[allow(missing_docs)]
                fn with<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<N, T, A>>;
                #[allow(missing_docs)]
                fn with_2<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Option<Vector<N, T, A>>;
                #[allow(missing_docs)]
                fn with_3<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Option<Vector<N, T, A>>;
                #[allow(missing_docs)]
                fn with_4<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Option<Vector<N, T, A>>;
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    index: usize,
                    value: T,
                ) -> Vector<N, T, A>;
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Vector<N, T, A>;
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Vector<N, T, A>;
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<N, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Vector<N, T, A>;
                #[allow(missing_docs)]
                fn set<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()>;
                #[allow(missing_docs)]
                fn set_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Result<(), ()>;
                #[allow(missing_docs)]
                fn set_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Result<(), ()>;
                #[allow(missing_docs)]
                fn set_4<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Result<(), ()>;
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    index: usize,
                    value: T,
                );
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                );
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                );
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<N, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                );
                #[allow(missing_docs)]
                fn splat<T: Scalar, A: VecAlignment>(value: T) -> Vector<N, T, A>;
            }
            impl VecLenCore<2> for ScalarCount<2> {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar, A: VecAlignment>(
                    array: [T; 2],
                ) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<2>>::from_array::<T>(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                ) -> [T; 2] {
                    <A as VecAlignmentCore<2>>::into_array::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar, A: VecAlignment>(
                    vec: &Vector<2, T, A>,
                ) -> &[T; 2] {
                    <A as VecAlignmentCore<2>>::as_array::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                ) -> &mut [T; 2] {
                    <A as VecAlignmentCore<2>>::as_array_mut::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    index: usize,
                ) -> Option<T> {
                    <A as VecAlignmentCore<2>>::get::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, A>> {
                    <A as VecAlignmentCore<2>>::get_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, A>> {
                    <A as VecAlignmentCore<2>>::get_3::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, A>> {
                    <A as VecAlignmentCore<2>>::get_4::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    index: usize,
                ) -> T {
                    <A as VecAlignmentCore<2>>::get_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<2>>::get_2_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<2>>::get_3_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<2>>::get_4_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                ) -> Option<&mut T> {
                    <A as VecAlignmentCore<2>>::get_mut::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    <A as VecAlignmentCore<2>>::get_mut_2::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    <A as VecAlignmentCore<2>>::get_mut_3::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    <A as VecAlignmentCore<2>>::get_mut_4::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                ) -> &mut T {
                    <A as VecAlignmentCore<2>>::get_mut_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    <A as VecAlignmentCore<2>>::get_mut_2_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    <A as VecAlignmentCore<2>>::get_mut_3_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    <A as VecAlignmentCore<2>>::get_mut_4_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    <A as VecAlignmentCore<2>>::get_mut_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<2>>::get_mut_1_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    <A as VecAlignmentCore<2>>::get_mut_1_3::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    <A as VecAlignmentCore<2>>::get_mut_2_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<2>>::get_mut_2_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    <A as VecAlignmentCore<2>>::get_mut_3_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    <A as VecAlignmentCore<2>>::get_mut_1_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<2>>::get_mut_1_1_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    <A as VecAlignmentCore<2>>::get_mut_1_2_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    <A as VecAlignmentCore<2>>::get_mut_2_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    <A as VecAlignmentCore<2>>::get_mut_1_1_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<2, T, A>> {
                    <A as VecAlignmentCore<2>>::with::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Option<Vector<2, T, A>> {
                    <A as VecAlignmentCore<2>>::with_2::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Option<Vector<2, T, A>> {
                    <A as VecAlignmentCore<2>>::with_3::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar, A: VecAlignment>(
                    vec: Vector<2, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Option<Vector<2, T, A>> {
                    <A as VecAlignmentCore<2>>::with_4::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<2, T, A>,
                    index: usize,
                    value: T,
                ) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<2>>::with_unchecked::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<2, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<
                        2,
                    >>::with_2_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<2, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<
                        2,
                    >>::with_3_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<2, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<
                        2,
                    >>::with_4_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<2>>::set::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<2>>::set_2::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<2>>::set_3::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<2>>::set_4::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    index: usize,
                    value: T,
                ) {
                    <A as VecAlignmentCore<2>>::set_unchecked::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        2,
                    >>::set_2_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        2,
                    >>::set_3_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<2, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        2,
                    >>::set_4_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar, A: VecAlignment>(value: T) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<2>>::splat::<T>(value)
                }
            }
            impl VecLenCore<3> for ScalarCount<3> {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar, A: VecAlignment>(
                    array: [T; 3],
                ) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<3>>::from_array::<T>(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                ) -> [T; 3] {
                    <A as VecAlignmentCore<3>>::into_array::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar, A: VecAlignment>(
                    vec: &Vector<3, T, A>,
                ) -> &[T; 3] {
                    <A as VecAlignmentCore<3>>::as_array::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                ) -> &mut [T; 3] {
                    <A as VecAlignmentCore<3>>::as_array_mut::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    index: usize,
                ) -> Option<T> {
                    <A as VecAlignmentCore<3>>::get::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, A>> {
                    <A as VecAlignmentCore<3>>::get_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, A>> {
                    <A as VecAlignmentCore<3>>::get_3::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, A>> {
                    <A as VecAlignmentCore<3>>::get_4::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    index: usize,
                ) -> T {
                    <A as VecAlignmentCore<3>>::get_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<3>>::get_2_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<3>>::get_3_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<3>>::get_4_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                ) -> Option<&mut T> {
                    <A as VecAlignmentCore<3>>::get_mut::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    <A as VecAlignmentCore<3>>::get_mut_2::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    <A as VecAlignmentCore<3>>::get_mut_3::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    <A as VecAlignmentCore<3>>::get_mut_4::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                ) -> &mut T {
                    <A as VecAlignmentCore<3>>::get_mut_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    <A as VecAlignmentCore<3>>::get_mut_2_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    <A as VecAlignmentCore<3>>::get_mut_3_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    <A as VecAlignmentCore<3>>::get_mut_4_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    <A as VecAlignmentCore<3>>::get_mut_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<3>>::get_mut_1_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    <A as VecAlignmentCore<3>>::get_mut_1_3::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    <A as VecAlignmentCore<3>>::get_mut_2_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<3>>::get_mut_2_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    <A as VecAlignmentCore<3>>::get_mut_3_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    <A as VecAlignmentCore<3>>::get_mut_1_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<3>>::get_mut_1_1_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    <A as VecAlignmentCore<3>>::get_mut_1_2_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    <A as VecAlignmentCore<3>>::get_mut_2_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    <A as VecAlignmentCore<3>>::get_mut_1_1_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<3, T, A>> {
                    <A as VecAlignmentCore<3>>::with::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Option<Vector<3, T, A>> {
                    <A as VecAlignmentCore<3>>::with_2::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Option<Vector<3, T, A>> {
                    <A as VecAlignmentCore<3>>::with_3::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar, A: VecAlignment>(
                    vec: Vector<3, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Option<Vector<3, T, A>> {
                    <A as VecAlignmentCore<3>>::with_4::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<3, T, A>,
                    index: usize,
                    value: T,
                ) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<3>>::with_unchecked::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<3, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<
                        3,
                    >>::with_2_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<3, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<
                        3,
                    >>::with_3_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<3, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<
                        3,
                    >>::with_4_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<3>>::set::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<3>>::set_2::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<3>>::set_3::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<3>>::set_4::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    index: usize,
                    value: T,
                ) {
                    <A as VecAlignmentCore<3>>::set_unchecked::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        3,
                    >>::set_2_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        3,
                    >>::set_3_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<3, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        3,
                    >>::set_4_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar, A: VecAlignment>(value: T) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<3>>::splat::<T>(value)
                }
            }
            impl VecLenCore<4> for ScalarCount<4> {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar, A: VecAlignment>(
                    array: [T; 4],
                ) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<4>>::from_array::<T>(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                ) -> [T; 4] {
                    <A as VecAlignmentCore<4>>::into_array::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar, A: VecAlignment>(
                    vec: &Vector<4, T, A>,
                ) -> &[T; 4] {
                    <A as VecAlignmentCore<4>>::as_array::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                ) -> &mut [T; 4] {
                    <A as VecAlignmentCore<4>>::as_array_mut::<T>(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    index: usize,
                ) -> Option<T> {
                    <A as VecAlignmentCore<4>>::get::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, A>> {
                    <A as VecAlignmentCore<4>>::get_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, A>> {
                    <A as VecAlignmentCore<4>>::get_3::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, A>> {
                    <A as VecAlignmentCore<4>>::get_4::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    index: usize,
                ) -> T {
                    <A as VecAlignmentCore<4>>::get_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, A> {
                    <A as VecAlignmentCore<4>>::get_2_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, A> {
                    <A as VecAlignmentCore<4>>::get_3_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<4>>::get_4_unchecked::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                ) -> Option<&mut T> {
                    <A as VecAlignmentCore<4>>::get_mut::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    <A as VecAlignmentCore<4>>::get_mut_2::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    <A as VecAlignmentCore<4>>::get_mut_3::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    <A as VecAlignmentCore<4>>::get_mut_4::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                ) -> &mut T {
                    <A as VecAlignmentCore<4>>::get_mut_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    <A as VecAlignmentCore<4>>::get_mut_2_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    <A as VecAlignmentCore<4>>::get_mut_3_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    <A as VecAlignmentCore<4>>::get_mut_4_unchecked::<T>(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    <A as VecAlignmentCore<4>>::get_mut_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<4>>::get_mut_1_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    <A as VecAlignmentCore<4>>::get_mut_1_3::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    <A as VecAlignmentCore<4>>::get_mut_2_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<4>>::get_mut_2_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    <A as VecAlignmentCore<4>>::get_mut_3_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    <A as VecAlignmentCore<4>>::get_mut_1_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    <A as VecAlignmentCore<4>>::get_mut_1_1_2::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    <A as VecAlignmentCore<4>>::get_mut_1_2_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    <A as VecAlignmentCore<4>>::get_mut_2_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    <A as VecAlignmentCore<4>>::get_mut_1_1_1_1::<T>(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<4, T, A>> {
                    <A as VecAlignmentCore<4>>::with::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Option<Vector<4, T, A>> {
                    <A as VecAlignmentCore<4>>::with_2::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Option<Vector<4, T, A>> {
                    <A as VecAlignmentCore<4>>::with_3::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar, A: VecAlignment>(
                    vec: Vector<4, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Option<Vector<4, T, A>> {
                    <A as VecAlignmentCore<4>>::with_4::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<4, T, A>,
                    index: usize,
                    value: T,
                ) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<4>>::with_unchecked::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<4, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<
                        4,
                    >>::with_2_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<4, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<
                        4,
                    >>::with_3_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar, A: VecAlignment>(
                    mut vec: Vector<4, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<
                        4,
                    >>::with_4_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<4>>::set::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<4>>::set_2::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<4>>::set_3::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) -> Result<(), ()> {
                    <A as VecAlignmentCore<4>>::set_4::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    index: usize,
                    value: T,
                ) {
                    <A as VecAlignmentCore<4>>::set_unchecked::<T>(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 2],
                    values: Vector<2, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        4,
                    >>::set_2_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 3],
                    values: Vector<3, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        4,
                    >>::set_3_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar, A: VecAlignment>(
                    vec: &mut Vector<4, T, A>,
                    indicies: [usize; 4],
                    values: Vector<4, T, A>,
                ) {
                    <A as VecAlignmentCore<
                        4,
                    >>::set_4_unchecked::<T>(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar, A: VecAlignment>(value: T) -> Vector<4, T, A> {
                    <A as VecAlignmentCore<4>>::splat::<T>(value)
                }
            }
            pub(super) trait VecAlignmentCore<
                const N: usize,
            >: alignment_seal::VecAlignment
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[allow(missing_docs)]
                fn from_array<T: Scalar>(array: [T; N]) -> Vector<N, T, Self>;
                #[allow(missing_docs)]
                fn into_array<T: Scalar>(vec: Vector<N, T, Self>) -> [T; N];
                #[allow(missing_docs)]
                fn as_array<T: Scalar>(vec: &Vector<N, T, Self>) -> &[T; N];
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar>(vec: &mut Vector<N, T, Self>) -> &mut [T; N];
                #[allow(missing_docs)]
                fn get<T: Scalar>(vec: Vector<N, T, Self>, index: usize) -> Option<T>;
                #[allow(missing_docs)]
                fn get_2<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, Self>>;
                #[allow(missing_docs)]
                fn get_3<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, Self>>;
                #[allow(missing_docs)]
                fn get_4<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, Self>>;
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    index: usize,
                ) -> T;
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, Self>;
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, Self>;
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, Self>;
                #[allow(missing_docs)]
                fn get_mut<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                ) -> Option<&mut T>;
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>>;
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>>;
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>>;
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                ) -> &mut T;
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                ) -> &mut Vec2P<T>;
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                ) -> &mut Vec3P<T>;
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                ) -> &mut Vec4P<T>;
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)>;
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)>;
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)>;
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)>;
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)>;
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)>;
                #[allow(missing_docs)]
                fn with<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<N, T, Self>>;
                #[allow(missing_docs)]
                fn with_2<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Option<Vector<N, T, Self>>;
                #[allow(missing_docs)]
                fn with_3<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Option<Vector<N, T, Self>>;
                #[allow(missing_docs)]
                fn with_4<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Option<Vector<N, T, Self>>;
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    index: usize,
                    value: T,
                ) -> Vector<N, T, Self>;
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Vector<N, T, Self>;
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Vector<N, T, Self>;
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar>(
                    vec: Vector<N, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Vector<N, T, Self>;
                #[allow(missing_docs)]
                fn set<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()>;
                #[allow(missing_docs)]
                fn set_2<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Result<(), ()>;
                #[allow(missing_docs)]
                fn set_3<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Result<(), ()>;
                #[allow(missing_docs)]
                fn set_4<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Result<(), ()>;
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    index: usize,
                    value: T,
                );
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                );
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                );
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar>(
                    vec: &mut Vector<N, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                );
                #[allow(missing_docs)]
                fn splat<T: Scalar>(value: T) -> Vector<N, T, Self>;
            }
            impl VecAlignmentCore<2> for VecAligned {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar>(array: [T; 2]) -> Vector<2, T, Self> {
                    T::aligned_vec2_from_array(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar>(vec: Vector<2, T, Self>) -> [T; 2] {
                    T::aligned_vec2_into_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar>(vec: &Vector<2, T, Self>) -> &[T; 2] {
                    T::aligned_vec2_as_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar>(vec: &mut Vector<2, T, Self>) -> &mut [T; 2] {
                    T::aligned_vec2_as_array_mut(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar>(vec: Vector<2, T, Self>, index: usize) -> Option<T> {
                    T::aligned_vec2_get(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, Self>> {
                    T::aligned_vec2_get_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, Self>> {
                    T::aligned_vec2_get_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, Self>> {
                    T::aligned_vec2_get_4(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    index: usize,
                ) -> T {
                    T::aligned_vec2_get_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, Self> {
                    T::aligned_vec2_get_2_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, Self> {
                    T::aligned_vec2_get_3_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, Self> {
                    T::aligned_vec2_get_4_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> Option<&mut T> {
                    T::aligned_vec2_get_mut(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    T::aligned_vec2_get_mut_2(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    T::aligned_vec2_get_mut_3(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    T::aligned_vec2_get_mut_4(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> &mut T {
                    T::aligned_vec2_get_mut_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    T::aligned_vec2_get_mut_2_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    T::aligned_vec2_get_mut_3_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    T::aligned_vec2_get_mut_4_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    T::aligned_vec2_get_mut_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    T::aligned_vec2_get_mut_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    T::aligned_vec2_get_mut_1_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    T::aligned_vec2_get_mut_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    T::aligned_vec2_get_mut_2_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    T::aligned_vec2_get_mut_3_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    T::aligned_vec2_get_mut_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    T::aligned_vec2_get_mut_1_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    T::aligned_vec2_get_mut_1_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    T::aligned_vec2_get_mut_2_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    T::aligned_vec2_get_mut_1_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<2, T, Self>> {
                    T::aligned_vec2_with(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Option<Vector<2, T, Self>> {
                    T::aligned_vec2_with_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Option<Vector<2, T, Self>> {
                    T::aligned_vec2_with_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Option<Vector<2, T, Self>> {
                    T::aligned_vec2_with_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar>(
                    mut vec: Vector<2, T, Self>,
                    index: usize,
                    value: T,
                ) -> Vector<2, T, Self> {
                    T::aligned_vec2_with_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar>(
                    mut vec: Vector<2, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Vector<2, T, Self> {
                    T::aligned_vec2_with_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar>(
                    mut vec: Vector<2, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Vector<2, T, Self> {
                    T::aligned_vec2_with_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar>(
                    mut vec: Vector<2, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Vector<2, T, Self> {
                    T::aligned_vec2_with_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    T::aligned_vec2_set(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec2_set_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec2_set_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec2_set_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                    value: T,
                ) {
                    T::aligned_vec2_set_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) {
                    T::aligned_vec2_set_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) {
                    T::aligned_vec2_set_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) {
                    T::aligned_vec2_set_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar>(value: T) -> Vector<2, T, Self> {
                    T::aligned_vec2_splat(value)
                }
            }
            impl VecAlignmentCore<2> for VecPacked {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar>(array: [T; 2]) -> Vector<2, T, Self> {
                    T::packed_vec2_from_array(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar>(vec: Vector<2, T, Self>) -> [T; 2] {
                    T::packed_vec2_into_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar>(vec: &Vector<2, T, Self>) -> &[T; 2] {
                    T::packed_vec2_as_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar>(vec: &mut Vector<2, T, Self>) -> &mut [T; 2] {
                    T::packed_vec2_as_array_mut(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar>(vec: Vector<2, T, Self>, index: usize) -> Option<T> {
                    T::packed_vec2_get(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, Self>> {
                    T::packed_vec2_get_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, Self>> {
                    T::packed_vec2_get_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, Self>> {
                    T::packed_vec2_get_4(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    index: usize,
                ) -> T {
                    T::packed_vec2_get_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, Self> {
                    T::packed_vec2_get_2_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, Self> {
                    T::packed_vec2_get_3_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, Self> {
                    T::packed_vec2_get_4_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> Option<&mut T> {
                    T::packed_vec2_get_mut(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    T::packed_vec2_get_mut_2(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    T::packed_vec2_get_mut_3(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    T::packed_vec2_get_mut_4(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> &mut T {
                    T::packed_vec2_get_mut_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    T::packed_vec2_get_mut_2_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    T::packed_vec2_get_mut_3_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    T::packed_vec2_get_mut_4_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    T::packed_vec2_get_mut_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    T::packed_vec2_get_mut_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    T::packed_vec2_get_mut_1_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    T::packed_vec2_get_mut_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    T::packed_vec2_get_mut_2_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    T::packed_vec2_get_mut_3_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    T::packed_vec2_get_mut_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    T::packed_vec2_get_mut_1_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    T::packed_vec2_get_mut_1_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    T::packed_vec2_get_mut_2_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    T::packed_vec2_get_mut_1_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<2, T, Self>> {
                    T::packed_vec2_with(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Option<Vector<2, T, Self>> {
                    T::packed_vec2_with_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Option<Vector<2, T, Self>> {
                    T::packed_vec2_with_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar>(
                    vec: Vector<2, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Option<Vector<2, T, Self>> {
                    T::packed_vec2_with_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar>(
                    mut vec: Vector<2, T, Self>,
                    index: usize,
                    value: T,
                ) -> Vector<2, T, Self> {
                    T::packed_vec2_with_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar>(
                    mut vec: Vector<2, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Vector<2, T, Self> {
                    T::packed_vec2_with_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar>(
                    mut vec: Vector<2, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Vector<2, T, Self> {
                    T::packed_vec2_with_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar>(
                    mut vec: Vector<2, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Vector<2, T, Self> {
                    T::packed_vec2_with_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    T::packed_vec2_set(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec2_set_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec2_set_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec2_set_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    index: usize,
                    value: T,
                ) {
                    T::packed_vec2_set_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) {
                    T::packed_vec2_set_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) {
                    T::packed_vec2_set_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar>(
                    vec: &mut Vector<2, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) {
                    T::packed_vec2_set_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar>(value: T) -> Vector<2, T, Self> {
                    T::packed_vec2_splat(value)
                }
            }
            impl VecAlignmentCore<3> for VecAligned {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar>(array: [T; 3]) -> Vector<3, T, Self> {
                    T::aligned_vec3_from_array(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar>(vec: Vector<3, T, Self>) -> [T; 3] {
                    T::aligned_vec3_into_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar>(vec: &Vector<3, T, Self>) -> &[T; 3] {
                    T::aligned_vec3_as_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar>(vec: &mut Vector<3, T, Self>) -> &mut [T; 3] {
                    T::aligned_vec3_as_array_mut(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar>(vec: Vector<3, T, Self>, index: usize) -> Option<T> {
                    T::aligned_vec3_get(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, Self>> {
                    T::aligned_vec3_get_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, Self>> {
                    T::aligned_vec3_get_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, Self>> {
                    T::aligned_vec3_get_4(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    index: usize,
                ) -> T {
                    T::aligned_vec3_get_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, Self> {
                    T::aligned_vec3_get_2_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, Self> {
                    T::aligned_vec3_get_3_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, Self> {
                    T::aligned_vec3_get_4_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> Option<&mut T> {
                    T::aligned_vec3_get_mut(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    T::aligned_vec3_get_mut_2(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    T::aligned_vec3_get_mut_3(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    T::aligned_vec3_get_mut_4(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> &mut T {
                    T::aligned_vec3_get_mut_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    T::aligned_vec3_get_mut_2_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    T::aligned_vec3_get_mut_3_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    T::aligned_vec3_get_mut_4_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    T::aligned_vec3_get_mut_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    T::aligned_vec3_get_mut_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    T::aligned_vec3_get_mut_1_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    T::aligned_vec3_get_mut_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    T::aligned_vec3_get_mut_2_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    T::aligned_vec3_get_mut_3_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    T::aligned_vec3_get_mut_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    T::aligned_vec3_get_mut_1_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    T::aligned_vec3_get_mut_1_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    T::aligned_vec3_get_mut_2_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    T::aligned_vec3_get_mut_1_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<3, T, Self>> {
                    T::aligned_vec3_with(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Option<Vector<3, T, Self>> {
                    T::aligned_vec3_with_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Option<Vector<3, T, Self>> {
                    T::aligned_vec3_with_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Option<Vector<3, T, Self>> {
                    T::aligned_vec3_with_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar>(
                    mut vec: Vector<3, T, Self>,
                    index: usize,
                    value: T,
                ) -> Vector<3, T, Self> {
                    T::aligned_vec3_with_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar>(
                    mut vec: Vector<3, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Vector<3, T, Self> {
                    T::aligned_vec3_with_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar>(
                    mut vec: Vector<3, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Vector<3, T, Self> {
                    T::aligned_vec3_with_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar>(
                    mut vec: Vector<3, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Vector<3, T, Self> {
                    T::aligned_vec3_with_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    T::aligned_vec3_set(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec3_set_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec3_set_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec3_set_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                    value: T,
                ) {
                    T::aligned_vec3_set_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) {
                    T::aligned_vec3_set_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) {
                    T::aligned_vec3_set_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) {
                    T::aligned_vec3_set_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar>(value: T) -> Vector<3, T, Self> {
                    T::aligned_vec3_splat(value)
                }
            }
            impl VecAlignmentCore<3> for VecPacked {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar>(array: [T; 3]) -> Vector<3, T, Self> {
                    T::packed_vec3_from_array(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar>(vec: Vector<3, T, Self>) -> [T; 3] {
                    T::packed_vec3_into_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar>(vec: &Vector<3, T, Self>) -> &[T; 3] {
                    T::packed_vec3_as_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar>(vec: &mut Vector<3, T, Self>) -> &mut [T; 3] {
                    T::packed_vec3_as_array_mut(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar>(vec: Vector<3, T, Self>, index: usize) -> Option<T> {
                    T::packed_vec3_get(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, Self>> {
                    T::packed_vec3_get_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, Self>> {
                    T::packed_vec3_get_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, Self>> {
                    T::packed_vec3_get_4(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    index: usize,
                ) -> T {
                    T::packed_vec3_get_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, Self> {
                    T::packed_vec3_get_2_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, Self> {
                    T::packed_vec3_get_3_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, Self> {
                    T::packed_vec3_get_4_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> Option<&mut T> {
                    T::packed_vec3_get_mut(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    T::packed_vec3_get_mut_2(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    T::packed_vec3_get_mut_3(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    T::packed_vec3_get_mut_4(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> &mut T {
                    T::packed_vec3_get_mut_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    T::packed_vec3_get_mut_2_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    T::packed_vec3_get_mut_3_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    T::packed_vec3_get_mut_4_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    T::packed_vec3_get_mut_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    T::packed_vec3_get_mut_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    T::packed_vec3_get_mut_1_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    T::packed_vec3_get_mut_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    T::packed_vec3_get_mut_2_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    T::packed_vec3_get_mut_3_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    T::packed_vec3_get_mut_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    T::packed_vec3_get_mut_1_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    T::packed_vec3_get_mut_1_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    T::packed_vec3_get_mut_2_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    T::packed_vec3_get_mut_1_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<3, T, Self>> {
                    T::packed_vec3_with(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Option<Vector<3, T, Self>> {
                    T::packed_vec3_with_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Option<Vector<3, T, Self>> {
                    T::packed_vec3_with_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar>(
                    vec: Vector<3, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Option<Vector<3, T, Self>> {
                    T::packed_vec3_with_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar>(
                    mut vec: Vector<3, T, Self>,
                    index: usize,
                    value: T,
                ) -> Vector<3, T, Self> {
                    T::packed_vec3_with_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar>(
                    mut vec: Vector<3, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Vector<3, T, Self> {
                    T::packed_vec3_with_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar>(
                    mut vec: Vector<3, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Vector<3, T, Self> {
                    T::packed_vec3_with_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar>(
                    mut vec: Vector<3, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Vector<3, T, Self> {
                    T::packed_vec3_with_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    T::packed_vec3_set(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec3_set_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec3_set_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec3_set_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    index: usize,
                    value: T,
                ) {
                    T::packed_vec3_set_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) {
                    T::packed_vec3_set_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) {
                    T::packed_vec3_set_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar>(
                    vec: &mut Vector<3, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) {
                    T::packed_vec3_set_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar>(value: T) -> Vector<3, T, Self> {
                    T::packed_vec3_splat(value)
                }
            }
            impl VecAlignmentCore<4> for VecAligned {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar>(array: [T; 4]) -> Vector<4, T, Self> {
                    T::aligned_vec4_from_array(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar>(vec: Vector<4, T, Self>) -> [T; 4] {
                    T::aligned_vec4_into_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar>(vec: &Vector<4, T, Self>) -> &[T; 4] {
                    T::aligned_vec4_as_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar>(vec: &mut Vector<4, T, Self>) -> &mut [T; 4] {
                    T::aligned_vec4_as_array_mut(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar>(vec: Vector<4, T, Self>, index: usize) -> Option<T> {
                    T::aligned_vec4_get(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, Self>> {
                    T::aligned_vec4_get_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, Self>> {
                    T::aligned_vec4_get_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, Self>> {
                    T::aligned_vec4_get_4(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    index: usize,
                ) -> T {
                    T::aligned_vec4_get_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, Self> {
                    T::aligned_vec4_get_2_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, Self> {
                    T::aligned_vec4_get_3_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, Self> {
                    T::aligned_vec4_get_4_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> Option<&mut T> {
                    T::aligned_vec4_get_mut(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    T::aligned_vec4_get_mut_2(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    T::aligned_vec4_get_mut_3(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    T::aligned_vec4_get_mut_4(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> &mut T {
                    T::aligned_vec4_get_mut_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    T::aligned_vec4_get_mut_2_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    T::aligned_vec4_get_mut_3_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    T::aligned_vec4_get_mut_4_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    T::aligned_vec4_get_mut_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    T::aligned_vec4_get_mut_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    T::aligned_vec4_get_mut_1_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    T::aligned_vec4_get_mut_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    T::aligned_vec4_get_mut_2_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    T::aligned_vec4_get_mut_3_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    T::aligned_vec4_get_mut_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    T::aligned_vec4_get_mut_1_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    T::aligned_vec4_get_mut_1_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    T::aligned_vec4_get_mut_2_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    T::aligned_vec4_get_mut_1_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<4, T, Self>> {
                    T::aligned_vec4_with(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Option<Vector<4, T, Self>> {
                    T::aligned_vec4_with_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Option<Vector<4, T, Self>> {
                    T::aligned_vec4_with_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Option<Vector<4, T, Self>> {
                    T::aligned_vec4_with_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar>(
                    mut vec: Vector<4, T, Self>,
                    index: usize,
                    value: T,
                ) -> Vector<4, T, Self> {
                    T::aligned_vec4_with_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar>(
                    mut vec: Vector<4, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Vector<4, T, Self> {
                    T::aligned_vec4_with_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar>(
                    mut vec: Vector<4, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Vector<4, T, Self> {
                    T::aligned_vec4_with_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar>(
                    mut vec: Vector<4, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Vector<4, T, Self> {
                    T::aligned_vec4_with_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    T::aligned_vec4_set(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec4_set_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec4_set_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Result<(), ()> {
                    T::aligned_vec4_set_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                    value: T,
                ) {
                    T::aligned_vec4_set_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) {
                    T::aligned_vec4_set_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) {
                    T::aligned_vec4_set_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) {
                    T::aligned_vec4_set_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar>(value: T) -> Vector<4, T, Self> {
                    T::aligned_vec4_splat(value)
                }
            }
            impl VecAlignmentCore<4> for VecPacked {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn from_array<T: Scalar>(array: [T; 4]) -> Vector<4, T, Self> {
                    T::packed_vec4_from_array(array)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn into_array<T: Scalar>(vec: Vector<4, T, Self>) -> [T; 4] {
                    T::packed_vec4_into_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array<T: Scalar>(vec: &Vector<4, T, Self>) -> &[T; 4] {
                    T::packed_vec4_as_array(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn as_array_mut<T: Scalar>(vec: &mut Vector<4, T, Self>) -> &mut [T; 4] {
                    T::packed_vec4_as_array_mut(vec)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get<T: Scalar>(vec: Vector<4, T, Self>, index: usize) -> Option<T> {
                    T::packed_vec4_get(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_2<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<Vector<2, T, Self>> {
                    T::packed_vec4_get_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_3<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<Vector<3, T, Self>> {
                    T::packed_vec4_get_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_4<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<Vector<4, T, Self>> {
                    T::packed_vec4_get_4(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_unchecked<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    index: usize,
                ) -> T {
                    T::packed_vec4_get_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_2_unchecked<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Vector<2, T, Self> {
                    T::packed_vec4_get_2_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_3_unchecked<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Vector<3, T, Self> {
                    T::packed_vec4_get_3_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_4_unchecked<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 4],
                ) -> Vector<4, T, Self> {
                    T::packed_vec4_get_4_unchecked(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> Option<&mut T> {
                    T::packed_vec4_get_mut(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec2P<T>> {
                    T::packed_vec4_get_mut_2(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec3P<T>> {
                    T::packed_vec4_get_mut_3(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_4<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> Option<&mut Vec4P<T>> {
                    T::packed_vec4_get_mut_4(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> &mut T {
                    T::packed_vec4_get_mut_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_2_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> &mut Vec2P<T> {
                    T::packed_vec4_get_mut_2_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_3_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> &mut Vec3P<T> {
                    T::packed_vec4_get_mut_3_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn get_mut_4_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                ) -> &mut Vec4P<T> {
                    T::packed_vec4_get_mut_4_unchecked(vec, index)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut T)> {
                    T::packed_vec4_get_mut_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec2P<T>)> {
                    T::packed_vec4_get_mut_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_3<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut T, &mut Vec3P<T>)> {
                    T::packed_vec4_get_mut_1_3(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut T)> {
                    T::packed_vec4_get_mut_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
                    T::packed_vec4_get_mut_2_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_3_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                ) -> Option<(&mut Vec3P<T>, &mut T)> {
                    T::packed_vec4_get_mut_3_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut T)> {
                    T::packed_vec4_get_mut_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
                    T::packed_vec4_get_mut_1_1_2(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_2_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
                    T::packed_vec4_get_mut_1_2_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_2_1_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                ) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
                    T::packed_vec4_get_mut_2_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn get_mut_1_1_1_1<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 4],
                ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
                    T::packed_vec4_get_mut_1_1_1_1(vec, indicies)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    index: usize,
                    value: T,
                ) -> Option<Vector<4, T, Self>> {
                    T::packed_vec4_with(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_2<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Option<Vector<4, T, Self>> {
                    T::packed_vec4_with_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_3<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Option<Vector<4, T, Self>> {
                    T::packed_vec4_with_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn with_4<T: Scalar>(
                    vec: Vector<4, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Option<Vector<4, T, Self>> {
                    T::packed_vec4_with_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_unchecked<T: Scalar>(
                    mut vec: Vector<4, T, Self>,
                    index: usize,
                    value: T,
                ) -> Vector<4, T, Self> {
                    T::packed_vec4_with_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_2_unchecked<T: Scalar>(
                    mut vec: Vector<4, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Vector<4, T, Self> {
                    T::packed_vec4_with_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_3_unchecked<T: Scalar>(
                    mut vec: Vector<4, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Vector<4, T, Self> {
                    T::packed_vec4_with_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn with_4_unchecked<T: Scalar>(
                    mut vec: Vector<4, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Vector<4, T, Self> {
                    T::packed_vec4_with_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                    value: T,
                ) -> Result<(), ()> {
                    T::packed_vec4_set(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_2<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec4_set_2(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_3<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec4_set_3(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn set_4<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) -> Result<(), ()> {
                    T::packed_vec4_set_4(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    index: usize,
                    value: T,
                ) {
                    T::packed_vec4_set_unchecked(vec, index, value)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_2_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 2],
                    values: Vector<2, T, Self>,
                ) {
                    T::packed_vec4_set_2_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_3_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 3],
                    values: Vector<3, T, Self>,
                ) {
                    T::packed_vec4_set_3_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                unsafe fn set_4_unchecked<T: Scalar>(
                    vec: &mut Vector<4, T, Self>,
                    indicies: [usize; 4],
                    values: Vector<4, T, Self>,
                ) {
                    T::packed_vec4_set_4_unchecked(vec, indicies, values)
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn splat<T: Scalar>(value: T) -> Vector<4, T, Self> {
                    T::packed_vec4_splat(value)
                }
            }
        }
        mod default {
            #[allow(unused_imports)]
            use crate::vector::{alignment::*, inner::*, length::*, *};
            const _: () = {};
            impl<const N: usize, T: ScalarDefault, A: VecAlignment> Vector<N, T, A>
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[inline(always)]
                #[allow(unused_mut)]
                fn default() -> Self {
                    <ScalarCount<N> as VecLenDefault<N>>::default::<T, A>()
                }
            }
            pub trait ScalarDefault: Scalar + Default {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec2_default() -> Vector<2, Self, VecAligned> {
                    Vector::from_array([<Self as Default>::default(); 2])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec3_default() -> Vector<3, Self, VecAligned> {
                    Vector::from_array([<Self as Default>::default(); 3])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn aligned_vec4_default() -> Vector<4, Self, VecAligned> {
                    Vector::from_array([<Self as Default>::default(); 4])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec2_default() -> Vector<2, Self, VecPacked> {
                    Vector::from_array([<Self as Default>::default(); 2])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec3_default() -> Vector<3, Self, VecPacked> {
                    Vector::from_array([<Self as Default>::default(); 3])
                }
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn packed_vec4_default() -> Vector<4, Self, VecPacked> {
                    Vector::from_array([<Self as Default>::default(); 4])
                }
            }
            pub(super) trait VecLenDefault<const N: usize>: VecLenInnerVec
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[allow(missing_docs)]
                fn default<T: ScalarDefault, A: VecAlignment>() -> Vector<N, T, A>;
            }
            impl VecLenDefault<2> for ScalarCount<2> {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault, A: VecAlignment>() -> Vector<2, T, A> {
                    <A as VecAlignmentDefault<2>>::default::<T>()
                }
            }
            impl VecLenDefault<3> for ScalarCount<3> {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault, A: VecAlignment>() -> Vector<3, T, A> {
                    <A as VecAlignmentDefault<3>>::default::<T>()
                }
            }
            impl VecLenDefault<4> for ScalarCount<4> {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault, A: VecAlignment>() -> Vector<4, T, A> {
                    <A as VecAlignmentDefault<4>>::default::<T>()
                }
            }
            pub(super) trait VecAlignmentDefault<
                const N: usize,
            >: alignment_seal::VecAlignment
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[allow(missing_docs)]
                fn default<T: ScalarDefault>() -> Vector<N, T, Self>;
            }
            impl VecAlignmentDefault<2> for VecAligned {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault>() -> Vector<2, T, Self> {
                    T::aligned_vec2_default()
                }
            }
            impl VecAlignmentDefault<2> for VecPacked {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault>() -> Vector<2, T, Self> {
                    T::packed_vec2_default()
                }
            }
            impl VecAlignmentDefault<3> for VecAligned {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault>() -> Vector<3, T, Self> {
                    T::aligned_vec3_default()
                }
            }
            impl VecAlignmentDefault<3> for VecPacked {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault>() -> Vector<3, T, Self> {
                    T::packed_vec3_default()
                }
            }
            impl VecAlignmentDefault<4> for VecAligned {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault>() -> Vector<4, T, Self> {
                    T::aligned_vec4_default()
                }
            }
            impl VecAlignmentDefault<4> for VecPacked {
                #[allow(unused_mut)]
                #[allow(missing_docs)]
                fn default<T: ScalarDefault>() -> Vector<4, T, Self> {
                    T::packed_vec4_default()
                }
            }
            use crate::vector::{ScalarCount, VecAlignment, VecLen, Vector};
            impl<const N: usize, T: ScalarDefault, A: VecAlignment> Default
            for Vector<N, T, A>
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn default() -> Self {
                    Self::default()
                }
            }
        }
        mod test {
            use std::ops::*;
            #[allow(unused_imports)]
            use crate::vector::{alignment::*, inner::*, length::*, *};
            const _: () = { (/*ERROR*/) };
            impl<
                const N: usize,
                T: ScalarAdd<Rhs>,
                A: VecAlignment,
                Rhs: Scalar,
            > Vector<N, T, A>
            where
                ScalarCount<N>: VecLen<N>,
            {}
            pub trait ScalarAdd<Rhs: Scalar>: Scalar + Add<Rhs, Output: Scalar> {}
            pub(super) trait VecLenAdd<const N: usize>: VecLenInnerVec
            where
                ScalarCount<N>: VecLen<N>,
            {}
            impl VecLenAdd<2> for ScalarCount<2> {}
            impl VecLenAdd<3> for ScalarCount<3> {}
            impl VecLenAdd<4> for ScalarCount<4> {}
            pub(super) trait VecAlignmentAdd<
                const N: usize,
            >: alignment_seal::VecAlignment
            where
                ScalarCount<N>: VecLen<N>,
            {}
            impl VecAlignmentAdd<2> for VecAligned {}
            impl VecAlignmentAdd<2> for VecPacked {}
            impl VecAlignmentAdd<3> for VecAligned {}
            impl VecAlignmentAdd<3> for VecPacked {}
            impl VecAlignmentAdd<4> for VecAligned {}
            impl VecAlignmentAdd<4> for VecPacked {}
        }
        pub use core::*;
        pub use default::*;
        pub use test::*;
        use super::{ScalarCount, VecAligned, VecLen, VecPacked};
        pub(crate) mod scalar_traits {
            pub use super::*;
        }
        #[allow(private_bounds)]
        pub(super) trait VecLenInterfaces<
            const N: usize,
        >: VecLenCore<N> + VecLenDefault<N>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        impl VecLenInterfaces<2> for ScalarCount<2> {}
        impl VecLenInterfaces<3> for ScalarCount<3> {}
        impl VecLenInterfaces<4> for ScalarCount<4> {}
        #[allow(private_bounds)]
        pub(super) trait VecAlignmentInterfaces<
            const N: usize,
        >: VecAlignmentCore<N> + VecAlignmentDefault<N>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        impl VecAlignmentInterfaces<2> for VecAligned {}
        impl VecAlignmentInterfaces<3> for VecAligned {}
        impl VecAlignmentInterfaces<4> for VecAligned {}
        impl VecAlignmentInterfaces<2> for VecPacked {}
        impl VecAlignmentInterfaces<3> for VecPacked {}
        impl VecAlignmentInterfaces<4> for VecPacked {}
    }
    mod impl_construct {
        use super::*;
        mod copy {
            use super::*;
            impl<const N: usize, T: Scalar, S: VecAlignment> Clone for Vector<N, T, S>
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[inline(always)]
                fn clone(&self) -> Self {
                    Self { inner: self.inner }
                }
            }
            impl<const N: usize, T: Scalar, S: VecAlignment> Copy for Vector<N, T, S>
            where
                ScalarCount<N>: VecLen<N>,
            {}
        }
        mod debug {
            use std::fmt::{Debug, Formatter, Result};
            use super::*;
            impl<const N: usize, T: Scalar, S: VecAlignment> Debug for Vector<N, T, S>
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[inline(always)]
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    self.inner.fmt(f)
                }
            }
        }
        mod display {
            use std::fmt::{self, Display, Formatter};
            use super::*;
            impl<const N: usize, T: Scalar, S: VecAlignment> Display for Vector<N, T, S>
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    f.write_fmt(
                        format_args!(
                            "({0})",
                            self.into_array().map(|c| c.to_string()).join(", "),
                        ),
                    )
                }
            }
        }
        mod partial_eq {
            use super::*;
            impl<const N: usize, T: Scalar, S: VecAlignment> PartialEq
            for Vector<N, T, S>
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[inline(always)]
                fn eq(&self, other: &Self) -> bool {
                    self.into_array().eq(&other.into_array())
                }
            }
        }
    }
    use crate::{construct::*, scalar::*};
    use alignment::*;
    use length::*;
    /// Statically-lengthed vector generic over N - length, T - Scalar, and A - Alignment.
    ///
    /// Storage affects the inner implementation of vector fns.
    ///
    /// Only use this type when being generic over N, T, and S.
    /// there are simpler type aliases to this type for when being not generic.
    ///
    /// - ```Vector2<T, S>```, ```Vector3<T, S>```, and ```Vector4<T, S>``` fill N.
    /// - ```VecN<N, T>```, ```Vec2<T>```, ```Vec3<T>```, and ```Vec4<T>``` use the default storage [```VecAligned```].
    /// - ```VecNP<N, T>```, ```Vec2P<T>```, ```Vec3P<T>```, and ```Vec4P<T>``` use the non-default storage [```VecPacked```].
    /// - [```scalar::aliases```](crate::scalar::aliases) contains aliases for each primitive.
    ///
    /// # Examples
    /// ```
    /// fn print_vec<const N: usize, T: Scalar, S: VecStorage>(vec: Vector<N, T, S>)
    /// where
    ///     ScalarCount<N>: VecLen<N>, // Required by Vector to ensure that N is either 2, 3, or 4.
    /// {
    ///     println!("{vec}")
    /// }
    /// ```
    #[repr(transparent)]
    pub struct Vector<const N: usize, T: Scalar, A: alignment_seal::VecAlignment>
    where
        length::ScalarCount<N>: length::VecLen<N>,
    {
        inner: inner::InnerVector<N, T, A>,
    }
    /// type alias to [```Vector```]```<2, T, VecAligned>```
    pub type Vec2<T> = Vector<2, T, VecAligned>;
    /// type alias to [```Vector```]```<3, T, VecAligned>```
    pub type Vec3<T> = Vector<3, T, VecAligned>;
    /// type alias to [```Vector```]```<4, T, VecAligned>```
    pub type Vec4<T> = Vector<4, T, VecAligned>;
    /// - type alias to [```Vector```]```<2, T, VecPacked>```
    /// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec2```].
    pub type Vec2P<T> = Vector<2, T, VecPacked>;
    /// - type alias to [```Vector```]```<3, T, VecPacked>```
    /// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec3```].
    pub type Vec3P<T> = Vector<3, T, VecPacked>;
    /// - type alias to [```Vector```]```<4, T, VecPacked>```
    /// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec4```].
    pub type Vec4P<T> = Vector<4, T, VecPacked>;
}
#[cfg(feature = "primitive_aliases")]
pub mod primitive_aliases {
    //! Type aliases for primitives.
    //! For example for (```FVec3```, ```IVec2```, ```BVecN<N>```
    use super::*;
    use ggmath_proc_macros::scalar_aliases;
    /// mathamatical type-aliases for [```f32```]
    pub mod f32 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```f32```] [```Vector```]
        pub type FVector<const N: usize, A> = Vector<N, f32, A>;
        /// type-alias for an [```f32```] [```Vec2```]
        pub type FVec2 = Vec2<f32>;
        /// type-alias for an [```f32```] [```Vec3```]
        pub type FVec3 = Vec3<f32>;
        /// type-alias for an [```f32```] [```Vec4```]
        pub type FVec4 = Vec4<f32>;
        /// type-alias for an [```f32```] [```Vec2P```]
        pub type FVec2P = Vec2P<f32>;
        /// type-alias for an [```f32```] [```Vec3P```]
        pub type FVec3P = Vec3P<f32>;
        /// type-alias for an [```f32```] [```Vec4P```]
        pub type FVec4P = Vec4P<f32>;
        /// type-alias for [```vector```] with [```f32```]
        pub fn fvector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, f32>,
        ) -> Vector<N, f32, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, f32, A>(value)
        }
        /// type-alias for [```vec2```] with [```f32```]
        pub fn fvec2(value: impl IntoVector<2, f32>) -> Vec2<f32> {
            vec2::<f32>(value)
        }
        /// type-alias for [```vec3```] with [```f32```]
        pub fn fvec3(value: impl IntoVector<3, f32>) -> Vec3<f32> {
            vec3::<f32>(value)
        }
        /// type-alias for [```vec4```] with [```f32```]
        pub fn fvec4(value: impl IntoVector<4, f32>) -> Vec4<f32> {
            vec4::<f32>(value)
        }
        /// type-alias for [```vec2p```] with [```f32```]
        pub fn fvec2p(value: impl IntoVector<2, f32>) -> Vec2P<f32> {
            vec2p::<f32>(value)
        }
        /// type-alias for [```vec3p```] with [```f32```]
        pub fn fvec3p(value: impl IntoVector<3, f32>) -> Vec3P<f32> {
            vec3p::<f32>(value)
        }
        /// type-alias for [```vec4p```] with [```f32```]
        pub fn fvec4p(value: impl IntoVector<4, f32>) -> Vec4P<f32> {
            vec4p::<f32>(value)
        }
    }
    /// mathamatical type-aliases for [```f64```]
    pub mod f64 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```f64```] [```Vector```]
        pub type DVector<const N: usize, A> = Vector<N, f64, A>;
        /// type-alias for an [```f64```] [```Vec2```]
        pub type DVec2 = Vec2<f64>;
        /// type-alias for an [```f64```] [```Vec3```]
        pub type DVec3 = Vec3<f64>;
        /// type-alias for an [```f64```] [```Vec4```]
        pub type DVec4 = Vec4<f64>;
        /// type-alias for an [```f64```] [```Vec2P```]
        pub type DVec2P = Vec2P<f64>;
        /// type-alias for an [```f64```] [```Vec3P```]
        pub type DVec3P = Vec3P<f64>;
        /// type-alias for an [```f64```] [```Vec4P```]
        pub type DVec4P = Vec4P<f64>;
        /// type-alias for [```vector```] with [```f64```]
        pub fn dvector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, f64>,
        ) -> Vector<N, f64, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, f64, A>(value)
        }
        /// type-alias for [```vec2```] with [```f64```]
        pub fn dvec2(value: impl IntoVector<2, f64>) -> Vec2<f64> {
            vec2::<f64>(value)
        }
        /// type-alias for [```vec3```] with [```f64```]
        pub fn dvec3(value: impl IntoVector<3, f64>) -> Vec3<f64> {
            vec3::<f64>(value)
        }
        /// type-alias for [```vec4```] with [```f64```]
        pub fn dvec4(value: impl IntoVector<4, f64>) -> Vec4<f64> {
            vec4::<f64>(value)
        }
        /// type-alias for [```vec2p```] with [```f64```]
        pub fn dvec2p(value: impl IntoVector<2, f64>) -> Vec2P<f64> {
            vec2p::<f64>(value)
        }
        /// type-alias for [```vec3p```] with [```f64```]
        pub fn dvec3p(value: impl IntoVector<3, f64>) -> Vec3P<f64> {
            vec3p::<f64>(value)
        }
        /// type-alias for [```vec4p```] with [```f64```]
        pub fn dvec4p(value: impl IntoVector<4, f64>) -> Vec4P<f64> {
            vec4p::<f64>(value)
        }
    }
    /// mathamatical type-aliases for [```u8```]
    pub mod u8 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```u8```] [```Vector```]
        pub type U8Vector<const N: usize, A> = Vector<N, u8, A>;
        /// type-alias for an [```u8```] [```Vec2```]
        pub type U8Vec2 = Vec2<u8>;
        /// type-alias for an [```u8```] [```Vec3```]
        pub type U8Vec3 = Vec3<u8>;
        /// type-alias for an [```u8```] [```Vec4```]
        pub type U8Vec4 = Vec4<u8>;
        /// type-alias for an [```u8```] [```Vec2P```]
        pub type U8Vec2P = Vec2P<u8>;
        /// type-alias for an [```u8```] [```Vec3P```]
        pub type U8Vec3P = Vec3P<u8>;
        /// type-alias for an [```u8```] [```Vec4P```]
        pub type U8Vec4P = Vec4P<u8>;
        /// type-alias for [```vector```] with [```u8```]
        pub fn u8vector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, u8>,
        ) -> Vector<N, u8, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, u8, A>(value)
        }
        /// type-alias for [```vec2```] with [```u8```]
        pub fn u8vec2(value: impl IntoVector<2, u8>) -> Vec2<u8> {
            vec2::<u8>(value)
        }
        /// type-alias for [```vec3```] with [```u8```]
        pub fn u8vec3(value: impl IntoVector<3, u8>) -> Vec3<u8> {
            vec3::<u8>(value)
        }
        /// type-alias for [```vec4```] with [```u8```]
        pub fn u8vec4(value: impl IntoVector<4, u8>) -> Vec4<u8> {
            vec4::<u8>(value)
        }
        /// type-alias for [```vec2p```] with [```u8```]
        pub fn u8vec2p(value: impl IntoVector<2, u8>) -> Vec2P<u8> {
            vec2p::<u8>(value)
        }
        /// type-alias for [```vec3p```] with [```u8```]
        pub fn u8vec3p(value: impl IntoVector<3, u8>) -> Vec3P<u8> {
            vec3p::<u8>(value)
        }
        /// type-alias for [```vec4p```] with [```u8```]
        pub fn u8vec4p(value: impl IntoVector<4, u8>) -> Vec4P<u8> {
            vec4p::<u8>(value)
        }
    }
    /// mathamatical type-aliases for [```u16```]
    pub mod u16 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```u16```] [```Vector```]
        pub type U16Vector<const N: usize, A> = Vector<N, u16, A>;
        /// type-alias for an [```u16```] [```Vec2```]
        pub type U16Vec2 = Vec2<u16>;
        /// type-alias for an [```u16```] [```Vec3```]
        pub type U16Vec3 = Vec3<u16>;
        /// type-alias for an [```u16```] [```Vec4```]
        pub type U16Vec4 = Vec4<u16>;
        /// type-alias for an [```u16```] [```Vec2P```]
        pub type U16Vec2P = Vec2P<u16>;
        /// type-alias for an [```u16```] [```Vec3P```]
        pub type U16Vec3P = Vec3P<u16>;
        /// type-alias for an [```u16```] [```Vec4P```]
        pub type U16Vec4P = Vec4P<u16>;
        /// type-alias for [```vector```] with [```u16```]
        pub fn u16vector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, u16>,
        ) -> Vector<N, u16, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, u16, A>(value)
        }
        /// type-alias for [```vec2```] with [```u16```]
        pub fn u16vec2(value: impl IntoVector<2, u16>) -> Vec2<u16> {
            vec2::<u16>(value)
        }
        /// type-alias for [```vec3```] with [```u16```]
        pub fn u16vec3(value: impl IntoVector<3, u16>) -> Vec3<u16> {
            vec3::<u16>(value)
        }
        /// type-alias for [```vec4```] with [```u16```]
        pub fn u16vec4(value: impl IntoVector<4, u16>) -> Vec4<u16> {
            vec4::<u16>(value)
        }
        /// type-alias for [```vec2p```] with [```u16```]
        pub fn u16vec2p(value: impl IntoVector<2, u16>) -> Vec2P<u16> {
            vec2p::<u16>(value)
        }
        /// type-alias for [```vec3p```] with [```u16```]
        pub fn u16vec3p(value: impl IntoVector<3, u16>) -> Vec3P<u16> {
            vec3p::<u16>(value)
        }
        /// type-alias for [```vec4p```] with [```u16```]
        pub fn u16vec4p(value: impl IntoVector<4, u16>) -> Vec4P<u16> {
            vec4p::<u16>(value)
        }
    }
    /// mathamatical type-aliases for [```u32```]
    pub mod u32 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```u32```] [```Vector```]
        pub type UVector<const N: usize, A> = Vector<N, u32, A>;
        /// type-alias for an [```u32```] [```Vec2```]
        pub type UVec2 = Vec2<u32>;
        /// type-alias for an [```u32```] [```Vec3```]
        pub type UVec3 = Vec3<u32>;
        /// type-alias for an [```u32```] [```Vec4```]
        pub type UVec4 = Vec4<u32>;
        /// type-alias for an [```u32```] [```Vec2P```]
        pub type UVec2P = Vec2P<u32>;
        /// type-alias for an [```u32```] [```Vec3P```]
        pub type UVec3P = Vec3P<u32>;
        /// type-alias for an [```u32```] [```Vec4P```]
        pub type UVec4P = Vec4P<u32>;
        /// type-alias for [```vector```] with [```u32```]
        pub fn uvector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, u32>,
        ) -> Vector<N, u32, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, u32, A>(value)
        }
        /// type-alias for [```vec2```] with [```u32```]
        pub fn uvec2(value: impl IntoVector<2, u32>) -> Vec2<u32> {
            vec2::<u32>(value)
        }
        /// type-alias for [```vec3```] with [```u32```]
        pub fn uvec3(value: impl IntoVector<3, u32>) -> Vec3<u32> {
            vec3::<u32>(value)
        }
        /// type-alias for [```vec4```] with [```u32```]
        pub fn uvec4(value: impl IntoVector<4, u32>) -> Vec4<u32> {
            vec4::<u32>(value)
        }
        /// type-alias for [```vec2p```] with [```u32```]
        pub fn uvec2p(value: impl IntoVector<2, u32>) -> Vec2P<u32> {
            vec2p::<u32>(value)
        }
        /// type-alias for [```vec3p```] with [```u32```]
        pub fn uvec3p(value: impl IntoVector<3, u32>) -> Vec3P<u32> {
            vec3p::<u32>(value)
        }
        /// type-alias for [```vec4p```] with [```u32```]
        pub fn uvec4p(value: impl IntoVector<4, u32>) -> Vec4P<u32> {
            vec4p::<u32>(value)
        }
    }
    /// mathamatical type-aliases for [```u64```]
    pub mod u64 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```u64```] [```Vector```]
        pub type U64Vector<const N: usize, A> = Vector<N, u64, A>;
        /// type-alias for an [```u64```] [```Vec2```]
        pub type U64Vec2 = Vec2<u64>;
        /// type-alias for an [```u64```] [```Vec3```]
        pub type U64Vec3 = Vec3<u64>;
        /// type-alias for an [```u64```] [```Vec4```]
        pub type U64Vec4 = Vec4<u64>;
        /// type-alias for an [```u64```] [```Vec2P```]
        pub type U64Vec2P = Vec2P<u64>;
        /// type-alias for an [```u64```] [```Vec3P```]
        pub type U64Vec3P = Vec3P<u64>;
        /// type-alias for an [```u64```] [```Vec4P```]
        pub type U64Vec4P = Vec4P<u64>;
        /// type-alias for [```vector```] with [```u64```]
        pub fn u64vector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, u64>,
        ) -> Vector<N, u64, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, u64, A>(value)
        }
        /// type-alias for [```vec2```] with [```u64```]
        pub fn u64vec2(value: impl IntoVector<2, u64>) -> Vec2<u64> {
            vec2::<u64>(value)
        }
        /// type-alias for [```vec3```] with [```u64```]
        pub fn u64vec3(value: impl IntoVector<3, u64>) -> Vec3<u64> {
            vec3::<u64>(value)
        }
        /// type-alias for [```vec4```] with [```u64```]
        pub fn u64vec4(value: impl IntoVector<4, u64>) -> Vec4<u64> {
            vec4::<u64>(value)
        }
        /// type-alias for [```vec2p```] with [```u64```]
        pub fn u64vec2p(value: impl IntoVector<2, u64>) -> Vec2P<u64> {
            vec2p::<u64>(value)
        }
        /// type-alias for [```vec3p```] with [```u64```]
        pub fn u64vec3p(value: impl IntoVector<3, u64>) -> Vec3P<u64> {
            vec3p::<u64>(value)
        }
        /// type-alias for [```vec4p```] with [```u64```]
        pub fn u64vec4p(value: impl IntoVector<4, u64>) -> Vec4P<u64> {
            vec4p::<u64>(value)
        }
    }
    /// mathamatical type-aliases for [```u128```]
    pub mod u128 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```u128```] [```Vector```]
        pub type U128Vector<const N: usize, A> = Vector<N, u128, A>;
        /// type-alias for an [```u128```] [```Vec2```]
        pub type U128Vec2 = Vec2<u128>;
        /// type-alias for an [```u128```] [```Vec3```]
        pub type U128Vec3 = Vec3<u128>;
        /// type-alias for an [```u128```] [```Vec4```]
        pub type U128Vec4 = Vec4<u128>;
        /// type-alias for an [```u128```] [```Vec2P```]
        pub type U128Vec2P = Vec2P<u128>;
        /// type-alias for an [```u128```] [```Vec3P```]
        pub type U128Vec3P = Vec3P<u128>;
        /// type-alias for an [```u128```] [```Vec4P```]
        pub type U128Vec4P = Vec4P<u128>;
        /// type-alias for [```vector```] with [```u128```]
        pub fn u128vector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, u128>,
        ) -> Vector<N, u128, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, u128, A>(value)
        }
        /// type-alias for [```vec2```] with [```u128```]
        pub fn u128vec2(value: impl IntoVector<2, u128>) -> Vec2<u128> {
            vec2::<u128>(value)
        }
        /// type-alias for [```vec3```] with [```u128```]
        pub fn u128vec3(value: impl IntoVector<3, u128>) -> Vec3<u128> {
            vec3::<u128>(value)
        }
        /// type-alias for [```vec4```] with [```u128```]
        pub fn u128vec4(value: impl IntoVector<4, u128>) -> Vec4<u128> {
            vec4::<u128>(value)
        }
        /// type-alias for [```vec2p```] with [```u128```]
        pub fn u128vec2p(value: impl IntoVector<2, u128>) -> Vec2P<u128> {
            vec2p::<u128>(value)
        }
        /// type-alias for [```vec3p```] with [```u128```]
        pub fn u128vec3p(value: impl IntoVector<3, u128>) -> Vec3P<u128> {
            vec3p::<u128>(value)
        }
        /// type-alias for [```vec4p```] with [```u128```]
        pub fn u128vec4p(value: impl IntoVector<4, u128>) -> Vec4P<u128> {
            vec4p::<u128>(value)
        }
    }
    /// mathamatical type-aliases for [```usize```]
    pub mod usize {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```usize```] [```Vector```]
        pub type USizeVector<const N: usize, A> = Vector<N, usize, A>;
        /// type-alias for an [```usize```] [```Vec2```]
        pub type USizeVec2 = Vec2<usize>;
        /// type-alias for an [```usize```] [```Vec3```]
        pub type USizeVec3 = Vec3<usize>;
        /// type-alias for an [```usize```] [```Vec4```]
        pub type USizeVec4 = Vec4<usize>;
        /// type-alias for an [```usize```] [```Vec2P```]
        pub type USizeVec2P = Vec2P<usize>;
        /// type-alias for an [```usize```] [```Vec3P```]
        pub type USizeVec3P = Vec3P<usize>;
        /// type-alias for an [```usize```] [```Vec4P```]
        pub type USizeVec4P = Vec4P<usize>;
        /// type-alias for [```vector```] with [```usize```]
        pub fn usizevector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, usize>,
        ) -> Vector<N, usize, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, usize, A>(value)
        }
        /// type-alias for [```vec2```] with [```usize```]
        pub fn usizevec2(value: impl IntoVector<2, usize>) -> Vec2<usize> {
            vec2::<usize>(value)
        }
        /// type-alias for [```vec3```] with [```usize```]
        pub fn usizevec3(value: impl IntoVector<3, usize>) -> Vec3<usize> {
            vec3::<usize>(value)
        }
        /// type-alias for [```vec4```] with [```usize```]
        pub fn usizevec4(value: impl IntoVector<4, usize>) -> Vec4<usize> {
            vec4::<usize>(value)
        }
        /// type-alias for [```vec2p```] with [```usize```]
        pub fn usizevec2p(value: impl IntoVector<2, usize>) -> Vec2P<usize> {
            vec2p::<usize>(value)
        }
        /// type-alias for [```vec3p```] with [```usize```]
        pub fn usizevec3p(value: impl IntoVector<3, usize>) -> Vec3P<usize> {
            vec3p::<usize>(value)
        }
        /// type-alias for [```vec4p```] with [```usize```]
        pub fn usizevec4p(value: impl IntoVector<4, usize>) -> Vec4P<usize> {
            vec4p::<usize>(value)
        }
    }
    /// mathamatical type-aliases for [```i8```]
    pub mod i8 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```i8```] [```Vector```]
        pub type I8Vector<const N: usize, A> = Vector<N, i8, A>;
        /// type-alias for an [```i8```] [```Vec2```]
        pub type I8Vec2 = Vec2<i8>;
        /// type-alias for an [```i8```] [```Vec3```]
        pub type I8Vec3 = Vec3<i8>;
        /// type-alias for an [```i8```] [```Vec4```]
        pub type I8Vec4 = Vec4<i8>;
        /// type-alias for an [```i8```] [```Vec2P```]
        pub type I8Vec2P = Vec2P<i8>;
        /// type-alias for an [```i8```] [```Vec3P```]
        pub type I8Vec3P = Vec3P<i8>;
        /// type-alias for an [```i8```] [```Vec4P```]
        pub type I8Vec4P = Vec4P<i8>;
        /// type-alias for [```vector```] with [```i8```]
        pub fn i8vector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, i8>,
        ) -> Vector<N, i8, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, i8, A>(value)
        }
        /// type-alias for [```vec2```] with [```i8```]
        pub fn i8vec2(value: impl IntoVector<2, i8>) -> Vec2<i8> {
            vec2::<i8>(value)
        }
        /// type-alias for [```vec3```] with [```i8```]
        pub fn i8vec3(value: impl IntoVector<3, i8>) -> Vec3<i8> {
            vec3::<i8>(value)
        }
        /// type-alias for [```vec4```] with [```i8```]
        pub fn i8vec4(value: impl IntoVector<4, i8>) -> Vec4<i8> {
            vec4::<i8>(value)
        }
        /// type-alias for [```vec2p```] with [```i8```]
        pub fn i8vec2p(value: impl IntoVector<2, i8>) -> Vec2P<i8> {
            vec2p::<i8>(value)
        }
        /// type-alias for [```vec3p```] with [```i8```]
        pub fn i8vec3p(value: impl IntoVector<3, i8>) -> Vec3P<i8> {
            vec3p::<i8>(value)
        }
        /// type-alias for [```vec4p```] with [```i8```]
        pub fn i8vec4p(value: impl IntoVector<4, i8>) -> Vec4P<i8> {
            vec4p::<i8>(value)
        }
    }
    /// mathamatical type-aliases for [```i16```]
    pub mod i16 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```i16```] [```Vector```]
        pub type I16Vector<const N: usize, A> = Vector<N, i16, A>;
        /// type-alias for an [```i16```] [```Vec2```]
        pub type I16Vec2 = Vec2<i16>;
        /// type-alias for an [```i16```] [```Vec3```]
        pub type I16Vec3 = Vec3<i16>;
        /// type-alias for an [```i16```] [```Vec4```]
        pub type I16Vec4 = Vec4<i16>;
        /// type-alias for an [```i16```] [```Vec2P```]
        pub type I16Vec2P = Vec2P<i16>;
        /// type-alias for an [```i16```] [```Vec3P```]
        pub type I16Vec3P = Vec3P<i16>;
        /// type-alias for an [```i16```] [```Vec4P```]
        pub type I16Vec4P = Vec4P<i16>;
        /// type-alias for [```vector```] with [```i16```]
        pub fn i16vector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, i16>,
        ) -> Vector<N, i16, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, i16, A>(value)
        }
        /// type-alias for [```vec2```] with [```i16```]
        pub fn i16vec2(value: impl IntoVector<2, i16>) -> Vec2<i16> {
            vec2::<i16>(value)
        }
        /// type-alias for [```vec3```] with [```i16```]
        pub fn i16vec3(value: impl IntoVector<3, i16>) -> Vec3<i16> {
            vec3::<i16>(value)
        }
        /// type-alias for [```vec4```] with [```i16```]
        pub fn i16vec4(value: impl IntoVector<4, i16>) -> Vec4<i16> {
            vec4::<i16>(value)
        }
        /// type-alias for [```vec2p```] with [```i16```]
        pub fn i16vec2p(value: impl IntoVector<2, i16>) -> Vec2P<i16> {
            vec2p::<i16>(value)
        }
        /// type-alias for [```vec3p```] with [```i16```]
        pub fn i16vec3p(value: impl IntoVector<3, i16>) -> Vec3P<i16> {
            vec3p::<i16>(value)
        }
        /// type-alias for [```vec4p```] with [```i16```]
        pub fn i16vec4p(value: impl IntoVector<4, i16>) -> Vec4P<i16> {
            vec4p::<i16>(value)
        }
    }
    /// mathamatical type-aliases for [```i32```]
    pub mod i32 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```i32```] [```Vector```]
        pub type IVector<const N: usize, A> = Vector<N, i32, A>;
        /// type-alias for an [```i32```] [```Vec2```]
        pub type IVec2 = Vec2<i32>;
        /// type-alias for an [```i32```] [```Vec3```]
        pub type IVec3 = Vec3<i32>;
        /// type-alias for an [```i32```] [```Vec4```]
        pub type IVec4 = Vec4<i32>;
        /// type-alias for an [```i32```] [```Vec2P```]
        pub type IVec2P = Vec2P<i32>;
        /// type-alias for an [```i32```] [```Vec3P```]
        pub type IVec3P = Vec3P<i32>;
        /// type-alias for an [```i32```] [```Vec4P```]
        pub type IVec4P = Vec4P<i32>;
        /// type-alias for [```vector```] with [```i32```]
        pub fn ivector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, i32>,
        ) -> Vector<N, i32, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, i32, A>(value)
        }
        /// type-alias for [```vec2```] with [```i32```]
        pub fn ivec2(value: impl IntoVector<2, i32>) -> Vec2<i32> {
            vec2::<i32>(value)
        }
        /// type-alias for [```vec3```] with [```i32```]
        pub fn ivec3(value: impl IntoVector<3, i32>) -> Vec3<i32> {
            vec3::<i32>(value)
        }
        /// type-alias for [```vec4```] with [```i32```]
        pub fn ivec4(value: impl IntoVector<4, i32>) -> Vec4<i32> {
            vec4::<i32>(value)
        }
        /// type-alias for [```vec2p```] with [```i32```]
        pub fn ivec2p(value: impl IntoVector<2, i32>) -> Vec2P<i32> {
            vec2p::<i32>(value)
        }
        /// type-alias for [```vec3p```] with [```i32```]
        pub fn ivec3p(value: impl IntoVector<3, i32>) -> Vec3P<i32> {
            vec3p::<i32>(value)
        }
        /// type-alias for [```vec4p```] with [```i32```]
        pub fn ivec4p(value: impl IntoVector<4, i32>) -> Vec4P<i32> {
            vec4p::<i32>(value)
        }
    }
    /// mathamatical type-aliases for [```i64```]
    pub mod i64 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```i64```] [```Vector```]
        pub type I64Vector<const N: usize, A> = Vector<N, i64, A>;
        /// type-alias for an [```i64```] [```Vec2```]
        pub type I64Vec2 = Vec2<i64>;
        /// type-alias for an [```i64```] [```Vec3```]
        pub type I64Vec3 = Vec3<i64>;
        /// type-alias for an [```i64```] [```Vec4```]
        pub type I64Vec4 = Vec4<i64>;
        /// type-alias for an [```i64```] [```Vec2P```]
        pub type I64Vec2P = Vec2P<i64>;
        /// type-alias for an [```i64```] [```Vec3P```]
        pub type I64Vec3P = Vec3P<i64>;
        /// type-alias for an [```i64```] [```Vec4P```]
        pub type I64Vec4P = Vec4P<i64>;
        /// type-alias for [```vector```] with [```i64```]
        pub fn i64vector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, i64>,
        ) -> Vector<N, i64, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, i64, A>(value)
        }
        /// type-alias for [```vec2```] with [```i64```]
        pub fn i64vec2(value: impl IntoVector<2, i64>) -> Vec2<i64> {
            vec2::<i64>(value)
        }
        /// type-alias for [```vec3```] with [```i64```]
        pub fn i64vec3(value: impl IntoVector<3, i64>) -> Vec3<i64> {
            vec3::<i64>(value)
        }
        /// type-alias for [```vec4```] with [```i64```]
        pub fn i64vec4(value: impl IntoVector<4, i64>) -> Vec4<i64> {
            vec4::<i64>(value)
        }
        /// type-alias for [```vec2p```] with [```i64```]
        pub fn i64vec2p(value: impl IntoVector<2, i64>) -> Vec2P<i64> {
            vec2p::<i64>(value)
        }
        /// type-alias for [```vec3p```] with [```i64```]
        pub fn i64vec3p(value: impl IntoVector<3, i64>) -> Vec3P<i64> {
            vec3p::<i64>(value)
        }
        /// type-alias for [```vec4p```] with [```i64```]
        pub fn i64vec4p(value: impl IntoVector<4, i64>) -> Vec4P<i64> {
            vec4p::<i64>(value)
        }
    }
    /// mathamatical type-aliases for [```i128```]
    pub mod i128 {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```i128```] [```Vector```]
        pub type I128Vector<const N: usize, A> = Vector<N, i128, A>;
        /// type-alias for an [```i128```] [```Vec2```]
        pub type I128Vec2 = Vec2<i128>;
        /// type-alias for an [```i128```] [```Vec3```]
        pub type I128Vec3 = Vec3<i128>;
        /// type-alias for an [```i128```] [```Vec4```]
        pub type I128Vec4 = Vec4<i128>;
        /// type-alias for an [```i128```] [```Vec2P```]
        pub type I128Vec2P = Vec2P<i128>;
        /// type-alias for an [```i128```] [```Vec3P```]
        pub type I128Vec3P = Vec3P<i128>;
        /// type-alias for an [```i128```] [```Vec4P```]
        pub type I128Vec4P = Vec4P<i128>;
        /// type-alias for [```vector```] with [```i128```]
        pub fn i128vector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, i128>,
        ) -> Vector<N, i128, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, i128, A>(value)
        }
        /// type-alias for [```vec2```] with [```i128```]
        pub fn i128vec2(value: impl IntoVector<2, i128>) -> Vec2<i128> {
            vec2::<i128>(value)
        }
        /// type-alias for [```vec3```] with [```i128```]
        pub fn i128vec3(value: impl IntoVector<3, i128>) -> Vec3<i128> {
            vec3::<i128>(value)
        }
        /// type-alias for [```vec4```] with [```i128```]
        pub fn i128vec4(value: impl IntoVector<4, i128>) -> Vec4<i128> {
            vec4::<i128>(value)
        }
        /// type-alias for [```vec2p```] with [```i128```]
        pub fn i128vec2p(value: impl IntoVector<2, i128>) -> Vec2P<i128> {
            vec2p::<i128>(value)
        }
        /// type-alias for [```vec3p```] with [```i128```]
        pub fn i128vec3p(value: impl IntoVector<3, i128>) -> Vec3P<i128> {
            vec3p::<i128>(value)
        }
        /// type-alias for [```vec4p```] with [```i128```]
        pub fn i128vec4p(value: impl IntoVector<4, i128>) -> Vec4P<i128> {
            vec4p::<i128>(value)
        }
    }
    /// mathamatical type-aliases for [```isize```]
    pub mod isize {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```isize```] [```Vector```]
        pub type ISizeVector<const N: usize, A> = Vector<N, isize, A>;
        /// type-alias for an [```isize```] [```Vec2```]
        pub type ISizeVec2 = Vec2<isize>;
        /// type-alias for an [```isize```] [```Vec3```]
        pub type ISizeVec3 = Vec3<isize>;
        /// type-alias for an [```isize```] [```Vec4```]
        pub type ISizeVec4 = Vec4<isize>;
        /// type-alias for an [```isize```] [```Vec2P```]
        pub type ISizeVec2P = Vec2P<isize>;
        /// type-alias for an [```isize```] [```Vec3P```]
        pub type ISizeVec3P = Vec3P<isize>;
        /// type-alias for an [```isize```] [```Vec4P```]
        pub type ISizeVec4P = Vec4P<isize>;
        /// type-alias for [```vector```] with [```isize```]
        pub fn isizevector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, isize>,
        ) -> Vector<N, isize, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, isize, A>(value)
        }
        /// type-alias for [```vec2```] with [```isize```]
        pub fn isizevec2(value: impl IntoVector<2, isize>) -> Vec2<isize> {
            vec2::<isize>(value)
        }
        /// type-alias for [```vec3```] with [```isize```]
        pub fn isizevec3(value: impl IntoVector<3, isize>) -> Vec3<isize> {
            vec3::<isize>(value)
        }
        /// type-alias for [```vec4```] with [```isize```]
        pub fn isizevec4(value: impl IntoVector<4, isize>) -> Vec4<isize> {
            vec4::<isize>(value)
        }
        /// type-alias for [```vec2p```] with [```isize```]
        pub fn isizevec2p(value: impl IntoVector<2, isize>) -> Vec2P<isize> {
            vec2p::<isize>(value)
        }
        /// type-alias for [```vec3p```] with [```isize```]
        pub fn isizevec3p(value: impl IntoVector<3, isize>) -> Vec3P<isize> {
            vec3p::<isize>(value)
        }
        /// type-alias for [```vec4p```] with [```isize```]
        pub fn isizevec4p(value: impl IntoVector<4, isize>) -> Vec4P<isize> {
            vec4p::<isize>(value)
        }
    }
    /// mathamatical type-aliases for [```bool```]
    pub mod bool {
        use super::*;
        use ggmath::vector::{alignment::*, into_vec::*, length::*, *};
        /// type-alias for an [```bool```] [```Vector```]
        pub type BVector<const N: usize, A> = Vector<N, bool, A>;
        /// type-alias for an [```bool```] [```Vec2```]
        pub type BVec2 = Vec2<bool>;
        /// type-alias for an [```bool```] [```Vec3```]
        pub type BVec3 = Vec3<bool>;
        /// type-alias for an [```bool```] [```Vec4```]
        pub type BVec4 = Vec4<bool>;
        /// type-alias for an [```bool```] [```Vec2P```]
        pub type BVec2P = Vec2P<bool>;
        /// type-alias for an [```bool```] [```Vec3P```]
        pub type BVec3P = Vec3P<bool>;
        /// type-alias for an [```bool```] [```Vec4P```]
        pub type BVec4P = Vec4P<bool>;
        /// type-alias for [```vector```] with [```bool```]
        pub fn bvector<const N: usize, A: VecAlignment>(
            value: impl IntoVector<N, bool>,
        ) -> Vector<N, bool, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            vector::<N, bool, A>(value)
        }
        /// type-alias for [```vec2```] with [```bool```]
        pub fn bvec2(value: impl IntoVector<2, bool>) -> Vec2<bool> {
            vec2::<bool>(value)
        }
        /// type-alias for [```vec3```] with [```bool```]
        pub fn bvec3(value: impl IntoVector<3, bool>) -> Vec3<bool> {
            vec3::<bool>(value)
        }
        /// type-alias for [```vec4```] with [```bool```]
        pub fn bvec4(value: impl IntoVector<4, bool>) -> Vec4<bool> {
            vec4::<bool>(value)
        }
        /// type-alias for [```vec2p```] with [```bool```]
        pub fn bvec2p(value: impl IntoVector<2, bool>) -> Vec2P<bool> {
            vec2p::<bool>(value)
        }
        /// type-alias for [```vec3p```] with [```bool```]
        pub fn bvec3p(value: impl IntoVector<3, bool>) -> Vec3P<bool> {
            vec3p::<bool>(value)
        }
        /// type-alias for [```vec4p```] with [```bool```]
        pub fn bvec4p(value: impl IntoVector<4, bool>) -> Vec4P<bool> {
            vec4p::<bool>(value)
        }
    }
}
use crate as ggmath;
