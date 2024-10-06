#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod element {
    use std::fmt::{self, Display};
    use crate::vec::*;
    mod impl_ {
        use super::*;
        pub mod default_impl {
            use super::*;
            mod num {
                use num_traits::*;
                use super::*;
                use crate::element::num::*;
                mod convert {
                    use super::*;
                    impl<
                        T: ElementDefaultImpl + Num,
                        N: NumElement + AsPrimitive<T>,
                    > ElementFromNum<N> for T {
                        fn from_num(value: N) -> Self {
                            N::as_(value)
                        }
                        fn vec2_from_num(value: Vec2<N>) -> Vec2<Self> {
                            Vec2::from_array([
                                Self::from_num(value.x()),
                                Self::from_num(value.y()),
                            ])
                        }
                        fn vec3_from_num(value: Vec3<N>) -> Vec3<Self> {
                            Vec3::from_array([
                                Self::from_num(value.x()),
                                Self::from_num(value.y()),
                                Self::from_num(value.z()),
                            ])
                        }
                        fn vec4_from_num(value: Vec4<N>) -> Vec4<Self> {
                            Vec4::from_array([
                                Self::from_num(value.x()),
                                Self::from_num(value.y()),
                                Self::from_num(value.z()),
                                Self::from_num(value.w()),
                            ])
                        }
                    }
                }
                mod signed {
                    use super::*;
                    impl<T: ElementDefaultImpl + NumElement + Signed> SignedElement
                    for T {
                        #[inline(always)]
                        fn neg_one() -> Self {
                            -Self::one()
                        }
                        fn abs(self) -> Self {
                            <Self as Signed>::abs(&self)
                        }
                        fn signum(self) -> Self {
                            <Self as Signed>::signum(&self)
                        }
                        fn is_positive(self) -> bool {
                            <Self as Signed>::is_positive(&self)
                        }
                        fn is_negative(self) -> bool {
                            <Self as Signed>::is_negative(&self)
                        }
                    }
                    impl<
                        const N: usize,
                        T: ElementDefaultImpl + NumElement + Signed,
                    > SignedElementVecFns<N> for T
                    where
                        MaybeVecNum<N>: VecNum,
                    {
                        fn neg_one() -> VecN<N, Self> {
                            -Self::one()
                        }
                        fn abs(value: VecN<N, Self>) -> VecN<N, Self> {
                            value.map(|c| c.abs())
                        }
                        fn signum(value: VecN<N, Self>) -> VecN<N, Self> {
                            value.map(|c| c.signum())
                        }
                        fn are_positive(value: VecN<N, Self>) -> VecN<N, bool> {
                            value.map(|c| c.is_positive())
                        }
                        fn are_negative(value: VecN<N, Self>) -> VecN<N, bool> {
                            value.map(|c| c.is_negative())
                        }
                    }
                }
            }
            mod ops {
                use std::ops::*;
                use gomath_proc_macros::{assign_ops, rhs_ops, self_ops};
                use crate::element::ops::*;
                use super::*;
            }
            mod vec {
                use super::*;
                mod base {
                    use super::*;
                    mod const_swizzle {
                        use super::*;
                        mod cget {
                            use super::*;
                            impl<T: ElementDefaultImpl> ElementVecConstGet for T {
                                #[inline(always)]
                                unsafe fn vec2_cget<const X: usize>(
                                    value: Self::InnerVec2,
                                ) -> Self {
                                    *value.get_unchecked(X)
                                }
                                #[inline(always)]
                                unsafe fn vec2_cget2<const X: usize, const Y: usize>(
                                    value: Self::InnerVec2,
                                ) -> Self::InnerVec2 {
                                    [*value.get_unchecked(X), *value.get_unchecked(Y)]
                                }
                                #[inline(always)]
                                unsafe fn vec2_cget3<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                >(value: Self::InnerVec2) -> Self::InnerVec3 {
                                    [
                                        *value.get_unchecked(X),
                                        *value.get_unchecked(Y),
                                        *value.get_unchecked(Z),
                                        T::default(),
                                    ]
                                }
                                #[inline(always)]
                                unsafe fn vec2_cget4<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                    const W: usize,
                                >(value: Self::InnerVec2) -> Self::InnerVec4 {
                                    [
                                        *value.get_unchecked(X),
                                        *value.get_unchecked(Y),
                                        *value.get_unchecked(Z),
                                        *value.get_unchecked(W),
                                    ]
                                }
                                #[inline(always)]
                                unsafe fn vec3_cget<const X: usize>(
                                    value: Self::InnerVec3,
                                ) -> Self {
                                    *value.get_unchecked(X)
                                }
                                #[inline(always)]
                                unsafe fn vec3_cget2<const X: usize, const Y: usize>(
                                    value: Self::InnerVec3,
                                ) -> Self::InnerVec2 {
                                    [*value.get_unchecked(X), *value.get_unchecked(Y)]
                                }
                                #[inline(always)]
                                unsafe fn vec3_cget3<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                >(value: Self::InnerVec3) -> Self::InnerVec3 {
                                    [
                                        *value.get_unchecked(X),
                                        *value.get_unchecked(Y),
                                        *value.get_unchecked(Z),
                                        T::default(),
                                    ]
                                }
                                #[inline(always)]
                                unsafe fn vec3_cget4<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                    const W: usize,
                                >(value: Self::InnerVec3) -> Self::InnerVec4 {
                                    [
                                        *value.get_unchecked(X),
                                        *value.get_unchecked(Y),
                                        *value.get_unchecked(Z),
                                        *value.get_unchecked(W),
                                    ]
                                }
                                #[inline(always)]
                                unsafe fn vec4_cget<const X: usize>(
                                    value: Self::InnerVec4,
                                ) -> Self {
                                    *value.get_unchecked(X)
                                }
                                #[inline(always)]
                                unsafe fn vec4_cget2<const X: usize, const Y: usize>(
                                    value: Self::InnerVec4,
                                ) -> Self::InnerVec2 {
                                    [*value.get_unchecked(X), *value.get_unchecked(Y)]
                                }
                                #[inline(always)]
                                unsafe fn vec4_cget3<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                >(value: Self::InnerVec4) -> Self::InnerVec3 {
                                    [
                                        *value.get_unchecked(X),
                                        *value.get_unchecked(Y),
                                        *value.get_unchecked(Z),
                                        T::default(),
                                    ]
                                }
                                #[inline(always)]
                                unsafe fn vec4_cget4<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                    const W: usize,
                                >(value: Self::InnerVec4) -> Self::InnerVec4 {
                                    [
                                        *value.get_unchecked(X),
                                        *value.get_unchecked(Y),
                                        *value.get_unchecked(Z),
                                        *value.get_unchecked(W),
                                    ]
                                }
                            }
                        }
                        mod cwith {
                            use super::*;
                            impl<T: ElementDefaultImpl> ElementVecConstWith for T {
                                #[inline(always)]
                                unsafe fn vec2_cwith<const X: usize>(
                                    mut vec: Self::InnerVec2,
                                    value: Self,
                                ) -> Self::InnerVec2 {
                                    *vec.get_unchecked_mut(X) = value;
                                    vec
                                }
                                #[inline(always)]
                                unsafe fn vec2_cwith2<const X: usize, const Y: usize>(
                                    mut vec: Self::InnerVec2,
                                    value: Self::InnerVec2,
                                ) -> Self::InnerVec2 {
                                    *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
                                    *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
                                    vec
                                }
                                #[inline(always)]
                                unsafe fn vec3_cwith<const X: usize>(
                                    mut vec: Self::InnerVec3,
                                    value: Self,
                                ) -> Self::InnerVec3 {
                                    *vec.get_unchecked_mut(X) = value;
                                    vec
                                }
                                #[inline(always)]
                                unsafe fn vec3_cwith2<const X: usize, const Y: usize>(
                                    mut vec: Self::InnerVec3,
                                    value: Self::InnerVec2,
                                ) -> Self::InnerVec3 {
                                    *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
                                    *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
                                    vec
                                }
                                #[inline(always)]
                                unsafe fn vec3_cwith3<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                >(
                                    mut vec: Self::InnerVec3,
                                    value: Self::InnerVec3,
                                ) -> Self::InnerVec3 {
                                    *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
                                    *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
                                    *vec.get_unchecked_mut(Z) = *value.get_unchecked(2);
                                    vec
                                }
                                #[inline(always)]
                                unsafe fn vec4_cwith<const X: usize>(
                                    mut vec: Self::InnerVec4,
                                    value: Self,
                                ) -> Self::InnerVec4 {
                                    *vec.get_unchecked_mut(X) = value;
                                    vec
                                }
                                #[inline(always)]
                                unsafe fn vec4_cwith2<const X: usize, const Y: usize>(
                                    mut vec: Self::InnerVec4,
                                    value: Self::InnerVec2,
                                ) -> Self::InnerVec4 {
                                    *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
                                    *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
                                    vec
                                }
                                #[inline(always)]
                                unsafe fn vec4_cwith3<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                >(
                                    mut vec: Self::InnerVec4,
                                    value: Self::InnerVec3,
                                ) -> Self::InnerVec4 {
                                    *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
                                    *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
                                    *vec.get_unchecked_mut(Z) = *value.get_unchecked(2);
                                    vec
                                }
                                #[inline(always)]
                                unsafe fn vec4_cwith4<
                                    const X: usize,
                                    const Y: usize,
                                    const Z: usize,
                                    const W: usize,
                                >(
                                    mut vec: Self::InnerVec4,
                                    value: Self::InnerVec4,
                                ) -> Self::InnerVec4 {
                                    *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
                                    *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
                                    *vec.get_unchecked_mut(Z) = *value.get_unchecked(2);
                                    *vec.get_unchecked_mut(W) = *value.get_unchecked(3);
                                    vec
                                }
                            }
                        }
                        impl<T: ElementDefaultImpl> ElementVecConstSwizzle for T {}
                    }
                    mod default {
                        use std_impl::ElementVecDefault;
                        use super::*;
                        impl<T: ElementDefaultImpl> ElementVecDefault for T {
                            #[inline(always)]
                            fn default_vec2() -> Self::InnerVec2 {
                                [T::default(); 2]
                            }
                            #[inline(always)]
                            fn default_vec3() -> Self::InnerVec3 {
                                [T::default(); 4]
                            }
                            #[inline(always)]
                            fn default_vec4() -> Self::InnerVec4 {
                                [T::default(); 4]
                            }
                        }
                    }
                    mod inner {
                        use vec::inner::ElementVecInner;
                        use super::*;
                        unsafe impl<T: ElementDefaultImpl> ElementVecInner for T {
                            type InnerVec2 = [Self; 2];
                            type InnerVec3 = [Self; 4];
                            type InnerVec4 = [Self; 4];
                        }
                    }
                    mod splat {
                        use vec::splat::ElementVecSplat;
                        use super::*;
                        impl<T: ElementDefaultImpl> ElementVecSplat for T {
                            #[inline(always)]
                            fn vec2_splat(value: Self) -> Self::InnerVec2 {
                                [value; 2]
                            }
                            #[inline(always)]
                            fn vec3_splat(value: Self) -> Self::InnerVec3 {
                                [value; 4]
                            }
                            #[inline(always)]
                            fn vec4_splat(value: Self) -> Self::InnerVec4 {
                                [value; 4]
                            }
                        }
                    }
                    mod swizzle {
                        use vec::swizzle::ElementVecSwizzle;
                        use super::*;
                        mod get {
                            use vec::swizzle::ElementVecGet;
                            use super::*;
                            impl<const N: usize, T: ElementDefaultImpl> ElementVecGet<N>
                            for T
                            where
                                MaybeVecNum<N>: VecNum<N>,
                            {
                                fn vec_get(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    index: usize,
                                ) -> Result<Self, &'static str> {
                                    Ok(
                                        match vec.get(index) {
                                            Some(some) => *some,
                                            None => return Err("x is out of vec2 bounds"),
                                        },
                                    )
                                }
                                fn vec_get2(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 2],
                                ) -> Result<Self::InnerVec2, &'static str> {
                                    Ok([
                                        match vec.get(indicies[0]) {
                                            Some(some) => *some,
                                            None => return Err("x is out of vec2 bounds"),
                                        },
                                        match vec.get(indicies[1]) {
                                            Some(some) => *some,
                                            None => return Err("y is out of vec2 bounds"),
                                        },
                                    ])
                                }
                                fn vec_get3(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 3],
                                ) -> Result<Self::InnerVec3, &'static str> {
                                    Ok([
                                        match vec.get(indicies[0]) {
                                            Some(some) => *some,
                                            None => return Err("x is out of vec2 bounds"),
                                        },
                                        match vec.get(indicies[1]) {
                                            Some(some) => *some,
                                            None => return Err("y is out of vec2 bounds"),
                                        },
                                        match vec.get(indicies[2]) {
                                            Some(some) => *some,
                                            None => return Err("z is out of vec2 bounds"),
                                        },
                                        T::default(),
                                    ])
                                }
                                fn vec_get4(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 4],
                                ) -> Result<Self::InnerVec4, &'static str> {
                                    Ok([
                                        match vec.get(indicies[0]) {
                                            Some(some) => *some,
                                            None => return Err("x is out of vec2 bounds"),
                                        },
                                        match vec.get(indicies[1]) {
                                            Some(some) => *some,
                                            None => return Err("y is out of vec2 bounds"),
                                        },
                                        match vec.get(indicies[2]) {
                                            Some(some) => *some,
                                            None => return Err("z is out of vec2 bounds"),
                                        },
                                        match vec.get(indicies[3]) {
                                            Some(some) => *some,
                                            None => return Err("w is out of vec2 bounds"),
                                        },
                                    ])
                                }
                                unsafe fn vec_get_unchecked(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    index: usize,
                                ) -> Self {
                                    *vec.get_unchecked(index)
                                }
                                unsafe fn vec_get2_unchecked(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 2],
                                ) -> Self::InnerVec2 {
                                    indicies.map(|i| vec.get_unchecked(indicies[i]))
                                }
                                unsafe fn vec_get3_unchecked(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 3],
                                ) -> Self::InnerVec3 {
                                    [
                                        *vec.get_unchecked(indicies[0]),
                                        *vec.get_unchecked(indicies[1]),
                                        *vec.get_unchecked(indicies[2]),
                                        T::default(),
                                    ]
                                }
                                unsafe fn vec_get4_unchecked(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 4],
                                ) -> Self::InnerVec4 {
                                    indicies.map(|i| vec.get_unchecked(indicies[i]))
                                }
                            }
                        }
                        mod with {
                            use vec::swizzle::ElementVecWith;
                            use super::*;
                            impl<const N: usize, T: ElementDefaultImpl> ElementVecWith<N>
                            for T
                            where
                                MaybeVecNum<N>: VecNum<N>,
                            {
                                fn vec_with(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    index: usize,
                                    value: Self,
                                ) -> Result<vec::inner::InnerVec<N, Self>, &'static str> {
                                    match vec.get_mut(x) {
                                        Some(some) => *some = value,
                                        None => return Err("x is out of vec2 bounds"),
                                    };
                                    Ok(vec)
                                }
                                fn vec_with2(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 2],
                                    value: Self::InnerVec2,
                                ) -> Result<vec::inner::InnerVec<N, Self>, &'static str> {
                                    match vec.get_mut(x) {
                                        Some(some) => *some = unsafe { T::vec2_cget::<0>(value) },
                                        None => return Err("x is out of vec2 bounds"),
                                    };
                                    match vec.get_mut(y) {
                                        Some(some) => *some = unsafe { T::vec2_cget::<1>(value) },
                                        None => return Err("y is out of vec2 bounds"),
                                    };
                                    Ok(vec)
                                }
                                fn vec_with3(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 3],
                                    value: Self::InnerVec3,
                                ) -> Result<vec::inner::InnerVec<N, Self>, &'static str> {
                                    if x < 3 {
                                        *unsafe { vec.get_unchecked_mut(x) } = value;
                                    } else {
                                        return Err("x is out of vec3 bounds");
                                    };
                                    Ok(vec)
                                }
                                fn vec_with4(
                                    vec: vec::inner::InnerVec<N, Self>,
                                    indicies: [usize; 4],
                                    value: Self::InnerVec4,
                                ) -> Result<vec::inner::InnerVec<N, Self>, &'static str> {
                                    if x < 3 {
                                        *unsafe { vec.get_unchecked_mut(x) } = unsafe {
                                            T::vec2_cget::<0>(value)
                                        };
                                    } else {
                                        return Err("x is out of vec3 bounds");
                                    };
                                    if y < 3 {
                                        *unsafe { vec.get_unchecked_mut(y) } = unsafe {
                                            T::vec2_cget::<1>(value)
                                        };
                                    } else {
                                        return Err("y is out of vec3 bounds");
                                    };
                                    Ok(vec)
                                }
                                unsafe fn vec_with_unchecked(
                                    vec: Self::InnerVec4,
                                    index: usize,
                                    value: Self,
                                ) -> Self::InnerVec4 {}
                                unsafe fn vec_with2_unchecked(
                                    vec: Self::InnerVec4,
                                    indicies: [usize; 2],
                                    value: Self::InnerVec2,
                                ) -> Self::InnerVec4 {}
                                unsafe fn vec_with3_unchecked(
                                    vec: Self::InnerVec4,
                                    indicies: [usize; 3],
                                    value: Self::InnerVec3,
                                ) -> Self::InnerVec4 {}
                                unsafe fn vec_with4_unchecked(
                                    vec: Self::InnerVec4,
                                    indicies: [usize; 4],
                                    value: Self::InnerVec4,
                                ) -> Self::InnerVec4 {}
                            }
                        }
                        impl<T: ElementDefaultImpl> ElementVecSwizzle for T {}
                    }
                    impl<T: ElementDefaultImpl> ElementVec for T {}
                }
            }
            pub trait ElementDefaultImpl: 'static + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display {}
            impl<T: ElementDefaultImpl> Element for T {}
        }
        mod primitive_impl {
            mod bool {
                type Ty = bool;
                use crate::element::default_impl::*;
                impl ElementDefaultImpl for Ty {}
            }
            mod floats {
                mod f32 {
                    type Ty = f32;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod f64 {
                    type Ty = f64;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
            }
            mod signed {
                mod i128 {
                    type Ty = i128;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod i16 {
                    type Ty = i16;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod i32 {
                    type Ty = i32;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod i64 {
                    type Ty = i64;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod i8 {
                    type Ty = i8;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod isize {
                    type Ty = isize;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
            }
            mod unsigned {
                mod u128 {
                    type Ty = u128;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod u16 {
                    type Ty = u16;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod u32 {
                    type Ty = u32;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod u64 {
                    type Ty = u64;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod u8 {
                    type Ty = u8;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
                mod usize {
                    type Ty = usize;
                    use crate::element::default_impl::*;
                    impl ElementDefaultImpl for Ty {}
                }
            }
        }
    }
    pub use impl_::*;
    pub mod num {
        use num_traits::*;
        use super::{ops::*, *};
        mod convert {
            use super::*;
            pub trait ElementNumConvert: Element + NumCast + ElementFromNum<
                    f32,
                > + ElementFromNum<
                    f64,
                > + ElementFromNum<
                    u8,
                > + ElementFromNum<
                    u16,
                > + ElementFromNum<
                    u32,
                > + ElementFromNum<
                    u64,
                > + ElementFromNum<
                    u128,
                > + ElementFromNum<
                    usize,
                > + ElementFromNum<
                    i8,
                > + ElementFromNum<
                    i16,
                > + ElementFromNum<
                    i32,
                > + ElementFromNum<i64> + ElementFromNum<i128> + ElementFromNum<isize> {
                fn as_num<N: NumElement>(self) -> N;
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N>;
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N>;
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N>;
            }
            pub trait ElementFromNum<N: NumElement>: Element {
                fn from_num(value: N) -> Self;
                fn vec2_from_num(value: Vec2<N>) -> Vec2<Self>;
                fn vec3_from_num(value: Vec3<N>) -> Vec3<Self>;
                fn vec4_from_num(value: Vec4<N>) -> Vec4<Self>;
            }
            impl ElementNumConvert for f32 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for f64 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for u8 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for u16 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for u32 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for u64 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for u128 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for usize {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for i8 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for i16 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for i32 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for i64 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for i128 {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
            impl ElementNumConvert for isize {
                fn as_num<N: NumElement>(self) -> N {
                    N::from_num(self)
                }
                fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                    N::vec2_from_num(value)
                }
                fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                    N::vec3_from_num(value)
                }
                fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                    N::vec4_from_num(value)
                }
            }
        }
        mod float {
            use super::*;
            pub use num_traits::Float;
            pub trait FloatElement: SignedElement + Float {}
            impl FloatElement for f32 {}
            impl FloatElement for f64 {}
        }
        mod int {
            use super::*;
            pub use num_traits::PrimInt;
            pub trait IntElement: NumElement + PrimInt + ElementBitAnd + ElementBitAndAssign + ElementBitOr + ElementBitOrAssign + ElementBitXor + ElementBitXorAssign + ElementShr + ElementShrAssign + ElementShl + ElementShlAssign {}
            impl IntElement for u8 {}
            impl IntElement for u16 {}
            impl IntElement for u32 {}
            impl IntElement for u64 {}
            impl IntElement for u128 {}
            impl IntElement for usize {}
            impl IntElement for i8 {}
            impl IntElement for i16 {}
            impl IntElement for i32 {}
            impl IntElement for i64 {}
            impl IntElement for i128 {}
            impl IntElement for isize {}
        }
        mod signed {
            use super::*;
            pub trait SignedElement: NumElement + Signed + ElementNeg<
                    Output = Self,
                > + SignedElementVecFns<
                    2,
                > + SignedElementVecFns<3> + SignedElementVecFns<4> {
                fn neg_one() -> Self;
                fn abs(self) -> Self;
                fn signum(self) -> Self;
                fn is_positive(self) -> bool;
                fn is_negative(self) -> bool;
            }
            pub trait SignedElementVecFns<const N: usize>: NumElement
            where
                MaybeVecNum<N>: VecNum,
            {
                fn neg_one() -> VecN<N, Self>;
                fn abs(value: VecN<N, Self>) -> VecN<N, Self>;
                fn signum(value: VecN<N, Self>) -> VecN<N, Self>;
                fn are_positive(value: VecN<N, Self>) -> VecN<N, bool>;
                fn are_negative(value: VecN<N, Self>) -> VecN<N, bool>;
            }
            const _: () = {
                const fn validate<T: SignedElement>() {}
                validate::<f32>()
            };
            const _: () = {
                const fn validate<T: SignedElement>() {}
                validate::<f64>()
            };
            const _: () = {
                const fn validate<T: SignedElement>() {}
                validate::<i8>()
            };
            const _: () = {
                const fn validate<T: SignedElement>() {}
                validate::<i16>()
            };
            const _: () = {
                const fn validate<T: SignedElement>() {}
                validate::<i32>()
            };
            const _: () = {
                const fn validate<T: SignedElement>() {}
                validate::<i64>()
            };
            const _: () = {
                const fn validate<T: SignedElement>() {}
                validate::<i128>()
            };
            const _: () = {
                const fn validate<T: SignedElement>() {}
                validate::<isize>()
            };
        }
        mod unsigned {
            use super::*;
            pub use num_traits::Unsigned;
            pub trait UnsignedElement: NumElement + Unsigned {}
            impl UnsignedElement for u8 {}
            impl UnsignedElement for u16 {}
            impl UnsignedElement for u32 {}
            impl UnsignedElement for u64 {}
            impl UnsignedElement for u128 {}
            impl UnsignedElement for usize {}
        }
        pub use convert::*;
        pub use float::*;
        pub use int::*;
        pub use signed::*;
        pub use unsigned::*;
        pub trait NumElement: Element + ElementNumConvert + Num + ElementAdd + ElementAddAssign + ElementSub + ElementSubAssign + ElementMul + ElementMulAssign + ElementDiv + ElementDivAssign + ElementRem + ElementRemAssign {
            const ZERO: Self;
            const ONE: Self;
        }
        impl NumElement for f32 {
            const ZERO: Self = 0 as f32;
            const ONE: Self = 1 as f32;
        }
        impl NumElement for f64 {
            const ZERO: Self = 0 as f64;
            const ONE: Self = 1 as f64;
        }
        impl NumElement for u8 {
            const ZERO: Self = 0 as u8;
            const ONE: Self = 1 as u8;
        }
        impl NumElement for u16 {
            const ZERO: Self = 0 as u16;
            const ONE: Self = 1 as u16;
        }
        impl NumElement for u32 {
            const ZERO: Self = 0 as u32;
            const ONE: Self = 1 as u32;
        }
        impl NumElement for u64 {
            const ZERO: Self = 0 as u64;
            const ONE: Self = 1 as u64;
        }
        impl NumElement for u128 {
            const ZERO: Self = 0 as u128;
            const ONE: Self = 1 as u128;
        }
        impl NumElement for usize {
            const ZERO: Self = 0 as usize;
            const ONE: Self = 1 as usize;
        }
        impl NumElement for i8 {
            const ZERO: Self = 0 as i8;
            const ONE: Self = 1 as i8;
        }
        impl NumElement for i16 {
            const ZERO: Self = 0 as i16;
            const ONE: Self = 1 as i16;
        }
        impl NumElement for i32 {
            const ZERO: Self = 0 as i32;
            const ONE: Self = 1 as i32;
        }
        impl NumElement for i64 {
            const ZERO: Self = 0 as i64;
            const ONE: Self = 1 as i64;
        }
        impl NumElement for i128 {
            const ZERO: Self = 0 as i128;
            const ONE: Self = 1 as i128;
        }
        impl NumElement for isize {
            const ZERO: Self = 0 as isize;
            const ONE: Self = 1 as isize;
        }
    }
    pub mod ops {
        use std::ops::*;
        use super::*;
        use crate::vec::ops::*;
        mod assign_ops {
            use gomath_proc_macros::{assign_ops, ops_macro};
            use super::*;
            pub trait ElementAddAssign<
                Rhs: Element = Self,
            >: Element + AddAssign<
                    Rhs,
                > + ElementVecAddAssign<
                    2,
                    Rhs,
                > + ElementVecAddAssign<3, Rhs> + ElementVecAddAssign<4, Rhs> {}
            pub trait ElementSubAssign<
                Rhs: Element = Self,
            >: Element + SubAssign<
                    Rhs,
                > + ElementVecSubAssign<
                    2,
                    Rhs,
                > + ElementVecSubAssign<3, Rhs> + ElementVecSubAssign<4, Rhs> {}
            pub trait ElementMulAssign<
                Rhs: Element = Self,
            >: Element + MulAssign<
                    Rhs,
                > + ElementVecMulAssign<
                    2,
                    Rhs,
                > + ElementVecMulAssign<3, Rhs> + ElementVecMulAssign<4, Rhs> {}
            pub trait ElementDivAssign<
                Rhs: Element = Self,
            >: Element + DivAssign<
                    Rhs,
                > + ElementVecDivAssign<
                    2,
                    Rhs,
                > + ElementVecDivAssign<3, Rhs> + ElementVecDivAssign<4, Rhs> {}
            pub trait ElementRemAssign<
                Rhs: Element = Self,
            >: Element + RemAssign<
                    Rhs,
                > + ElementVecRemAssign<
                    2,
                    Rhs,
                > + ElementVecRemAssign<3, Rhs> + ElementVecRemAssign<4, Rhs> {}
            pub trait ElementBitAndAssign<
                Rhs: Element = Self,
            >: Element + BitAndAssign<
                    Rhs,
                > + ElementVecBitAndAssign<
                    2,
                    Rhs,
                > + ElementVecBitAndAssign<3, Rhs> + ElementVecBitAndAssign<4, Rhs> {}
            pub trait ElementBitOrAssign<
                Rhs: Element = Self,
            >: Element + BitOrAssign<
                    Rhs,
                > + ElementVecBitOrAssign<
                    2,
                    Rhs,
                > + ElementVecBitOrAssign<3, Rhs> + ElementVecBitOrAssign<4, Rhs> {}
            pub trait ElementBitXorAssign<
                Rhs: Element = Self,
            >: Element + BitXorAssign<
                    Rhs,
                > + ElementVecBitXorAssign<
                    2,
                    Rhs,
                > + ElementVecBitXorAssign<3, Rhs> + ElementVecBitXorAssign<4, Rhs> {}
            pub trait ElementShrAssign<
                Rhs: Element = Self,
            >: Element + ShrAssign<
                    Rhs,
                > + ElementVecShrAssign<
                    2,
                    Rhs,
                > + ElementVecShrAssign<3, Rhs> + ElementVecShrAssign<4, Rhs> {}
            pub trait ElementShlAssign<
                Rhs: Element = Self,
            >: Element + ShlAssign<
                    Rhs,
                > + ElementVecShlAssign<
                    2,
                    Rhs,
                > + ElementVecShlAssign<3, Rhs> + ElementVecShlAssign<4, Rhs> {}
        }
        mod rhs_ops {
            use gomath_proc_macros::{ops_macro, rhs_ops};
            use super::*;
            pub trait ElementAdd<
                Rhs: Element = Self,
            >: Element + Add<
                    Rhs,
                    Output: Element,
                > + ElementVecAdd<
                    2,
                    Rhs,
                > + ElementVecAdd<3, Rhs> + ElementVecAdd<4, Rhs> {}
            pub trait ElementSub<
                Rhs: Element = Self,
            >: Element + Sub<
                    Rhs,
                    Output: Element,
                > + ElementVecSub<
                    2,
                    Rhs,
                > + ElementVecSub<3, Rhs> + ElementVecSub<4, Rhs> {}
            pub trait ElementMul<
                Rhs: Element = Self,
            >: Element + Mul<
                    Rhs,
                    Output: Element,
                > + ElementVecMul<
                    2,
                    Rhs,
                > + ElementVecMul<3, Rhs> + ElementVecMul<4, Rhs> {}
            pub trait ElementDiv<
                Rhs: Element = Self,
            >: Element + Div<
                    Rhs,
                    Output: Element,
                > + ElementVecDiv<
                    2,
                    Rhs,
                > + ElementVecDiv<3, Rhs> + ElementVecDiv<4, Rhs> {}
            pub trait ElementRem<
                Rhs: Element = Self,
            >: Element + Rem<
                    Rhs,
                    Output: Element,
                > + ElementVecRem<
                    2,
                    Rhs,
                > + ElementVecRem<3, Rhs> + ElementVecRem<4, Rhs> {}
            pub trait ElementBitAnd<
                Rhs: Element = Self,
            >: Element + BitAnd<
                    Rhs,
                    Output: Element,
                > + ElementVecBitAnd<
                    2,
                    Rhs,
                > + ElementVecBitAnd<3, Rhs> + ElementVecBitAnd<4, Rhs> {}
            pub trait ElementBitOr<
                Rhs: Element = Self,
            >: Element + BitOr<
                    Rhs,
                    Output: Element,
                > + ElementVecBitOr<
                    2,
                    Rhs,
                > + ElementVecBitOr<3, Rhs> + ElementVecBitOr<4, Rhs> {}
            pub trait ElementBitXor<
                Rhs: Element = Self,
            >: Element + BitXor<
                    Rhs,
                    Output: Element,
                > + ElementVecBitXor<
                    2,
                    Rhs,
                > + ElementVecBitXor<3, Rhs> + ElementVecBitXor<4, Rhs> {}
            pub trait ElementShr<
                Rhs: Element = Self,
            >: Element + Shr<
                    Rhs,
                    Output: Element,
                > + ElementVecShr<
                    2,
                    Rhs,
                > + ElementVecShr<3, Rhs> + ElementVecShr<4, Rhs> {}
            pub trait ElementShl<
                Rhs: Element = Self,
            >: Element + Shl<
                    Rhs,
                    Output: Element,
                > + ElementVecShl<
                    2,
                    Rhs,
                > + ElementVecShl<3, Rhs> + ElementVecShl<4, Rhs> {}
        }
        mod self_ops {
            use gomath_proc_macros::{ops_macro, self_ops};
            use super::*;
            pub trait ElementNeg: Element + Neg<
                    Output: Element,
                > + ElementVecNeg<2> + ElementVecNeg<3> + ElementVecNeg<4> {}
            pub trait ElementNot: Element + Not<
                    Output: Element,
                > + ElementVecNot<2> + ElementVecNot<3> + ElementVecNot<4> {}
        }
        pub use assign_ops::*;
        pub use rhs_ops::*;
        pub use self_ops::*;
    }
    pub trait Element: 'static + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display + ElementVec {}
}
pub mod mat {
    use std::fmt::{self, Display, Formatter};
    use crate::{element::*, vec::*};
    mod major {
        use super::*;
        trait Seal {}
        #[allow(private_bounds)]
        pub trait MatMajor: Seal + fmt::Debug + Copy + PartialEq + PartialOrd + Default {
            type Mat2Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
            type Mat2x3Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
            type Mat2x4Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
            type Mat3x2Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
            type Mat3Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
            type Mat3x4Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
            type Mat4x2Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
            type Mat4x3Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
            type Mat4Inner<T: Element>: fmt::Debug
                + Copy
                + PartialEq
                + PartialOrd
                + Default;
        }
        pub struct CM;
        #[automatically_derived]
        impl ::core::fmt::Debug for CM {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "CM")
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CM {
            #[inline]
            fn clone(&self) -> CM {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CM {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CM {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CM {
            #[inline]
            fn eq(&self, other: &CM) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for CM {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for CM {
            #[inline]
            fn partial_cmp(
                &self,
                other: &CM,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for CM {
            #[inline]
            fn cmp(&self, other: &CM) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for CM {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
        }
        #[automatically_derived]
        impl ::core::default::Default for CM {
            #[inline]
            fn default() -> CM {
                CM {}
            }
        }
        impl Seal for CM {}
        impl MatMajor for CM {
            type Mat2Inner<T: Element> = [Vec2<T>; 2];
            type Mat2x3Inner<T: Element> = [Vec3<T>; 2];
            type Mat2x4Inner<T: Element> = [Vec4<T>; 2];
            type Mat3x2Inner<T: Element> = [Vec2<T>; 3];
            type Mat3Inner<T: Element> = [Vec3<T>; 3];
            type Mat3x4Inner<T: Element> = [Vec4<T>; 3];
            type Mat4x2Inner<T: Element> = [Vec2<T>; 4];
            type Mat4x3Inner<T: Element> = [Vec3<T>; 4];
            type Mat4Inner<T: Element> = [Vec4<T>; 4];
        }
        pub struct RM;
        #[automatically_derived]
        impl ::core::fmt::Debug for RM {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "RM")
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RM {
            #[inline]
            fn clone(&self) -> RM {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for RM {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RM {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RM {
            #[inline]
            fn eq(&self, other: &RM) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for RM {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for RM {
            #[inline]
            fn partial_cmp(
                &self,
                other: &RM,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for RM {
            #[inline]
            fn cmp(&self, other: &RM) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for RM {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
        }
        #[automatically_derived]
        impl ::core::default::Default for RM {
            #[inline]
            fn default() -> RM {
                RM {}
            }
        }
        impl Seal for RM {}
        impl MatMajor for RM {
            type Mat2Inner<T: Element> = [Vec2<T>; 2];
            type Mat2x3Inner<T: Element> = [Vec2<T>; 3];
            type Mat2x4Inner<T: Element> = [Vec2<T>; 4];
            type Mat3x2Inner<T: Element> = [Vec3<T>; 2];
            type Mat3Inner<T: Element> = [Vec3<T>; 3];
            type Mat3x4Inner<T: Element> = [Vec3<T>; 4];
            type Mat4x2Inner<T: Element> = [Vec4<T>; 2];
            type Mat4x3Inner<T: Element> = [Vec4<T>; 3];
            type Mat4Inner<T: Element> = [Vec4<T>; 4];
        }
    }
    pub use major::*;
    trait Seal {}
    #[allow(private_bounds)]
    pub trait MatCxR: Seal + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display {
        type T: Element;
        const C: usize;
        const R: usize;
        type M: MatMajor;
    }
    #[repr(transparent)]
    pub struct Mat2<M: MatMajor, T: Element = f32> {
        inner: M::Mat2Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat2",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat2<M, T> {
            Mat2 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat2<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat2<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat2<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat2<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat2<M, T>
    where
        M::Mat2Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat2<M, T> {
            Mat2 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat2<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat2<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat2<M, T> {
        type T = T;
        const C: usize = 2;
        const R: usize = 2;
        type M = M;
    }
    #[repr(transparent)]
    pub struct Mat2x3<M: MatMajor, T: Element = f32> {
        inner: M::Mat2x3Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat2x3",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat2x3<M, T> {
            Mat2x3 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat2x3<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat2x3<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2x3Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat2x3<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat2x3<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat2x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat2x3<M, T> {
            Mat2x3 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat2x3<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat2x3<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat2x3<M, T> {
        type T = T;
        const C: usize = 2;
        const R: usize = 3;
        type M = M;
    }
    #[repr(transparent)]
    pub struct Mat2x4<M: MatMajor, T: Element = f32> {
        inner: M::Mat2x4Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat2x4",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat2x4<M, T> {
            Mat2x4 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat2x4<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat2x4<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2x4Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat2x4<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat2x4<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat2x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat2x4<M, T> {
            Mat2x4 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat2x4<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat2x4<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat2x4<M, T> {
        type T = T;
        const C: usize = 2;
        const R: usize = 4;
        type M = M;
    }
    #[repr(transparent)]
    pub struct Mat3x2<M: MatMajor, T: Element = f32> {
        inner: M::Mat2Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat3x2",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat3x2<M, T> {
            Mat3x2 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat3x2<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat3x2<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat3x2<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat3x2<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat3x2<M, T>
    where
        M::Mat2Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat3x2<M, T> {
            Mat3x2 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat3x2<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat3x2<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat3x2<M, T> {
        type T = T;
        const C: usize = 3;
        const R: usize = 2;
        type M = M;
    }
    #[repr(transparent)]
    pub struct Mat3<M: MatMajor, T: Element = f32> {
        inner: M::Mat2x3Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat3",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat3<M, T> {
            Mat3 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat3<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat3<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2x3Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat3<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat3<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat3<M, T> {
            Mat3 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat3<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat3<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat3<M, T> {
        type T = T;
        const C: usize = 3;
        const R: usize = 3;
        type M = M;
    }
    #[repr(transparent)]
    pub struct Mat3x4<M: MatMajor, T: Element = f32> {
        inner: M::Mat2x4Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat3x4",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat3x4<M, T> {
            Mat3x4 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat3x4<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat3x4<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2x4Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat3x4<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat3x4<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat3x4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat3x4<M, T> {
            Mat3x4 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat3x4<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat3x4<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat3x4<M, T> {
        type T = T;
        const C: usize = 3;
        const R: usize = 4;
        type M = M;
    }
    #[repr(transparent)]
    pub struct Mat4x2<M: MatMajor, T: Element = f32> {
        inner: M::Mat2Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat4x2",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat4x2<M, T> {
            Mat4x2 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat4x2<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat4x2<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat4x2<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat4x2<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat4x2<M, T>
    where
        M::Mat2Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat4x2<M, T> {
            Mat4x2 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat4x2<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat4x2<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat4x2<M, T> {
        type T = T;
        const C: usize = 4;
        const R: usize = 2;
        type M = M;
    }
    #[repr(transparent)]
    pub struct Mat4x3<M: MatMajor, T: Element = f32> {
        inner: M::Mat2x3Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat4x3",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat4x3<M, T> {
            Mat4x3 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat4x3<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat4x3<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2x3Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat4x3<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat4x3<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat4x3<M, T>
    where
        M::Mat2x3Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat4x3<M, T> {
            Mat4x3 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat4x3<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat4x3<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat4x3<M, T> {
        type T = T;
        const C: usize = 4;
        const R: usize = 3;
        type M = M;
    }
    #[repr(transparent)]
    pub struct Mat4<M: MatMajor, T: Element = f32> {
        inner: M::Mat2x4Inner<T>,
    }
    #[automatically_derived]
    impl<
        M: ::core::fmt::Debug + MatMajor,
        T: ::core::fmt::Debug + Element,
    > ::core::fmt::Debug for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mat4",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::clone::Clone + MatMajor,
        T: ::core::clone::Clone + Element,
    > ::core::clone::Clone for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Mat4<M, T> {
            Mat4 {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::marker::Copy + MatMajor,
        T: ::core::marker::Copy + Element,
    > ::core::marker::Copy for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::marker::Copy,
    {}
    #[automatically_derived]
    impl<M: MatMajor, T: Element> ::core::marker::StructuralPartialEq for Mat4<M, T> {}
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialEq + MatMajor,
        T: ::core::cmp::PartialEq + Element,
    > ::core::cmp::PartialEq for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Mat4<M, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Eq + MatMajor, T: ::core::cmp::Eq + Element> ::core::cmp::Eq
    for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<M::Mat2x4Inner<T>>;
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::cmp::PartialOrd + MatMajor,
        T: ::core::cmp::PartialOrd + Element,
    > ::core::cmp::PartialOrd for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::PartialOrd,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Mat4<M, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<M: ::core::cmp::Ord + MatMajor, T: ::core::cmp::Ord + Element> ::core::cmp::Ord
    for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::cmp::Ord,
    {
        #[inline]
        fn cmp(&self, other: &Mat4<M, T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::hash::Hash + MatMajor,
        T: ::core::hash::Hash + Element,
    > ::core::hash::Hash for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::hash::Hash,
    {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl<
        M: ::core::default::Default + MatMajor,
        T: ::core::default::Default + Element,
    > ::core::default::Default for Mat4<M, T>
    where
        M::Mat2x4Inner<T>: ::core::default::Default,
    {
        #[inline]
        fn default() -> Mat4<M, T> {
            Mat4 {
                inner: ::core::default::Default::default(),
            }
        }
    }
    impl<M: MatMajor, T: Element> Seal for Mat4<M, T> {}
    impl<M: MatMajor, T: Element> Display for Mat4<M, T> {
        fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<M: MatMajor, T: Element> MatCxR for Mat4<M, T> {
        type T = T;
        const C: usize = 4;
        const R: usize = 4;
        type M = M;
    }
}
pub mod ops {
    pub use gomath_proc_macros::assign_ops;
    pub use gomath_proc_macros::rhs_ops;
    pub use gomath_proc_macros::self_ops;
}
pub mod vec {
    use crate::element::*;
    use gomath_proc_macros::vecnum_trait;
    use inner::*;
    pub mod array {
        use std::{
            mem::{transmute, transmute_copy},
            ops::{Index, IndexMut},
            slice::SliceIndex,
        };
        use super::*;
        impl<const N: usize, T: Element> VecN<N, T>
        where
            MaybeVecNum<N>: VecNum<N>,
        {
            #[inline(always)]
            pub fn from_array(value: [T; N]) -> Self {
                unsafe { transmute_copy(&value) }
            }
            #[inline(always)]
            pub fn into_array(self) -> [T; N] {
                *unsafe { transmute::<&Self, &[T; N]>(&self) }
            }
            #[inline(always)]
            pub fn as_array(&self) -> &[T; N] {
                unsafe { transmute(self) }
            }
            #[inline(always)]
            pub fn as_array_mut(&mut self) -> &mut [T; N] {
                unsafe { transmute(self) }
            }
            #[inline(always)]
            pub fn iter(self) -> std::array::IntoIter<T, N> {
                self.into_array().into_iter()
            }
            #[inline(always)]
            pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
                self.as_array_mut().iter_mut()
            }
            #[inline(always)]
            pub fn map<T2: Element>(self, f: impl FnMut(T) -> T2) -> VecN<N, T2> {
                VecN::from_array(self.as_array().map(f))
            }
            #[inline(always)]
            pub fn count(self, mut f: impl FnMut(T) -> bool) -> usize {
                Iterator::count(self.iter().filter(|c| f(*c)))
            }
            #[inline(always)]
            pub fn any(self, f: impl FnMut(T) -> bool) -> bool {
                Iterator::any(&mut self.iter(), f)
            }
            #[inline(always)]
            pub fn all(self, f: impl FnMut(T) -> bool) -> bool {
                Iterator::all(&mut self.iter(), f)
            }
        }
        impl<const N: usize, T: Element, Idx: SliceIndex<[T]>> Index<Idx> for VecN<N, T>
        where
            MaybeVecNum<N>: VecNum<N>,
        {
            type Output = Idx::Output;
            #[inline(always)]
            fn index(&self, index: Idx) -> &Self::Output {
                self.as_array().index(index)
            }
        }
        impl<const N: usize, T: Element, Idx: SliceIndex<[T]>> IndexMut<Idx>
        for VecN<N, T>
        where
            MaybeVecNum<N>: VecNum<N>,
        {
            #[inline(always)]
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
                self.as_array_mut().index_mut(index)
            }
        }
    }
    pub mod const_swizzle {
        use super::*;
        mod cget {
            use gomath_proc_macros::vec_cget_wrappers;
            use super::*;
            impl<T: Element> Vec2<T> {
                #[inline(always)]
                pub fn x(self) -> T {
                    unsafe { self.cget::<0usize>() }
                }
                #[inline(always)]
                pub fn y(self) -> T {
                    unsafe { self.cget::<1usize>() }
                }
                #[inline(always)]
                pub fn xx(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xy(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yx(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yy(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 1usize>() }
                }
            }
            impl<T: Element> Vec3<T> {
                #[inline(always)]
                pub fn x(self) -> T {
                    unsafe { self.cget::<0usize>() }
                }
                #[inline(always)]
                pub fn y(self) -> T {
                    unsafe { self.cget::<1usize>() }
                }
                #[inline(always)]
                pub fn z(self) -> T {
                    unsafe { self.cget::<2usize>() }
                }
                #[inline(always)]
                pub fn xx(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xy(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xz(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yx(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yy(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yz(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zx(self) -> Vec2<T> {
                    unsafe { self.cget2::<2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zy(self) -> Vec2<T> {
                    unsafe { self.cget2::<2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zz(self) -> Vec2<T> {
                    unsafe { self.cget2::<2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxz(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyz(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzz(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxz(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyz(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzz(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxz(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zyz(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzx(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzy(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzz(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zyxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zyyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zyzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zyzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 2usize, 2usize>() }
                }
            }
            impl<T: Element> Vec4<T> {
                #[inline(always)]
                pub fn x(self) -> T {
                    unsafe { self.cget::<0usize>() }
                }
                #[inline(always)]
                pub fn y(self) -> T {
                    unsafe { self.cget::<1usize>() }
                }
                #[inline(always)]
                pub fn z(self) -> T {
                    unsafe { self.cget::<2usize>() }
                }
                #[inline(always)]
                pub fn w(self) -> T {
                    unsafe { self.cget::<3usize>() }
                }
                #[inline(always)]
                pub fn xx(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xy(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xz(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xw(self) -> Vec2<T> {
                    unsafe { self.cget2::<0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yx(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yy(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yz(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yw(self) -> Vec2<T> {
                    unsafe { self.cget2::<1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zx(self) -> Vec2<T> {
                    unsafe { self.cget2::<2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zy(self) -> Vec2<T> {
                    unsafe { self.cget2::<2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zz(self) -> Vec2<T> {
                    unsafe { self.cget2::<2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zw(self) -> Vec2<T> {
                    unsafe { self.cget2::<2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wx(self) -> Vec2<T> {
                    unsafe { self.cget2::<3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wy(self) -> Vec2<T> {
                    unsafe { self.cget2::<3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wz(self) -> Vec2<T> {
                    unsafe { self.cget2::<3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn ww(self) -> Vec2<T> {
                    unsafe { self.cget2::<3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxz(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxw(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyz(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyw(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xzx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzz(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzw(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xwx(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xwy(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xwz(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xww(self) -> Vec3<T> {
                    unsafe { self.cget3::<0usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxz(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxw(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyz(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyw(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yzx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzz(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzw(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn ywx(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn ywy(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn ywz(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yww(self) -> Vec3<T> {
                    unsafe { self.cget3::<1usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxz(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxw(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zyz(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyw(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zzx(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzy(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzz(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzw(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zwx(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zwy(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zwz(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zww(self) -> Vec3<T> {
                    unsafe { self.cget3::<2usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wxx(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wxy(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wxz(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wxw(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wyx(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wyy(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wyz(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wyw(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wzx(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wzy(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wzz(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wzw(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wwx(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wwy(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wwz(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn www(self) -> Vec3<T> {
                    unsafe { self.cget3::<3usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xxzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xxwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xxwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xxwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xxww(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 0usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xyzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xyzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xyzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xywx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xywy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xywz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyww(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 1usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xzxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xzyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xzzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xzwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xzwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xzwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xzww(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 2usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xwxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xwxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xwxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xwxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xwyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xwyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xwyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xwyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xwzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xwzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xwzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xwzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xwwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn xwwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xwwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xwww(self) -> Vec4<T> {
                    unsafe { self.cget4::<0usize, 3usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yxzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yxwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yxwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yxwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yxww(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 0usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yyzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yyzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yyzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yywx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yywy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yywz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yyww(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 1usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yzxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yzyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yzzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yzwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn yzwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn yzwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yzww(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 2usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn ywxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn ywxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn ywxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn ywxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn ywyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn ywyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn ywyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn ywyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn ywzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn ywzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn ywzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn ywzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn ywwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn ywwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn ywwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn ywww(self) -> Vec4<T> {
                    unsafe { self.cget4::<1usize, 3usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zxzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zxwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zxwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zxwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zxww(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 0usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zyxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zyyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zyzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zyzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zyzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zywx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zywy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zywz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zyww(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 1usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zzxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zzyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zzzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zzwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zzwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zzwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zzww(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 2usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zwxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zwxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zwxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zwxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zwyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zwyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zwyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zwyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zwzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zwzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zwzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zwzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zwwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn zwwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn zwwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn zwww(self) -> Vec4<T> {
                    unsafe { self.cget4::<2usize, 3usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wxxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wxxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wxxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wxxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wxyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wxyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wxyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wxyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wxzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wxzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wxzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wxzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wxwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wxwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wxwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wxww(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 0usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wyxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wyxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wyxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wyxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wyyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wyyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wyyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wyyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wyzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wyzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wyzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wyzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wywx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wywy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wywz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wyww(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 1usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wzxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wzxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wzxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wzxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wzyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wzyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wzyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wzyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wzzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wzzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wzzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wzzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wzwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wzwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wzwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wzww(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 2usize, 3usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wwxx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 0usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wwxy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wwxz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wwxw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wwyx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 1usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wwyy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 1usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wwyz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wwyw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wwzx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 2usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wwzy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 2usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wwzz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 2usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wwzw(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn wwwx(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 3usize, 0usize>() }
                }
                #[inline(always)]
                pub fn wwwy(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 3usize, 1usize>() }
                }
                #[inline(always)]
                pub fn wwwz(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 3usize, 2usize>() }
                }
                #[inline(always)]
                pub fn wwww(self) -> Vec4<T> {
                    unsafe { self.cget4::<3usize, 3usize, 3usize, 3usize>() }
                }
            }
            pub trait ElementVecConstGet<const N: usize>: ElementVecInner
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                unsafe fn vec_cget<const X: usize>(vec: InnerVec<N, Self>) -> Self;
                unsafe fn vec_cget2<const X: usize, const Y: usize>(
                    vec: InnerVec<N, Self>,
                ) -> Self::InnerVec2;
                unsafe fn vec_cget3<const X: usize, const Y: usize, const Z: usize>(
                    vec: InnerVec<N, Self>,
                ) -> Self::InnerVec3;
                unsafe fn vec_cget4<
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: InnerVec<N, Self>) -> Self::InnerVec4;
            }
            impl<const N: usize, T: Element> VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                pub unsafe fn cget<const X: usize>(self) -> T {
                    MaybeVecNum::<N>::cget::<T, X>(self)
                }
                pub unsafe fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T> {
                    MaybeVecNum::<N>::cget2::<T, X, Y>(self)
                }
                pub unsafe fn cget3<const X: usize, const Y: usize, const Z: usize>(
                    self,
                ) -> Vec3<T> {
                    MaybeVecNum::<N>::cget3::<T, X, Y, Z>(self)
                }
                pub unsafe fn cget4<
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(self) -> Vec4<T> {
                    MaybeVecNum::<N>::cget4::<T, X, Y, Z, W>(self)
                }
            }
            pub(super) trait VecNumConstGet<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                unsafe fn cget<T: Element, const X: usize>(vec: VecN<N, T>) -> T;
                unsafe fn cget2<T: Element, const X: usize, const Y: usize>(
                    vec: VecN<N, T>,
                ) -> Vec2<T>;
                unsafe fn cget3<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                >(vec: VecN<N, T>) -> Vec3<T>;
                unsafe fn cget4<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: VecN<N, T>) -> Vec4<T>;
            }
            impl VecNumConstGet<2> for MaybeVecNum<2> {
                unsafe fn cget<T: Element, const X: usize>(vec: VecN<2, T>) -> T {
                    T::vec_cget(vec)
                }
                unsafe fn cget2<T: Element, const X: usize, const Y: usize>(
                    vec: VecN<2, T>,
                ) -> Vec2<T> {
                    VecN::from_inner(T::vec_cget2(vec))
                }
                unsafe fn cget3<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                >(vec: VecN<2, T>) -> Vec3<T> {
                    VecN::from_inner(T::vec_cget3(vec))
                }
                unsafe fn cget4<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: VecN<2, T>) -> Vec4<T> {
                    VecN::from_inner(T::vec_cget4(vec))
                }
            }
            impl VecNumConstGet<3> for MaybeVecNum<3> {
                unsafe fn cget<T: Element, const X: usize>(vec: VecN<3, T>) -> T {
                    T::vec_cget(vec)
                }
                unsafe fn cget2<T: Element, const X: usize, const Y: usize>(
                    vec: VecN<3, T>,
                ) -> Vec2<T> {
                    VecN::from_inner(T::vec_cget2(vec))
                }
                unsafe fn cget3<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                >(vec: VecN<3, T>) -> Vec3<T> {
                    VecN::from_inner(T::vec_cget3(vec))
                }
                unsafe fn cget4<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: VecN<3, T>) -> Vec4<T> {
                    VecN::from_inner(T::vec_cget4(vec))
                }
            }
            impl VecNumConstGet<4> for MaybeVecNum<4> {
                unsafe fn cget<T: Element, const X: usize>(vec: VecN<4, T>) -> T {
                    T::vec_cget(vec)
                }
                unsafe fn cget2<T: Element, const X: usize, const Y: usize>(
                    vec: VecN<4, T>,
                ) -> Vec2<T> {
                    VecN::from_inner(T::vec_cget2(vec))
                }
                unsafe fn cget3<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                >(vec: VecN<4, T>) -> Vec3<T> {
                    VecN::from_inner(T::vec_cget3(vec))
                }
                unsafe fn cget4<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: VecN<4, T>) -> Vec4<T> {
                    VecN::from_inner(T::vec_cget4(vec))
                }
            }
        }
        mod cget_mut {
            use std::mem::transmute;
            use gomath_proc_macros::vec_cget_mut_wrappers;
            use super::*;
            impl<T: Element> Vec2<T> {
                #[inline(always)]
                pub fn x_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<0usize>() }
                }
                #[inline(always)]
                pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn xy_mut(&mut self) -> &mut Vec2<T> {
                    unsafe { self.cget_mut2::<0usize>() }
                }
                #[inline(always)]
                pub fn y_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<1usize>() }
                }
            }
            impl<T: Element> Vec3<T> {
                #[inline(always)]
                pub fn x_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<0usize>() }
                }
                #[inline(always)]
                pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn x_y_z_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                    unsafe { self.cget_mut_1_1_1::<0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn x_yz_mut(&mut self) -> (&mut T, &mut Vec2<T>) {
                    unsafe { self.cget_mut_1_2::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn x_z_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xy_mut(&mut self) -> &mut Vec2<T> {
                    unsafe { self.cget_mut2::<0usize>() }
                }
                #[inline(always)]
                pub fn xy_z_mut(&mut self) -> (&mut Vec2<T>, &mut T) {
                    unsafe { self.cget_mut_2_1::<0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xyz_mut(&mut self) -> &mut Vec3<T> {
                    unsafe { self.cget_mut3::<0usize>() }
                }
                #[inline(always)]
                pub fn y_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<1usize>() }
                }
                #[inline(always)]
                pub fn y_z_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn yz_mut(&mut self) -> &mut Vec2<T> {
                    unsafe { self.cget_mut2::<1usize>() }
                }
                #[inline(always)]
                pub fn z_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<2usize>() }
                }
            }
            impl<T: Element> Vec4<T> {
                #[inline(always)]
                pub fn x_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<0usize>() }
                }
                #[inline(always)]
                pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn x_y_z_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                    unsafe { self.cget_mut_1_1_1::<0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn x_y_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
                    unsafe { self.cget_mut_1_1_1_1::<0usize, 1usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn x_y_zw_mut(&mut self) -> (&mut T, &mut T, &mut Vec2<T>) {
                    unsafe { self.cget_mut_1_1_2::<0usize, 1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn x_y_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                    unsafe { self.cget_mut_1_1_1::<0usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn x_yz_mut(&mut self) -> (&mut T, &mut Vec2<T>) {
                    unsafe { self.cget_mut_1_2::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn x_yz_w_mut(&mut self) -> (&mut T, &mut Vec2<T>, &mut T) {
                    unsafe { self.cget_mut_1_2_1::<0usize, 1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn x_yzw_mut(&mut self) -> (&mut T, &mut Vec3<T>) {
                    unsafe { self.cget_mut_1_3::<0usize, 1usize>() }
                }
                #[inline(always)]
                pub fn x_z_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn x_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                    unsafe { self.cget_mut_1_1_1::<0usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn x_zw_mut(&mut self) -> (&mut T, &mut Vec2<T>) {
                    unsafe { self.cget_mut_1_2::<0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn x_w_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xy_mut(&mut self) -> &mut Vec2<T> {
                    unsafe { self.cget_mut2::<0usize>() }
                }
                #[inline(always)]
                pub fn xy_z_mut(&mut self) -> (&mut Vec2<T>, &mut T) {
                    unsafe { self.cget_mut_2_1::<0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xy_z_w_mut(&mut self) -> (&mut Vec2<T>, &mut T, &mut T) {
                    unsafe { self.cget_mut_2_1_1::<0usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xy_zw_mut(&mut self) -> (&mut Vec2<T>, &mut Vec2<T>) {
                    unsafe { self.cget_mut_2_2::<0usize, 2usize>() }
                }
                #[inline(always)]
                pub fn xy_w_mut(&mut self) -> (&mut Vec2<T>, &mut T) {
                    unsafe { self.cget_mut_2_1::<0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xyz_mut(&mut self) -> &mut Vec3<T> {
                    unsafe { self.cget_mut3::<0usize>() }
                }
                #[inline(always)]
                pub fn xyz_w_mut(&mut self) -> (&mut Vec3<T>, &mut T) {
                    unsafe { self.cget_mut_3_1::<0usize, 3usize>() }
                }
                #[inline(always)]
                pub fn xyzw_mut(&mut self) -> &mut Vec4<T> {
                    unsafe { self.cget_mut4::<0usize>() }
                }
                #[inline(always)]
                pub fn y_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<1usize>() }
                }
                #[inline(always)]
                pub fn y_z_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn y_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
                    unsafe { self.cget_mut_1_1_1::<1usize, 2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn y_zw_mut(&mut self) -> (&mut T, &mut Vec2<T>) {
                    unsafe { self.cget_mut_1_2::<1usize, 2usize>() }
                }
                #[inline(always)]
                pub fn y_w_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yz_mut(&mut self) -> &mut Vec2<T> {
                    unsafe { self.cget_mut2::<1usize>() }
                }
                #[inline(always)]
                pub fn yz_w_mut(&mut self) -> (&mut Vec2<T>, &mut T) {
                    unsafe { self.cget_mut_2_1::<1usize, 3usize>() }
                }
                #[inline(always)]
                pub fn yzw_mut(&mut self) -> &mut Vec3<T> {
                    unsafe { self.cget_mut3::<1usize>() }
                }
                #[inline(always)]
                pub fn z_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<2usize>() }
                }
                #[inline(always)]
                pub fn z_w_mut(&mut self) -> (&mut T, &mut T) {
                    unsafe { self.cget_mut_1_1::<2usize, 3usize>() }
                }
                #[inline(always)]
                pub fn zw_mut(&mut self) -> &mut Vec2<T> {
                    unsafe { self.cget_mut2::<2usize>() }
                }
                #[inline(always)]
                pub fn w_mut(&mut self) -> &mut T {
                    unsafe { self.cget_mut::<3usize>() }
                }
            }
            impl<const N: usize, T: Element> VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                pub unsafe fn cget_mut<const V0: usize>(&mut self) -> &mut T {
                    self.get_unchecked_mut(V0)
                }
                pub unsafe fn cget_mut2<const V0: usize>(&mut self) -> &mut Vec2<T> {
                    transmute(self.get_unchecked_mut(V0))
                }
                pub unsafe fn cget_mut3<const V0: usize>(&mut self) -> &mut Vec3<T> {
                    transmute(self.get_unchecked_mut(V0))
                }
                pub unsafe fn cget_mut4<const V0: usize>(&mut self) -> &mut Vec4<T> {
                    transmute(self.get_unchecked_mut(V0))
                }
                pub unsafe fn cget_mut_1_1<const V0: usize, const V1: usize>(
                    &mut self,
                ) -> (&mut T, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                    )
                }
                pub unsafe fn cget_mut_2_1<const V0: usize, const V1: usize>(
                    &mut self,
                ) -> (&mut Vec2<T>, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                    )
                }
                pub unsafe fn cget_mut_3_1<const V0: usize, const V1: usize>(
                    &mut self,
                ) -> (&mut Vec3<T>, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                    )
                }
                pub unsafe fn cget_mut_1_2<const V0: usize, const V1: usize>(
                    &mut self,
                ) -> (&mut T, &mut Vec2<T>) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                    )
                }
                pub unsafe fn cget_mut_2_2<const V0: usize, const V1: usize>(
                    &mut self,
                ) -> (&mut Vec2<T>, &mut Vec2<T>) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                    )
                }
                pub unsafe fn cget_mut_1_3<const V0: usize, const V1: usize>(
                    &mut self,
                ) -> (&mut T, &mut Vec3<T>) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                    )
                }
                pub unsafe fn cget_mut_1_1_1<
                    const V0: usize,
                    const V1: usize,
                    const V2: usize,
                >(&mut self) -> (&mut T, &mut T, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                        transmute(self.get_unchecked_mut(V2)),
                    )
                }
                pub unsafe fn cget_mut_2_1_1<
                    const V0: usize,
                    const V1: usize,
                    const V2: usize,
                >(&mut self) -> (&mut Vec2<T>, &mut T, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                        transmute(self.get_unchecked_mut(V2)),
                    )
                }
                pub unsafe fn cget_mut_1_2_1<
                    const V0: usize,
                    const V1: usize,
                    const V2: usize,
                >(&mut self) -> (&mut T, &mut Vec2<T>, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                        transmute(self.get_unchecked_mut(V2)),
                    )
                }
                pub unsafe fn cget_mut_1_1_2<
                    const V0: usize,
                    const V1: usize,
                    const V2: usize,
                >(&mut self) -> (&mut T, &mut T, &mut Vec2<T>) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                        transmute(self.get_unchecked_mut(V2)),
                    )
                }
                pub unsafe fn cget_mut_1_1_1_1<
                    const V0: usize,
                    const V1: usize,
                    const V2: usize,
                    const V3: usize,
                >(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(V0)),
                        transmute(self.get_unchecked_mut(V1)),
                        transmute(self.get_unchecked_mut(V2)),
                        transmute(self.get_unchecked_mut(V3)),
                    )
                }
            }
        }
        mod cset {
            use gomath_proc_macros::vec_cset_wrappers;
            use super::*;
            impl<T: Element> Vec2<T> {
                #[inline(always)]
                pub fn set_x(&mut self, value: T) {
                    unsafe { self.cset::<0usize>(value) }
                }
                #[inline(always)]
                pub fn set_y(&mut self, value: T) {
                    unsafe { self.cset::<1usize>(value) }
                }
                #[inline(always)]
                pub fn set_xy(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_yx(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<1usize, 0usize>(value) }
                }
            }
            impl<T: Element> Vec3<T> {
                #[inline(always)]
                pub fn set_x(&mut self, value: T) {
                    unsafe { self.cset::<0usize>(value) }
                }
                #[inline(always)]
                pub fn set_y(&mut self, value: T) {
                    unsafe { self.cset::<1usize>(value) }
                }
                #[inline(always)]
                pub fn set_z(&mut self, value: T) {
                    unsafe { self.cset::<2usize>(value) }
                }
                #[inline(always)]
                pub fn set_xy(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_xz(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_yx(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_yz(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_zx(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_zy(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_xyz(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<0usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_xzy(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<0usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_yxz(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<1usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_yzx(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<1usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_zxy(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<2usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_zyx(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<2usize, 1usize, 0usize>(value) }
                }
            }
            impl<T: Element> Vec4<T> {
                #[inline(always)]
                pub fn set_x(&mut self, value: T) {
                    unsafe { self.cset::<0usize>(value) }
                }
                #[inline(always)]
                pub fn set_y(&mut self, value: T) {
                    unsafe { self.cset::<1usize>(value) }
                }
                #[inline(always)]
                pub fn set_z(&mut self, value: T) {
                    unsafe { self.cset::<2usize>(value) }
                }
                #[inline(always)]
                pub fn set_w(&mut self, value: T) {
                    unsafe { self.cset::<3usize>(value) }
                }
                #[inline(always)]
                pub fn set_xy(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_xz(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_xw(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_yx(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_yz(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_yw(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_zx(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_zy(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_zw(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_wx(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_wy(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_wz(&mut self, value: Vec2<T>) {
                    unsafe { self.cset2::<3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_xyz(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<0usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_xyw(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<0usize, 1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_xzy(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<0usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_xzw(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<0usize, 2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_xwy(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<0usize, 3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_xwz(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<0usize, 3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_yxz(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<1usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_yxw(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<1usize, 0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_yzx(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<1usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_yzw(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<1usize, 2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_ywx(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<1usize, 3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_ywz(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<1usize, 3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_zxy(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<2usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_zxw(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<2usize, 0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_zyx(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<2usize, 1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_zyw(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<2usize, 1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_zwx(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<2usize, 3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_zwy(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<2usize, 3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_wxy(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<3usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_wxz(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<3usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_wyx(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<3usize, 1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_wyz(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<3usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_wzx(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<3usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_wzy(&mut self, value: Vec3<T>) {
                    unsafe { self.cset3::<3usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_xyzw(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<0usize, 1usize, 2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_xywz(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<0usize, 1usize, 3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_xzyw(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<0usize, 2usize, 1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_xzwy(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<0usize, 2usize, 3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_xwyz(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<0usize, 3usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_xwzy(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<0usize, 3usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_yxzw(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<1usize, 0usize, 2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_yxwz(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<1usize, 0usize, 3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_yzxw(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<1usize, 2usize, 0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_yzwx(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<1usize, 2usize, 3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_ywxz(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<1usize, 3usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_ywzx(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<1usize, 3usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_zxyw(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<2usize, 0usize, 1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_zxwy(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<2usize, 0usize, 3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_zyxw(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<2usize, 1usize, 0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn set_zywx(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<2usize, 1usize, 3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_zwxy(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<2usize, 3usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_zwyx(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<2usize, 3usize, 1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_wxyz(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<3usize, 0usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_wxzy(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<3usize, 0usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_wyxz(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<3usize, 1usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn set_wyzx(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<3usize, 1usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn set_wzxy(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<3usize, 2usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn set_wzyx(&mut self, value: Vec4<T>) {
                    unsafe { self.cset4::<3usize, 2usize, 1usize, 0usize>(value) }
                }
            }
            impl<const N: usize, T: Element> VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                pub unsafe fn cset<const X: usize>(&mut self, value: T) {
                    *self.get_unchecked_mut(X) = value;
                }
                pub unsafe fn cset2<const X: usize, const Y: usize>(
                    &mut self,
                    value: Vec2<T>,
                ) {
                    *self.get_unchecked_mut(X) = value.x();
                    *self.get_unchecked_mut(Y) = value.y();
                }
                pub unsafe fn cset3<const X: usize, const Y: usize, const Z: usize>(
                    &mut self,
                    value: Vec3<T>,
                ) {
                    *self.get_unchecked_mut(X) = value.x();
                    *self.get_unchecked_mut(Y) = value.y();
                    *self.get_unchecked_mut(Z) = value.z();
                }
                pub unsafe fn cset4<
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(&mut self, value: Vec4<T>) {
                    *self.get_unchecked_mut(X) = value.x();
                    *self.get_unchecked_mut(Y) = value.y();
                    *self.get_unchecked_mut(Z) = value.z();
                    *self.get_unchecked_mut(W) = value.w();
                }
            }
        }
        mod cwith {
            use gomath_proc_macros::vec_cwith_wrappers;
            use super::*;
            impl<T: Element> Vec2<T> {
                #[inline(always)]
                pub fn with_x(self, value: T) -> Self {
                    unsafe { self.cwith::<0usize>(value) }
                }
                #[inline(always)]
                pub fn with_y(self, value: T) -> Self {
                    unsafe { self.cwith::<1usize>(value) }
                }
                #[inline(always)]
                pub fn with_xy(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_yx(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<1usize, 0usize>(value) }
                }
            }
            impl<T: Element> Vec3<T> {
                #[inline(always)]
                pub fn with_x(self, value: T) -> Self {
                    unsafe { self.cwith::<0usize>(value) }
                }
                #[inline(always)]
                pub fn with_y(self, value: T) -> Self {
                    unsafe { self.cwith::<1usize>(value) }
                }
                #[inline(always)]
                pub fn with_z(self, value: T) -> Self {
                    unsafe { self.cwith::<2usize>(value) }
                }
                #[inline(always)]
                pub fn with_xy(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_xz(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_yx(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_yz(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_zx(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_zy(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_xyz(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<0usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_xzy(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<0usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_yxz(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<1usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_yzx(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<1usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_zxy(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<2usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_zyx(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<2usize, 1usize, 0usize>(value) }
                }
            }
            impl<T: Element> Vec4<T> {
                #[inline(always)]
                pub fn with_x(self, value: T) -> Self {
                    unsafe { self.cwith::<0usize>(value) }
                }
                #[inline(always)]
                pub fn with_y(self, value: T) -> Self {
                    unsafe { self.cwith::<1usize>(value) }
                }
                #[inline(always)]
                pub fn with_z(self, value: T) -> Self {
                    unsafe { self.cwith::<2usize>(value) }
                }
                #[inline(always)]
                pub fn with_w(self, value: T) -> Self {
                    unsafe { self.cwith::<3usize>(value) }
                }
                #[inline(always)]
                pub fn with_xy(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_xz(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_xw(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_yx(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_yz(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_yw(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_zx(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_zy(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_zw(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_wx(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_wy(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_wz(self, value: Vec2<T>) -> Self {
                    unsafe { self.cwith2::<3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_xyz(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<0usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_xyw(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<0usize, 1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_xzy(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<0usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_xzw(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<0usize, 2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_xwy(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<0usize, 3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_xwz(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<0usize, 3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_yxz(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<1usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_yxw(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<1usize, 0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_yzx(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<1usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_yzw(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<1usize, 2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_ywx(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<1usize, 3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_ywz(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<1usize, 3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_zxy(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<2usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_zxw(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<2usize, 0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_zyx(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<2usize, 1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_zyw(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<2usize, 1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_zwx(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<2usize, 3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_zwy(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<2usize, 3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_wxy(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<3usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_wxz(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<3usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_wyx(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<3usize, 1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_wyz(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<3usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_wzx(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<3usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_wzy(self, value: Vec3<T>) -> Self {
                    unsafe { self.cwith3::<3usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_xyzw(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<0usize, 1usize, 2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_xywz(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<0usize, 1usize, 3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_xzyw(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<0usize, 2usize, 1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_xzwy(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<0usize, 2usize, 3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_xwyz(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<0usize, 3usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_xwzy(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<0usize, 3usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_yxzw(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<1usize, 0usize, 2usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_yxwz(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<1usize, 0usize, 3usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_yzxw(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<1usize, 2usize, 0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_yzwx(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<1usize, 2usize, 3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_ywxz(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<1usize, 3usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_ywzx(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<1usize, 3usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_zxyw(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<2usize, 0usize, 1usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_zxwy(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<2usize, 0usize, 3usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_zyxw(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<2usize, 1usize, 0usize, 3usize>(value) }
                }
                #[inline(always)]
                pub fn with_zywx(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<2usize, 1usize, 3usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_zwxy(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<2usize, 3usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_zwyx(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<2usize, 3usize, 1usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_wxyz(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<3usize, 0usize, 1usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_wxzy(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<3usize, 0usize, 2usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_wyxz(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<3usize, 1usize, 0usize, 2usize>(value) }
                }
                #[inline(always)]
                pub fn with_wyzx(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<3usize, 1usize, 2usize, 0usize>(value) }
                }
                #[inline(always)]
                pub fn with_wzxy(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<3usize, 2usize, 0usize, 1usize>(value) }
                }
                #[inline(always)]
                pub fn with_wzyx(self, value: Vec4<T>) -> Self {
                    unsafe { self.cwith4::<3usize, 2usize, 1usize, 0usize>(value) }
                }
            }
            pub trait ElementVecConstWith<const N: usize>: ElementVecInner
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                unsafe fn vec_cwith<const X: usize>(
                    vec: InnerVec<N, Self>,
                    value: Self,
                ) -> InnerVec<N, Self>;
                unsafe fn vec_cwith2<const X: usize, const Y: usize>(
                    vec: InnerVec<N, Self>,
                    value: Self::InnerVec2,
                ) -> InnerVec<N, Self>;
                unsafe fn vec_cwith3<const X: usize, const Y: usize, const Z: usize>(
                    vec: InnerVec<N, Self>,
                    value: Self::InnerVec3,
                ) -> InnerVec<N, Self>;
                unsafe fn vec_cwith4<
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: InnerVec<N, Self>, value: Self::InnerVec4) -> InnerVec<N, Self>;
            }
            impl<const N: usize, T: Element> VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                pub unsafe fn cwith<const X: usize>(self, value: T) -> Self {
                    MaybeVecNum::<N>::cwith::<T, X>(self, value)
                }
                pub unsafe fn cwith2<const X: usize, const Y: usize>(
                    self,
                    value: Vec2<T>,
                ) -> Self {
                    MaybeVecNum::<N>::cwith2::<T, X, Y>(self, value)
                }
                pub unsafe fn cwith3<const X: usize, const Y: usize, const Z: usize>(
                    self,
                    value: Vec3<T>,
                ) -> Self {
                    MaybeVecNum::<N>::cwith3::<T, X, Y, Z>(self, value)
                }
                pub unsafe fn cwith4<
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(self, value: Vec4<T>) -> Self {
                    MaybeVecNum::<N>::cwith4::<T, X, Y, Z, W>(self, value)
                }
            }
            pub(super) trait VecNumConstWith<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                unsafe fn cwith<T: Element, const X: usize>(
                    vec: VecN<N, T>,
                    value: T,
                ) -> VecN<N, T>;
                unsafe fn cwith2<T: Element, const X: usize, const Y: usize>(
                    vec: VecN<N, T>,
                    value: Vec2<T>,
                ) -> VecN<N, T>;
                unsafe fn cwith3<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                >(vec: VecN<N, T>, value: Vec3<T>) -> VecN<N, T>;
                unsafe fn cwith4<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: VecN<N, T>, value: Vec4<T>) -> VecN<N, T>;
            }
            impl VecNumConstWith<2> for MaybeVecNum<2> {
                unsafe fn cwith<T: Element, const X: usize>(
                    vec: VecN<2, T>,
                    value: T,
                ) -> VecN<2, T> {
                    T::vec_cwith(vec, value)
                }
                unsafe fn cwith2<T: Element, const X: usize, const Y: usize>(
                    vec: VecN<2, T>,
                    value: Vec2<T>,
                ) -> VecN<2, T> {
                    T::vec_cwith2(vec, value.inner)
                }
                unsafe fn cwith3<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                >(vec: VecN<2, T>, value: Vec3<T>) -> VecN<2, T> {
                    T::vec_cwith3(vec, value.inner)
                }
                unsafe fn cwith4<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: VecN<2, T>, value: Vec4<T>) -> VecN<2, T> {
                    T::vec_cwith4(vec, value.inner)
                }
            }
            impl VecNumConstWith<3> for MaybeVecNum<3> {
                unsafe fn cwith<T: Element, const X: usize>(
                    vec: VecN<3, T>,
                    value: T,
                ) -> VecN<3, T> {
                    T::vec_cwith(vec, value)
                }
                unsafe fn cwith2<T: Element, const X: usize, const Y: usize>(
                    vec: VecN<3, T>,
                    value: Vec2<T>,
                ) -> VecN<3, T> {
                    T::vec_cwith2(vec, value.inner)
                }
                unsafe fn cwith3<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                >(vec: VecN<3, T>, value: Vec3<T>) -> VecN<3, T> {
                    T::vec_cwith3(vec, value.inner)
                }
                unsafe fn cwith4<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: VecN<3, T>, value: Vec4<T>) -> VecN<3, T> {
                    T::vec_cwith4(vec, value.inner)
                }
            }
            impl VecNumConstWith<4> for MaybeVecNum<4> {
                unsafe fn cwith<T: Element, const X: usize>(
                    vec: VecN<4, T>,
                    value: T,
                ) -> VecN<4, T> {
                    T::vec_cwith(vec, value)
                }
                unsafe fn cwith2<T: Element, const X: usize, const Y: usize>(
                    vec: VecN<4, T>,
                    value: Vec2<T>,
                ) -> VecN<4, T> {
                    T::vec_cwith2(vec, value.inner)
                }
                unsafe fn cwith3<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                >(vec: VecN<4, T>, value: Vec3<T>) -> VecN<4, T> {
                    T::vec_cwith3(vec, value.inner)
                }
                unsafe fn cwith4<
                    T: Element,
                    const X: usize,
                    const Y: usize,
                    const Z: usize,
                    const W: usize,
                >(vec: VecN<4, T>, value: Vec4<T>) -> VecN<4, T> {
                    T::vec_cwith4(vec, value.inner)
                }
            }
        }
        pub use cget::*;
        pub use cwith::*;
        pub trait ElementVecConstSwizzle: ElementVecConstGet<
                2,
            > + ElementVecConstGet<
                3,
            > + ElementVecConstGet<
                4,
            > + ElementVecConstWith<
                2,
            > + ElementVecConstWith<3> + ElementVecConstWith<4> {}
        pub(super) trait VecNumConstSwizzle<
            const N: usize,
        >: VecNumConstGet<N> + VecNumConstWith<N>
        where
            MaybeVecNum<N>: VecNum<N>,
        {}
        impl VecNumConstSwizzle<2> for MaybeVecNum<2> {}
        impl VecNumConstSwizzle<3> for MaybeVecNum<3> {}
        impl VecNumConstSwizzle<4> for MaybeVecNum<4> {}
    }
    pub mod from_split {
        use super::*;
        #[inline(always)]
        pub fn vec2<T: Element>(value: impl VecSplit<2, T>) -> Vec2<T> {
            value.into_vec()
        }
        #[inline(always)]
        pub fn vec3<T: Element>(value: impl VecSplit<3, T>) -> Vec3<T> {
            value.into_vec()
        }
        #[inline(always)]
        pub fn vec4<T: Element>(value: impl VecSplit<4, T>) -> Vec4<T> {
            value.into_vec()
        }
        impl<const N: usize, T: Element> VecN<N, T>
        where
            MaybeVecNum<N>: VecNum<N>,
        {
            #[inline(always)]
            pub fn from_split<S: VecSplit<N, T>>(value: S) -> Self {
                value.into_vec()
            }
        }
        pub trait VecSplit<const N: usize, T: Element>
        where
            MaybeVecNum<N>: VecNum<N>,
        {
            fn into_vec(self) -> VecN<N, T>;
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<2, T> for (Vec1<T>, Vec1<T>) {
            fn into_vec(self) -> VecN<2, T> {
                VecN::from_array([self.0, self.1])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<2, T> for (Vec2<T>) {
            fn into_vec(self) -> VecN<2, T> {
                self
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<3, T> for (Vec1<T>, Vec1<T>, Vec1<T>) {
            fn into_vec(self) -> VecN<3, T> {
                VecN::from_array([self.0, self.1, self.2])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<3, T> for (Vec2<T>, Vec1<T>) {
            fn into_vec(self) -> VecN<3, T> {
                VecN::from_array([self.0[0], self.0[1], self.1])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<3, T> for (Vec1<T>, Vec2<T>) {
            fn into_vec(self) -> VecN<3, T> {
                VecN::from_array([self.0, self.1[0], self.1[1]])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<3, T> for (Vec3<T>) {
            fn into_vec(self) -> VecN<3, T> {
                self
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<4, T> for (Vec1<T>, Vec1<T>, Vec1<T>, Vec1<T>) {
            fn into_vec(self) -> VecN<4, T> {
                VecN::from_array([self.0, self.1, self.2, self.3])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<4, T> for (Vec2<T>, Vec1<T>, Vec1<T>) {
            fn into_vec(self) -> VecN<4, T> {
                VecN::from_array([self.0[0], self.0[1], self.1, self.2])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<4, T> for (Vec1<T>, Vec2<T>, Vec1<T>) {
            fn into_vec(self) -> VecN<4, T> {
                VecN::from_array([self.0, self.1[0], self.1[1], self.2])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<4, T> for (Vec1<T>, Vec1<T>, Vec2<T>) {
            fn into_vec(self) -> VecN<4, T> {
                VecN::from_array([self.0, self.1, self.2[0], self.2[1]])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<4, T> for (Vec2<T>, Vec2<T>) {
            fn into_vec(self) -> VecN<4, T> {
                VecN::from_array([self.0[0], self.0[1], self.1[0], self.1[1]])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<4, T> for (Vec3<T>, Vec1<T>) {
            fn into_vec(self) -> VecN<4, T> {
                VecN::from_array([self.0[0], self.0[1], self.0[2], self.1])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<4, T> for (Vec1<T>, Vec3<T>) {
            fn into_vec(self) -> VecN<4, T> {
                VecN::from_array([self.0, self.1[0], self.1[1], self.1[2]])
            }
        }
        #[allow(unused_parens)]
        impl<T: Element> VecSplit<4, T> for (Vec4<T>) {
            fn into_vec(self) -> VecN<4, T> {
                self
            }
        }
        type Vec1<T> = T;
    }
    pub mod inner {
        use std::mem::transmute;
        use super::*;
        pub use gomath_proc_macros::impl_element_vec_inner;
        pub unsafe trait ElementVecInner: Sized {
            type InnerVec2: std::fmt::Debug + Copy + PartialEq + PartialOrd;
            type InnerVec3: std::fmt::Debug + Copy + PartialEq + PartialOrd;
            type InnerVec4: std::fmt::Debug + Copy + PartialEq + PartialOrd;
        }
        pub type InnerVec<const N: usize, T> = <MaybeVecNum<
            N,
        > as VecNumInner<N>>::InnerVec<T>;
        pub type InnerVec2<T> = InnerVec<2, T>;
        pub type InnerVec3<T> = InnerVec<3, T>;
        pub type InnerVec4<T> = InnerVec<4, T>;
        impl<const N: usize, T: Element> VecN<N, T>
        where
            MaybeVecNum<N>: VecNum<N>,
        {
            #[inline(always)]
            pub fn from_inner(inner: InnerVec<N, T>) -> Self {
                Self { inner }
            }
            #[inline(always)]
            pub fn from_inner_ref(inner: &InnerVec<N, T>) -> &Self {
                unsafe { transmute(inner) }
            }
            #[inline(always)]
            pub fn from_inner_mut(inner: &mut InnerVec<N, T>) -> &mut Self {
                unsafe { transmute(inner) }
            }
            #[inline(always)]
            pub fn inner(&self) -> &InnerVec<N, T> {
                &self.inner
            }
            #[inline(always)]
            pub fn inner_mut(&mut self) -> &mut InnerVec<N, T> {
                &mut self.inner
            }
            #[inline(always)]
            pub fn into_inner(self) -> InnerVec<N, T> {
                self.inner
            }
        }
        const _: &'static str = "pub(super) trait VecNumInner < const N : usize > where MaybeVecNum < N > :\nVecNum < N >\n{\n    type InnerVec < T : ElementVecInner > : std :: fmt :: Debug + Copy +\n    PartialEq + PartialOrd;\n} impl VecNumInner < 2 > for MaybeVecNum < 2 >\n{ type InnerVec < T : ElementVecInner > = T :: InnerVec2 < > ; } impl\nVecNumInner < 3 > for MaybeVecNum < 3 >\n{ type InnerVec < T : ElementVecInner > = T :: InnerVec3 < > ; } impl\nVecNumInner < 4 > for MaybeVecNum < 4 >\n{ type InnerVec < T : ElementVecInner > = T :: InnerVec4 < > ; } const _ : ()\n= {};";
    }
    pub mod num {}
    pub mod ops {
        use std::ops::*;
        use super::*;
        use crate::element::ops::*;
        mod assign_ops {
            use gomath_proc_macros::{assign_ops, ops_macro};
            use super::*;
            pub trait ElementVecAddAssign<
                const N: usize,
                Rhs: Element,
            >: Element + AddAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_add_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumAddAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn add_assign<Rhs: Element, T: ElementAddAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumAddAssign<2> for MaybeVecNum<2> {
                fn add_assign<Rhs: Element, T: ElementAddAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_add_assign(vec, rhs)
                }
            }
            impl VecNumAddAssign<3> for MaybeVecNum<3> {
                fn add_assign<Rhs: Element, T: ElementAddAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_add_assign(vec, rhs)
                }
            }
            impl VecNumAddAssign<4> for MaybeVecNum<4> {
                fn add_assign<Rhs: Element, T: ElementAddAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_add_assign(vec, rhs)
                }
            }
            pub trait ElementVecSubAssign<
                const N: usize,
                Rhs: Element,
            >: Element + SubAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_sub_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumSubAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn sub_assign<Rhs: Element, T: ElementSubAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumSubAssign<2> for MaybeVecNum<2> {
                fn sub_assign<Rhs: Element, T: ElementSubAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_sub_assign(vec, rhs)
                }
            }
            impl VecNumSubAssign<3> for MaybeVecNum<3> {
                fn sub_assign<Rhs: Element, T: ElementSubAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_sub_assign(vec, rhs)
                }
            }
            impl VecNumSubAssign<4> for MaybeVecNum<4> {
                fn sub_assign<Rhs: Element, T: ElementSubAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_sub_assign(vec, rhs)
                }
            }
            pub trait ElementVecMulAssign<
                const N: usize,
                Rhs: Element,
            >: Element + MulAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_mul_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumMulAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn mul_assign<Rhs: Element, T: ElementMulAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumMulAssign<2> for MaybeVecNum<2> {
                fn mul_assign<Rhs: Element, T: ElementMulAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_mul_assign(vec, rhs)
                }
            }
            impl VecNumMulAssign<3> for MaybeVecNum<3> {
                fn mul_assign<Rhs: Element, T: ElementMulAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_mul_assign(vec, rhs)
                }
            }
            impl VecNumMulAssign<4> for MaybeVecNum<4> {
                fn mul_assign<Rhs: Element, T: ElementMulAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_mul_assign(vec, rhs)
                }
            }
            pub trait ElementVecDivAssign<
                const N: usize,
                Rhs: Element,
            >: Element + DivAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_div_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumDivAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn div_assign<Rhs: Element, T: ElementDivAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumDivAssign<2> for MaybeVecNum<2> {
                fn div_assign<Rhs: Element, T: ElementDivAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_div_assign(vec, rhs)
                }
            }
            impl VecNumDivAssign<3> for MaybeVecNum<3> {
                fn div_assign<Rhs: Element, T: ElementDivAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_div_assign(vec, rhs)
                }
            }
            impl VecNumDivAssign<4> for MaybeVecNum<4> {
                fn div_assign<Rhs: Element, T: ElementDivAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_div_assign(vec, rhs)
                }
            }
            pub trait ElementVecRemAssign<
                const N: usize,
                Rhs: Element,
            >: Element + RemAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_rem_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumRemAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn rem_assign<Rhs: Element, T: ElementRemAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumRemAssign<2> for MaybeVecNum<2> {
                fn rem_assign<Rhs: Element, T: ElementRemAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_rem_assign(vec, rhs)
                }
            }
            impl VecNumRemAssign<3> for MaybeVecNum<3> {
                fn rem_assign<Rhs: Element, T: ElementRemAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_rem_assign(vec, rhs)
                }
            }
            impl VecNumRemAssign<4> for MaybeVecNum<4> {
                fn rem_assign<Rhs: Element, T: ElementRemAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_rem_assign(vec, rhs)
                }
            }
            pub trait ElementVecBitAndAssign<
                const N: usize,
                Rhs: Element,
            >: Element + BitAndAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_bitand_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumBitAndAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn bitand_assign<Rhs: Element, T: ElementBitAndAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumBitAndAssign<2> for MaybeVecNum<2> {
                fn bitand_assign<Rhs: Element, T: ElementBitAndAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_bitand_assign(vec, rhs)
                }
            }
            impl VecNumBitAndAssign<3> for MaybeVecNum<3> {
                fn bitand_assign<Rhs: Element, T: ElementBitAndAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_bitand_assign(vec, rhs)
                }
            }
            impl VecNumBitAndAssign<4> for MaybeVecNum<4> {
                fn bitand_assign<Rhs: Element, T: ElementBitAndAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_bitand_assign(vec, rhs)
                }
            }
            pub trait ElementVecBitOrAssign<
                const N: usize,
                Rhs: Element,
            >: Element + BitOrAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_bitor_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumBitOrAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn bitor_assign<Rhs: Element, T: ElementBitOrAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumBitOrAssign<2> for MaybeVecNum<2> {
                fn bitor_assign<Rhs: Element, T: ElementBitOrAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_bitor_assign(vec, rhs)
                }
            }
            impl VecNumBitOrAssign<3> for MaybeVecNum<3> {
                fn bitor_assign<Rhs: Element, T: ElementBitOrAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_bitor_assign(vec, rhs)
                }
            }
            impl VecNumBitOrAssign<4> for MaybeVecNum<4> {
                fn bitor_assign<Rhs: Element, T: ElementBitOrAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_bitor_assign(vec, rhs)
                }
            }
            pub trait ElementVecBitXorAssign<
                const N: usize,
                Rhs: Element,
            >: Element + BitXorAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_bitxor_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumBitXorAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn bitxor_assign<Rhs: Element, T: ElementBitXorAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumBitXorAssign<2> for MaybeVecNum<2> {
                fn bitxor_assign<Rhs: Element, T: ElementBitXorAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_bitxor_assign(vec, rhs)
                }
            }
            impl VecNumBitXorAssign<3> for MaybeVecNum<3> {
                fn bitxor_assign<Rhs: Element, T: ElementBitXorAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_bitxor_assign(vec, rhs)
                }
            }
            impl VecNumBitXorAssign<4> for MaybeVecNum<4> {
                fn bitxor_assign<Rhs: Element, T: ElementBitXorAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_bitxor_assign(vec, rhs)
                }
            }
            pub trait ElementVecShrAssign<
                const N: usize,
                Rhs: Element,
            >: Element + ShrAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_shr_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumShrAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn shr_assign<Rhs: Element, T: ElementShrAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumShrAssign<2> for MaybeVecNum<2> {
                fn shr_assign<Rhs: Element, T: ElementShrAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_shr_assign(vec, rhs)
                }
            }
            impl VecNumShrAssign<3> for MaybeVecNum<3> {
                fn shr_assign<Rhs: Element, T: ElementShrAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_shr_assign(vec, rhs)
                }
            }
            impl VecNumShrAssign<4> for MaybeVecNum<4> {
                fn shr_assign<Rhs: Element, T: ElementShrAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_shr_assign(vec, rhs)
                }
            }
            pub trait ElementVecShlAssign<
                const N: usize,
                Rhs: Element,
            >: Element + ShlAssign<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_shl_assign(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }
            trait VecNumShlAssign<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn shl_assign<Rhs: Element, T: ElementShlAssign<Rhs>>(
                    vec: &mut VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                );
            }
            impl VecNumShlAssign<2> for MaybeVecNum<2> {
                fn shl_assign<Rhs: Element, T: ElementShlAssign<Rhs>>(
                    vec: &mut VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) {
                    T::vec_shl_assign(vec, rhs)
                }
            }
            impl VecNumShlAssign<3> for MaybeVecNum<3> {
                fn shl_assign<Rhs: Element, T: ElementShlAssign<Rhs>>(
                    vec: &mut VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) {
                    T::vec_shl_assign(vec, rhs)
                }
            }
            impl VecNumShlAssign<4> for MaybeVecNum<4> {
                fn shl_assign<Rhs: Element, T: ElementShlAssign<Rhs>>(
                    vec: &mut VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) {
                    T::vec_shl_assign(vec, rhs)
                }
            }
            pub(super) trait VecNumAssignOps<
                const N: usize,
            >: VecNumAddAssign<
                    N,
                > + VecNumSubAssign<
                    N,
                > + VecNumMulAssign<
                    N,
                > + VecNumDivAssign<
                    N,
                > + VecNumRemAssign<
                    N,
                > + VecNumBitAndAssign<
                    N,
                > + VecNumBitOrAssign<
                    N,
                > + VecNumBitXorAssign<N> + VecNumShrAssign<N> + VecNumShlAssign<N>
            where
                MaybeVecNum<N>: VecNum<N>,
            {}
            impl VecNumAssignOps<2> for MaybeVecNum<2> {}
            impl VecNumAssignOps<3> for MaybeVecNum<3> {}
            impl VecNumAssignOps<4> for MaybeVecNum<4> {}
        }
        mod component_dot {
            use super::*;
            pub trait VecNComponentProduct {
                type ComponentProduct;
                fn component_dot(self) -> Self::ComponentProduct;
            }
            impl<T: Element + Mul<Output = T>> VecNComponentProduct for Vec2<T> {
                type ComponentProduct = T::Output;
                #[inline(always)]
                fn component_dot(self) -> Self::ComponentProduct {
                    self.x() * self.y()
                }
            }
            impl<T: Element + Mul<Output = T>> VecNComponentProduct for Vec3<T> {
                type ComponentProduct = T::Output;
                #[inline(always)]
                fn component_dot(self) -> Self::ComponentProduct {
                    self.x() * self.y() * self.z()
                }
            }
            impl<T: Element + Mul<Output = T>> VecNComponentProduct for Vec4<T> {
                type ComponentProduct = T::Output;
                #[inline(always)]
                fn component_dot(self) -> Self::ComponentProduct {
                    self.x() * self.y() * self.z() * self.w()
                }
            }
        }
        mod component_sum {
            use super::*;
            pub trait VecNComponentSum {
                type ComponentSum;
                fn component_sum(self) -> Self::ComponentSum;
            }
            impl<T: Element + Add<Output = T>> VecNComponentSum for Vec2<T> {
                type ComponentSum = T::Output;
                #[inline(always)]
                fn component_sum(self) -> Self::ComponentSum {
                    self.x() + self.y()
                }
            }
            impl<T: Element + Add<Output = T>> VecNComponentSum for Vec3<T> {
                type ComponentSum = T::Output;
                #[inline(always)]
                fn component_sum(self) -> Self::ComponentSum {
                    self.x() + self.y() + self.z()
                }
            }
            impl<T: Element + Add<Output = T>> VecNComponentSum for Vec4<T> {
                type ComponentSum = T::Output;
                #[inline(always)]
                fn component_sum(self) -> Self::ComponentSum {
                    self.x() + self.y() + self.z() + self.w()
                }
            }
        }
        mod rhs_ops {
            use gomath_proc_macros::{ops_macro, rhs_ops};
            use super::*;
            pub trait ElementVecAdd<
                const N: usize,
                Rhs: Element,
            >: Element + Add<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_add(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as Add<Rhs>>::Output>;
            }
            impl<const N: usize, T: ElementAdd<Rhs>, Rhs: Element> Add<VecN<N, Rhs>>
            for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn add(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_add(self, rhs)
                }
            }
            trait VecNumAdd<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn add<Rhs: Element, T: ElementAdd<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as Add<Rhs>>::Output>;
            }
            impl VecNumAdd<2> for MaybeVecNum<2> {
                fn add<Rhs: Element, T: ElementAdd<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as Add<Rhs>>::Output> {
                    T::vec_add(vec, rhs)
                }
            }
            impl VecNumAdd<3> for MaybeVecNum<3> {
                fn add<Rhs: Element, T: ElementAdd<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as Add<Rhs>>::Output> {
                    T::vec_add(vec, rhs)
                }
            }
            impl VecNumAdd<4> for MaybeVecNum<4> {
                fn add<Rhs: Element, T: ElementAdd<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as Add<Rhs>>::Output> {
                    T::vec_add(vec, rhs)
                }
            }
            pub trait ElementVecSub<
                const N: usize,
                Rhs: Element,
            >: Element + Sub<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_sub(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as Sub<Rhs>>::Output>;
            }
            impl<const N: usize, T: ElementSub<Rhs>, Rhs: Element> Sub<VecN<N, Rhs>>
            for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn sub(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_sub(self, rhs)
                }
            }
            trait VecNumSub<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn sub<Rhs: Element, T: ElementSub<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as Sub<Rhs>>::Output>;
            }
            impl VecNumSub<2> for MaybeVecNum<2> {
                fn sub<Rhs: Element, T: ElementSub<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as Sub<Rhs>>::Output> {
                    T::vec_sub(vec, rhs)
                }
            }
            impl VecNumSub<3> for MaybeVecNum<3> {
                fn sub<Rhs: Element, T: ElementSub<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as Sub<Rhs>>::Output> {
                    T::vec_sub(vec, rhs)
                }
            }
            impl VecNumSub<4> for MaybeVecNum<4> {
                fn sub<Rhs: Element, T: ElementSub<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as Sub<Rhs>>::Output> {
                    T::vec_sub(vec, rhs)
                }
            }
            pub trait ElementVecMul<
                const N: usize,
                Rhs: Element,
            >: Element + Mul<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_mul(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as Mul<Rhs>>::Output>;
            }
            impl<const N: usize, T: ElementMul<Rhs>, Rhs: Element> Mul<VecN<N, Rhs>>
            for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn mul(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_mul(self, rhs)
                }
            }
            trait VecNumMul<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn mul<Rhs: Element, T: ElementMul<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as Mul<Rhs>>::Output>;
            }
            impl VecNumMul<2> for MaybeVecNum<2> {
                fn mul<Rhs: Element, T: ElementMul<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as Mul<Rhs>>::Output> {
                    T::vec_mul(vec, rhs)
                }
            }
            impl VecNumMul<3> for MaybeVecNum<3> {
                fn mul<Rhs: Element, T: ElementMul<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as Mul<Rhs>>::Output> {
                    T::vec_mul(vec, rhs)
                }
            }
            impl VecNumMul<4> for MaybeVecNum<4> {
                fn mul<Rhs: Element, T: ElementMul<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as Mul<Rhs>>::Output> {
                    T::vec_mul(vec, rhs)
                }
            }
            pub trait ElementVecDiv<
                const N: usize,
                Rhs: Element,
            >: Element + Div<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_div(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as Div<Rhs>>::Output>;
            }
            impl<const N: usize, T: ElementDiv<Rhs>, Rhs: Element> Div<VecN<N, Rhs>>
            for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn div(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_div(self, rhs)
                }
            }
            trait VecNumDiv<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn div<Rhs: Element, T: ElementDiv<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as Div<Rhs>>::Output>;
            }
            impl VecNumDiv<2> for MaybeVecNum<2> {
                fn div<Rhs: Element, T: ElementDiv<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as Div<Rhs>>::Output> {
                    T::vec_div(vec, rhs)
                }
            }
            impl VecNumDiv<3> for MaybeVecNum<3> {
                fn div<Rhs: Element, T: ElementDiv<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as Div<Rhs>>::Output> {
                    T::vec_div(vec, rhs)
                }
            }
            impl VecNumDiv<4> for MaybeVecNum<4> {
                fn div<Rhs: Element, T: ElementDiv<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as Div<Rhs>>::Output> {
                    T::vec_div(vec, rhs)
                }
            }
            pub trait ElementVecRem<
                const N: usize,
                Rhs: Element,
            >: Element + Rem<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_rem(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as Rem<Rhs>>::Output>;
            }
            impl<const N: usize, T: ElementRem<Rhs>, Rhs: Element> Rem<VecN<N, Rhs>>
            for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn rem(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_rem(self, rhs)
                }
            }
            trait VecNumRem<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn rem<Rhs: Element, T: ElementRem<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as Rem<Rhs>>::Output>;
            }
            impl VecNumRem<2> for MaybeVecNum<2> {
                fn rem<Rhs: Element, T: ElementRem<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as Rem<Rhs>>::Output> {
                    T::vec_rem(vec, rhs)
                }
            }
            impl VecNumRem<3> for MaybeVecNum<3> {
                fn rem<Rhs: Element, T: ElementRem<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as Rem<Rhs>>::Output> {
                    T::vec_rem(vec, rhs)
                }
            }
            impl VecNumRem<4> for MaybeVecNum<4> {
                fn rem<Rhs: Element, T: ElementRem<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as Rem<Rhs>>::Output> {
                    T::vec_rem(vec, rhs)
                }
            }
            pub trait ElementVecBitAnd<
                const N: usize,
                Rhs: Element,
            >: Element + BitAnd<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_bitand(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as BitAnd<Rhs>>::Output>;
            }
            impl<
                const N: usize,
                T: ElementBitAnd<Rhs>,
                Rhs: Element,
            > BitAnd<VecN<N, Rhs>> for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn bitand(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_bitand(self, rhs)
                }
            }
            trait VecNumBitAnd<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn bitand<Rhs: Element, T: ElementBitAnd<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as BitAnd<Rhs>>::Output>;
            }
            impl VecNumBitAnd<2> for MaybeVecNum<2> {
                fn bitand<Rhs: Element, T: ElementBitAnd<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as BitAnd<Rhs>>::Output> {
                    T::vec_bitand(vec, rhs)
                }
            }
            impl VecNumBitAnd<3> for MaybeVecNum<3> {
                fn bitand<Rhs: Element, T: ElementBitAnd<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as BitAnd<Rhs>>::Output> {
                    T::vec_bitand(vec, rhs)
                }
            }
            impl VecNumBitAnd<4> for MaybeVecNum<4> {
                fn bitand<Rhs: Element, T: ElementBitAnd<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as BitAnd<Rhs>>::Output> {
                    T::vec_bitand(vec, rhs)
                }
            }
            pub trait ElementVecBitOr<
                const N: usize,
                Rhs: Element,
            >: Element + BitOr<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_bitor(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as BitOr<Rhs>>::Output>;
            }
            impl<const N: usize, T: ElementBitOr<Rhs>, Rhs: Element> BitOr<VecN<N, Rhs>>
            for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn bitor(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_bitor(self, rhs)
                }
            }
            trait VecNumBitOr<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn bitor<Rhs: Element, T: ElementBitOr<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as BitOr<Rhs>>::Output>;
            }
            impl VecNumBitOr<2> for MaybeVecNum<2> {
                fn bitor<Rhs: Element, T: ElementBitOr<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as BitOr<Rhs>>::Output> {
                    T::vec_bitor(vec, rhs)
                }
            }
            impl VecNumBitOr<3> for MaybeVecNum<3> {
                fn bitor<Rhs: Element, T: ElementBitOr<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as BitOr<Rhs>>::Output> {
                    T::vec_bitor(vec, rhs)
                }
            }
            impl VecNumBitOr<4> for MaybeVecNum<4> {
                fn bitor<Rhs: Element, T: ElementBitOr<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as BitOr<Rhs>>::Output> {
                    T::vec_bitor(vec, rhs)
                }
            }
            pub trait ElementVecBitXor<
                const N: usize,
                Rhs: Element,
            >: Element + BitXor<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_bitxor(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as BitXor<Rhs>>::Output>;
            }
            impl<
                const N: usize,
                T: ElementBitXor<Rhs>,
                Rhs: Element,
            > BitXor<VecN<N, Rhs>> for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn bitxor(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_bitxor(self, rhs)
                }
            }
            trait VecNumBitXor<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn bitxor<Rhs: Element, T: ElementBitXor<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as BitXor<Rhs>>::Output>;
            }
            impl VecNumBitXor<2> for MaybeVecNum<2> {
                fn bitxor<Rhs: Element, T: ElementBitXor<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as BitXor<Rhs>>::Output> {
                    T::vec_bitxor(vec, rhs)
                }
            }
            impl VecNumBitXor<3> for MaybeVecNum<3> {
                fn bitxor<Rhs: Element, T: ElementBitXor<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as BitXor<Rhs>>::Output> {
                    T::vec_bitxor(vec, rhs)
                }
            }
            impl VecNumBitXor<4> for MaybeVecNum<4> {
                fn bitxor<Rhs: Element, T: ElementBitXor<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as BitXor<Rhs>>::Output> {
                    T::vec_bitxor(vec, rhs)
                }
            }
            pub trait ElementVecShr<
                const N: usize,
                Rhs: Element,
            >: Element + Shr<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_shr(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as Shr<Rhs>>::Output>;
            }
            impl<const N: usize, T: ElementShr<Rhs>, Rhs: Element> Shr<VecN<N, Rhs>>
            for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn shr(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_shr(self, rhs)
                }
            }
            trait VecNumShr<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn shr<Rhs: Element, T: ElementShr<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as Shr<Rhs>>::Output>;
            }
            impl VecNumShr<2> for MaybeVecNum<2> {
                fn shr<Rhs: Element, T: ElementShr<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as Shr<Rhs>>::Output> {
                    T::vec_shr(vec, rhs)
                }
            }
            impl VecNumShr<3> for MaybeVecNum<3> {
                fn shr<Rhs: Element, T: ElementShr<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as Shr<Rhs>>::Output> {
                    T::vec_shr(vec, rhs)
                }
            }
            impl VecNumShr<4> for MaybeVecNum<4> {
                fn shr<Rhs: Element, T: ElementShr<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as Shr<Rhs>>::Output> {
                    T::vec_shr(vec, rhs)
                }
            }
            pub trait ElementVecShl<
                const N: usize,
                Rhs: Element,
            >: Element + Shl<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_shl(
                    vec: VecN<N, Self>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <Self as Shl<Rhs>>::Output>;
            }
            impl<const N: usize, T: ElementShl<Rhs>, Rhs: Element> Shl<VecN<N, Rhs>>
            for VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                type Output = VecN<N, T::Output>;
                fn shl(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::vec_shl(self, rhs)
                }
            }
            trait VecNumShl<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn shl<Rhs: Element, T: ElementShl<Rhs>>(
                    vec: VecN<N, T>,
                    rhs: VecN<N, Rhs>,
                ) -> VecN<N, <T as Shl<Rhs>>::Output>;
            }
            impl VecNumShl<2> for MaybeVecNum<2> {
                fn shl<Rhs: Element, T: ElementShl<Rhs>>(
                    vec: VecN<2, T>,
                    rhs: VecN<2, Rhs>,
                ) -> VecN<2, <T as Shl<Rhs>>::Output> {
                    T::vec_shl(vec, rhs)
                }
            }
            impl VecNumShl<3> for MaybeVecNum<3> {
                fn shl<Rhs: Element, T: ElementShl<Rhs>>(
                    vec: VecN<3, T>,
                    rhs: VecN<3, Rhs>,
                ) -> VecN<3, <T as Shl<Rhs>>::Output> {
                    T::vec_shl(vec, rhs)
                }
            }
            impl VecNumShl<4> for MaybeVecNum<4> {
                fn shl<Rhs: Element, T: ElementShl<Rhs>>(
                    vec: VecN<4, T>,
                    rhs: VecN<4, Rhs>,
                ) -> VecN<4, <T as Shl<Rhs>>::Output> {
                    T::vec_shl(vec, rhs)
                }
            }
            pub(super) trait VecNumRhsOps<
                const N: usize,
            >: VecNumAdd<
                    N,
                > + VecNumSub<
                    N,
                > + VecNumMul<
                    N,
                > + VecNumDiv<
                    N,
                > + VecNumRem<
                    N,
                > + VecNumBitAnd<
                    N,
                > + VecNumBitOr<N> + VecNumBitXor<N> + VecNumShr<N> + VecNumShl<N>
            where
                MaybeVecNum<N>: VecNum<N>,
            {}
            impl VecNumRhsOps<2> for MaybeVecNum<2> {}
            impl VecNumRhsOps<3> for MaybeVecNum<3> {}
            impl VecNumRhsOps<4> for MaybeVecNum<4> {}
        }
        mod self_ops {
            use gomath_proc_macros::{ops_macro, self_ops};
            use super::*;
            pub trait ElementVecNeg<const N: usize>: Element + Neg<Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_neg(vec: VecN<N, Self>) -> VecN<N, <Self as Neg>::Output>;
            }
            trait VecNumNeg<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn neg<T: ElementNeg>(vec: VecN<N, T>) -> VecN<N, <T as Neg>::Output>;
            }
            impl VecNumNeg<2> for MaybeVecNum<2> {
                fn neg<T: ElementNeg>(vec: VecN<2, T>) -> VecN<2, <T as Neg>::Output> {
                    T::vec_neg(vec)
                }
            }
            impl VecNumNeg<3> for MaybeVecNum<3> {
                fn neg<T: ElementNeg>(vec: VecN<3, T>) -> VecN<3, <T as Neg>::Output> {
                    T::vec_neg(vec)
                }
            }
            impl VecNumNeg<4> for MaybeVecNum<4> {
                fn neg<T: ElementNeg>(vec: VecN<4, T>) -> VecN<4, <T as Neg>::Output> {
                    T::vec_neg(vec)
                }
            }
            pub trait ElementVecNot<const N: usize>: Element + Not<Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_not(vec: VecN<N, Self>) -> VecN<N, <Self as Not>::Output>;
            }
            trait VecNumNot<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn not<T: ElementNot>(vec: VecN<N, T>) -> VecN<N, <T as Not>::Output>;
            }
            impl VecNumNot<2> for MaybeVecNum<2> {
                fn not<T: ElementNot>(vec: VecN<2, T>) -> VecN<2, <T as Not>::Output> {
                    T::vec_not(vec)
                }
            }
            impl VecNumNot<3> for MaybeVecNum<3> {
                fn not<T: ElementNot>(vec: VecN<3, T>) -> VecN<3, <T as Not>::Output> {
                    T::vec_not(vec)
                }
            }
            impl VecNumNot<4> for MaybeVecNum<4> {
                fn not<T: ElementNot>(vec: VecN<4, T>) -> VecN<4, <T as Not>::Output> {
                    T::vec_not(vec)
                }
            }
            pub(super) trait VecNumSelfOps<const N: usize>: VecNumNeg<N> + VecNumNot<N>
            where
                MaybeVecNum<N>: VecNum<N>,
            {}
            impl VecNumSelfOps<2> for MaybeVecNum<2> {}
            impl VecNumSelfOps<3> for MaybeVecNum<3> {}
            impl VecNumSelfOps<4> for MaybeVecNum<4> {}
        }
        pub use assign_ops::*;
        pub use rhs_ops::*;
        pub use self_ops::*;
        pub(super) trait VecNumOps<const N: usize>: VecNumSelfOps<N> + VecNumRhsOps<N>
        where
            MaybeVecNum<N>: VecNum<N>,
        {}
        impl VecNumOps<2> for MaybeVecNum<2> {}
        impl VecNumOps<3> for MaybeVecNum<3> {}
        impl VecNumOps<4> for MaybeVecNum<4> {}
    }
    pub mod splat {
        use super::*;
        #[inline(always)]
        pub fn splat2<T: Element>(value: T) -> Vec2<T> {
            Vec2::splat(value)
        }
        #[inline(always)]
        pub fn splat3<T: Element>(value: T) -> Vec3<T> {
            Vec3::splat(value)
        }
        #[inline(always)]
        pub fn splat4<T: Element>(value: T) -> Vec4<T> {
            Vec4::splat(value)
        }
        pub trait VecNSplat<T: Element> {
            fn splat(value: T) -> Self;
        }
        pub trait ElementVecSplat: ElementVecInner {
            fn vec2_splat(value: Self) -> Self::InnerVec2;
            fn vec3_splat(value: Self) -> Self::InnerVec3;
            fn vec4_splat(value: Self) -> Self::InnerVec4;
        }
        impl<T: Element> VecNSplat<T> for Vec2<T> {
            #[inline(always)]
            fn splat(value: T) -> Self {
                Self::from_inner(T::vec2_splat(value))
            }
        }
        impl<T: Element> VecNSplat<T> for Vec3<T> {
            #[inline(always)]
            fn splat(value: T) -> Self {
                Self::from_inner(T::vec3_splat(value))
            }
        }
        impl<T: Element> VecNSplat<T> for Vec4<T> {
            #[inline(always)]
            fn splat(value: T) -> Self {
                Self::from_inner(T::vec4_splat(value))
            }
        }
    }
    pub mod std_impl {
        use super::*;
        mod default {
            use super::*;
            pub trait ElementVecDefault: ElementVecInner {
                fn default_vec2() -> Self::InnerVec2;
                fn default_vec3() -> Self::InnerVec3;
                fn default_vec4() -> Self::InnerVec4;
            }
            impl<T: Element> Default for Vec2<T> {
                fn default() -> Self {
                    Self { inner: T::default_vec2() }
                }
            }
            impl<T: Element> Default for Vec3<T> {
                fn default() -> Self {
                    Self { inner: T::default_vec3() }
                }
            }
            impl<T: Element> Default for Vec4<T> {
                fn default() -> Self {
                    Self { inner: T::default_vec4() }
                }
            }
        }
        mod display {
            use std::fmt::{self, Display, Formatter};
            use super::*;
            impl<T: Element> Display for Vec2<T> {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    f.write_fmt(format_args!("({0}, {1})", self.x(), self.y()))
                }
            }
            impl<T: Element> Display for Vec3<T> {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    f.write_fmt(
                        format_args!("({0}, {1}, {2})", self.x(), self.y(), self.z()),
                    )
                }
            }
            impl<T: Element> Display for Vec4<T> {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    f.write_fmt(
                        format_args!(
                            "({0}, {1}, {2}, {3})",
                            self.x(),
                            self.y(),
                            self.z(),
                            self.w(),
                        ),
                    )
                }
            }
        }
        pub use default::*;
        #[allow(unused_imports)]
        pub use display::*;
    }
    pub mod swizzle {
        use super::*;
        mod get {
            use super::*;
            pub trait ElementVecGet<const N: usize>: Sized + ElementVecInner
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_get(
                    vec: InnerVec<N, Self>,
                    index: usize,
                ) -> Result<Self, &'static str>;
                fn vec_get2(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 2],
                ) -> Result<Self::InnerVec2, &'static str>;
                fn vec_get3(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 3],
                ) -> Result<Self::InnerVec3, &'static str>;
                fn vec_get4(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 4],
                ) -> Result<Self::InnerVec4, &'static str>;
                unsafe fn vec_get_unchecked(
                    vec: InnerVec<N, Self>,
                    index: usize,
                ) -> Self;
                unsafe fn vec_get2_unchecked(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 2],
                ) -> Self::InnerVec2;
                unsafe fn vec_get3_unchecked(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 3],
                ) -> Self::InnerVec3;
                unsafe fn vec_get4_unchecked(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 4],
                ) -> Self::InnerVec4;
            }
            impl<const N: usize, T: Element> VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                pub fn get(self, index: usize) -> Result<T, &'static str> {
                    MaybeVecNum::<N>::get(self, index)
                }
                pub fn get2(
                    self,
                    indicies: [usize; 2],
                ) -> Result<Vec2<T>, &'static str> {
                    MaybeVecNum::<N>::get2(self, indicies)
                }
                pub fn get3(
                    self,
                    indicies: [usize; 3],
                ) -> Result<Vec3<T>, &'static str> {
                    MaybeVecNum::<N>::get3(self, indicies)
                }
                pub fn get4(
                    self,
                    indicies: [usize; 4],
                ) -> Result<Vec4<T>, &'static str> {
                    MaybeVecNum::<N>::get4(self, indicies)
                }
                pub unsafe fn get_unchecked(self, index: usize) -> T {
                    MaybeVecNum::<N>::get_unchecked(self, index)
                }
                pub unsafe fn get2_unchecked(self, indicies: [usize; 2]) -> Vec2<T> {
                    MaybeVecNum::<N>::get2_unchecked(self, indicies)
                }
                pub unsafe fn get3_unchecked(self, indicies: [usize; 3]) -> Vec3<T> {
                    MaybeVecNum::<N>::get3_unchecked(self, indicies)
                }
                pub unsafe fn get4_unchecked(self, indicies: [usize; 4]) -> Vec4<T> {
                    MaybeVecNum::<N>::get4_unchecked(self, indicies)
                }
            }
            const _: &'static str = "pub(super) trait VecNumGet < const N : usize > where MaybeVecNum < N > :\nVecNum < N >\n{\n    fn get < T : Element > (vec : InnerVec < N, T > , index : usize) -> Result\n    < T, & \'static str > ; fn get2 < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 2],) -> Result < Vec2 < T >\n    , & \'static str > ; fn get3 < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 3],) -> Result < Vec3 < T >\n    , & \'static str > ; fn get4 < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 4],) -> Result < InnerVec <\n    T > , & \'static str > ; unsafe fn get_unchecked < T : Element >\n    (vec : InnerVec < N, T > , index : usize) -> T; unsafe fn get2_unchecked <\n    T : Element > (vec : InnerVec < N, T > , indicies : [usize; 2],) ->\n    InnerVec < 2, T > ; unsafe fn get3_unchecked < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 3],) -> InnerVec < 3, T > ;\n    unsafe fn get4_unchecked < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 4],) -> InnerVec < 4, T > ;\n} impl VecNumGet < 2 > for MaybeVecNum < 2 >\n{\n    fn get < T : Element > (vec : InnerVec < N, T > , index : usize) -> Result\n    < T, & \'static str > { T :: vec_get < > (vec, index) } fn get2 < T :\n    Element > (vec : InnerVec < N, T > , indicies : [usize; 2],) -> Result <\n    Vec2 < T > , & \'static str > { T :: vec_get2 < > (vec, indicies) } fn get3\n    < T : Element > (vec : InnerVec < N, T > , indicies : [usize; 3],) ->\n    Result < Vec3 < T > , & \'static str >\n    { T :: vec_get3 < > (vec, indicies) } fn get4 < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 4],) -> Result < InnerVec <\n    T > , & \'static str > { T :: vec_get4 < > (vec, indicies) } unsafe fn\n    get_unchecked < T : Element > (vec : InnerVec < N, T > , index : usize) ->\n    T { T :: vec_get_unchecked < > (vec, index) } unsafe fn get2_unchecked < T\n    : Element > (vec : InnerVec < N, T > , indicies : [usize; 2],) -> InnerVec\n    < 2, T > { T :: vec_get2_unchecked < > (vec, indicies) } unsafe fn\n    get3_unchecked < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 3],) -> InnerVec < 3, T >\n    { T :: vec_get3_unchecked < > (vec, indicies) } unsafe fn get4_unchecked <\n    T : Element > (vec : InnerVec < N, T > , indicies : [usize; 4],) ->\n    InnerVec < 4, T > { T :: vec_get4_unchecked < > (vec, indicies) }\n} impl VecNumGet < 3 > for MaybeVecNum < 3 >\n{\n    fn get < T : Element > (vec : InnerVec < N, T > , index : usize) -> Result\n    < T, & \'static str > { T :: vec_get < > (vec, index) } fn get2 < T :\n    Element > (vec : InnerVec < N, T > , indicies : [usize; 2],) -> Result <\n    Vec2 < T > , & \'static str > { T :: vec_get2 < > (vec, indicies) } fn get3\n    < T : Element > (vec : InnerVec < N, T > , indicies : [usize; 3],) ->\n    Result < Vec3 < T > , & \'static str >\n    { T :: vec_get3 < > (vec, indicies) } fn get4 < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 4],) -> Result < InnerVec <\n    T > , & \'static str > { T :: vec_get4 < > (vec, indicies) } unsafe fn\n    get_unchecked < T : Element > (vec : InnerVec < N, T > , index : usize) ->\n    T { T :: vec_get_unchecked < > (vec, index) } unsafe fn get2_unchecked < T\n    : Element > (vec : InnerVec < N, T > , indicies : [usize; 2],) -> InnerVec\n    < 2, T > { T :: vec_get2_unchecked < > (vec, indicies) } unsafe fn\n    get3_unchecked < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 3],) -> InnerVec < 3, T >\n    { T :: vec_get3_unchecked < > (vec, indicies) } unsafe fn get4_unchecked <\n    T : Element > (vec : InnerVec < N, T > , indicies : [usize; 4],) ->\n    InnerVec < 4, T > { T :: vec_get4_unchecked < > (vec, indicies) }\n} impl VecNumGet < 4 > for MaybeVecNum < 4 >\n{\n    fn get < T : Element > (vec : InnerVec < N, T > , index : usize) -> Result\n    < T, & \'static str > { T :: vec_get < > (vec, index) } fn get2 < T :\n    Element > (vec : InnerVec < N, T > , indicies : [usize; 2],) -> Result <\n    Vec2 < T > , & \'static str > { T :: vec_get2 < > (vec, indicies) } fn get3\n    < T : Element > (vec : InnerVec < N, T > , indicies : [usize; 3],) ->\n    Result < Vec3 < T > , & \'static str >\n    { T :: vec_get3 < > (vec, indicies) } fn get4 < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 4],) -> Result < InnerVec <\n    T > , & \'static str > { T :: vec_get4 < > (vec, indicies) } unsafe fn\n    get_unchecked < T : Element > (vec : InnerVec < N, T > , index : usize) ->\n    T { T :: vec_get_unchecked < > (vec, index) } unsafe fn get2_unchecked < T\n    : Element > (vec : InnerVec < N, T > , indicies : [usize; 2],) -> InnerVec\n    < 2, T > { T :: vec_get2_unchecked < > (vec, indicies) } unsafe fn\n    get3_unchecked < T : Element >\n    (vec : InnerVec < N, T > , indicies : [usize; 3],) -> InnerVec < 3, T >\n    { T :: vec_get3_unchecked < > (vec, indicies) } unsafe fn get4_unchecked <\n    T : Element > (vec : InnerVec < N, T > , indicies : [usize; 4],) ->\n    InnerVec < 4, T > { T :: vec_get4_unchecked < > (vec, indicies) }\n} const _ : () = {};";
        }
        mod get_mut {
            use std::mem::transmute;
            use super::*;
            impl<const N: usize, T: Element> VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                pub fn get_mut(&mut self, v0: usize) -> Result<&mut T, &'static str> {
                    if v0 + 1 > N {
                        Err("v0 outside of vec bounds")
                    } else {
                        Ok(unsafe { self.get_unchecked_mut(v0) })
                    }
                }
                pub fn get_mut2(
                    &mut self,
                    v0: usize,
                ) -> Result<&mut Vec2<T>, &'static str> {
                    if v0 + 2 > N {
                        Err("v0 outside of vec bounds")
                    } else {
                        Ok(unsafe { self.get_unchecked_mut2(v0) })
                    }
                }
                pub fn get_mut3(
                    &mut self,
                    v0: usize,
                ) -> Result<&mut Vec3<T>, &'static str> {
                    if v0 + 3 > N {
                        Err("v0 outside of vec bounds")
                    } else {
                        Ok(unsafe { self.get_unchecked_mut3(v0) })
                    }
                }
                pub fn get_mut4(
                    &mut self,
                    v0: usize,
                ) -> Result<&mut Vec4<T>, &'static str> {
                    if v0 + 4 > N {
                        Err("v0 outside of vec bounds")
                    } else {
                        Ok(unsafe { self.get_unchecked_mut4(v0) })
                    }
                }
                pub fn get_mut_1_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> Result<(&mut T, &mut T), &'static str> {
                    if v0 + 1 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 1 > N {
                        Err("v1 outside of vec bounds")
                    } else if v1 == v0 {
                        Err("v1 conflicts with v0")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                            )
                        })
                    }
                }
                pub fn get_mut_2_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> Result<(&mut Vec2<T>, &mut T), &'static str> {
                    if v0 + 2 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 1 > N {
                        Err("v1 outside of vec bounds")
                    } else if v1 == v0 || v1 == v0 + 1 {
                        Err("v1 conflicts with v0")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                            )
                        })
                    }
                }
                pub fn get_mut_3_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> Result<(&mut Vec3<T>, &mut T), &'static str> {
                    if v0 + 3 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 1 > N {
                        Err("v1 outside of vec bounds")
                    } else if v1 == v0 || v1 == v0 + 1 || v1 == v0 + 2 {
                        Err("v1 conflicts with v0")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                            )
                        })
                    }
                }
                pub fn get_mut_1_2(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> Result<(&mut T, &mut Vec2<T>), &'static str> {
                    if v0 + 1 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 2 > N {
                        Err("v1 outside of vec bounds")
                    } else if v1 == v0 || v1 + 1 == v0 {
                        Err("v1 conflicts with v0")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                            )
                        })
                    }
                }
                pub fn get_mut_2_2(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> Result<(&mut Vec2<T>, &mut Vec2<T>), &'static str> {
                    if v0 + 2 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 2 > N {
                        Err("v1 outside of vec bounds")
                    } else if v1 == v0 || v1 + 1 == v0 || v1 == v0 + 1 {
                        Err("v1 conflicts with v0")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                            )
                        })
                    }
                }
                pub fn get_mut_1_3(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> Result<(&mut T, &mut Vec3<T>), &'static str> {
                    if v0 + 1 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 3 > N {
                        Err("v1 outside of vec bounds")
                    } else if v1 == v0 || v1 + 1 == v0 || v1 + 2 == v0 {
                        Err("v1 conflicts with v0")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                            )
                        })
                    }
                }
                pub fn get_mut_1_1_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                ) -> Result<(&mut T, &mut T, &mut T), &'static str> {
                    if v0 + 1 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 1 > N {
                        Err("v1 outside of vec bounds")
                    } else if v2 + 1 > N {
                        Err("v2 outside of vec bounds")
                    } else if v1 == v0 {
                        Err("v1 conflicts with v0")
                    } else if v2 == v0 {
                        Err("v2 conflicts with v0")
                    } else if v2 == v1 {
                        Err("v2 conflicts with v1")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                                transmute(self.get_unchecked_mut(v2)),
                            )
                        })
                    }
                }
                pub fn get_mut_2_1_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                ) -> Result<(&mut Vec2<T>, &mut T, &mut T), &'static str> {
                    if v0 + 2 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 1 > N {
                        Err("v1 outside of vec bounds")
                    } else if v2 + 1 > N {
                        Err("v2 outside of vec bounds")
                    } else if v1 == v0 || v1 == v0 + 1 {
                        Err("v1 conflicts with v0")
                    } else if v2 == v0 || v2 == v0 + 1 {
                        Err("v2 conflicts with v0")
                    } else if v2 == v1 {
                        Err("v2 conflicts with v1")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                                transmute(self.get_unchecked_mut(v2)),
                            )
                        })
                    }
                }
                pub fn get_mut_1_2_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                ) -> Result<(&mut T, &mut Vec2<T>, &mut T), &'static str> {
                    if v0 + 1 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 2 > N {
                        Err("v1 outside of vec bounds")
                    } else if v2 + 1 > N {
                        Err("v2 outside of vec bounds")
                    } else if v1 == v0 || v1 + 1 == v0 {
                        Err("v1 conflicts with v0")
                    } else if v2 == v0 {
                        Err("v2 conflicts with v0")
                    } else if v2 == v1 || v2 == v1 + 1 {
                        Err("v2 conflicts with v1")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                                transmute(self.get_unchecked_mut(v2)),
                            )
                        })
                    }
                }
                pub fn get_mut_1_1_2(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                ) -> Result<(&mut T, &mut T, &mut Vec2<T>), &'static str> {
                    if v0 + 1 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 1 > N {
                        Err("v1 outside of vec bounds")
                    } else if v2 + 2 > N {
                        Err("v2 outside of vec bounds")
                    } else if v1 == v0 {
                        Err("v1 conflicts with v0")
                    } else if v2 == v0 || v2 + 1 == v0 {
                        Err("v2 conflicts with v0")
                    } else if v2 == v1 || v2 + 1 == v1 {
                        Err("v2 conflicts with v1")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                                transmute(self.get_unchecked_mut(v2)),
                            )
                        })
                    }
                }
                pub fn get_mut_1_1_1_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                    v3: usize,
                ) -> Result<(&mut T, &mut T, &mut T, &mut T), &'static str> {
                    if v0 + 1 > N {
                        Err("v0 outside of vec bounds")
                    } else if v1 + 1 > N {
                        Err("v1 outside of vec bounds")
                    } else if v2 + 1 > N {
                        Err("v2 outside of vec bounds")
                    } else if v3 + 1 > N {
                        Err("v3 outside of vec bounds")
                    } else if v1 == v0 {
                        Err("v1 conflicts with v0")
                    } else if v2 == v0 {
                        Err("v2 conflicts with v0")
                    } else if v3 == v0 {
                        Err("v3 conflicts with v0")
                    } else if v2 == v1 {
                        Err("v2 conflicts with v1")
                    } else if v3 == v1 {
                        Err("v3 conflicts with v1")
                    } else if v3 == v2 {
                        Err("v3 conflicts with v2")
                    } else {
                        Ok(unsafe {
                            (
                                transmute(self.get_unchecked_mut(v0)),
                                transmute(self.get_unchecked_mut(v1)),
                                transmute(self.get_unchecked_mut(v2)),
                                transmute(self.get_unchecked_mut(v3)),
                            )
                        })
                    }
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut(&mut self, v0: usize) -> &mut T {
                    self.as_array_mut().get_unchecked_mut(v0)
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut2(&mut self, v0: usize) -> &mut Vec2<T> {
                    transmute(self.get_unchecked_mut(v0))
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut3(&mut self, v0: usize) -> &mut Vec3<T> {
                    transmute(self.get_unchecked_mut(v0))
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut4(&mut self, v0: usize) -> &mut Vec4<T> {
                    transmute(self.get_unchecked_mut(v0))
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_1_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> (&mut T, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_2_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> (&mut Vec2<T>, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_3_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> (&mut Vec3<T>, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_1_2(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> (&mut T, &mut Vec2<T>) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_2_2(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> (&mut Vec2<T>, &mut Vec2<T>) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_1_3(
                    &mut self,
                    v0: usize,
                    v1: usize,
                ) -> (&mut T, &mut Vec3<T>) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_1_1_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                ) -> (&mut T, &mut T, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                        transmute(self.get_unchecked_mut(v2)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_2_1_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                ) -> (&mut Vec2<T>, &mut T, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                        transmute(self.get_unchecked_mut(v2)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_1_2_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                ) -> (&mut T, &mut Vec2<T>, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                        transmute(self.get_unchecked_mut(v2)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_1_1_2(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                ) -> (&mut T, &mut T, &mut Vec2<T>) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                        transmute(self.get_unchecked_mut(v2)),
                    )
                }
                #[inline(always)]
                pub unsafe fn get_unchecked_mut_1_1_1_1(
                    &mut self,
                    v0: usize,
                    v1: usize,
                    v2: usize,
                    v3: usize,
                ) -> (&mut T, &mut T, &mut T, &mut T) {
                    (
                        transmute(self.get_unchecked_mut(v0)),
                        transmute(self.get_unchecked_mut(v1)),
                        transmute(self.get_unchecked_mut(v2)),
                        transmute(self.get_unchecked_mut(v3)),
                    )
                }
            }
        }
        mod set {
            use super::*;
            impl<const N: usize, T: Element> VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                pub fn set(&mut self, x: usize, value: T) -> Result<(), &'static str> {
                    if x >= N {
                        Err("x outside of vec bounds")
                    } else {
                        unsafe {
                            self.set_unchecked(x, value);
                        }
                        Ok(())
                    }
                }
                pub fn set2(
                    &mut self,
                    x: usize,
                    y: usize,
                    value: Vec2<T>,
                ) -> Result<(), &'static str> {
                    if x >= N {
                        Err("x outside of vec bounds")
                    } else if y >= N {
                        Err("y outside of vec bounds")
                    } else if y == x {
                        Err("y conflicts with x")
                    } else {
                        unsafe {
                            self.set2_unchecked(x, y, value);
                        }
                        Ok(())
                    }
                }
                pub fn set3(
                    &mut self,
                    x: usize,
                    y: usize,
                    z: usize,
                    value: Vec3<T>,
                ) -> Result<(), &'static str> {
                    if x >= N {
                        Err("x outside of vec bounds")
                    } else if y >= N {
                        Err("y outside of vec bounds")
                    } else if z >= N {
                        Err("z outside of vec bounds")
                    } else if y == x {
                        Err("y conflicts with x")
                    } else if z == x {
                        Err("z conflicts with x")
                    } else if z == y {
                        Err("z conflicts with y")
                    } else {
                        unsafe {
                            self.set3_unchecked(x, y, z, value);
                        }
                        Ok(())
                    }
                }
                pub fn set4(
                    &mut self,
                    x: usize,
                    y: usize,
                    z: usize,
                    w: usize,
                    value: Vec4<T>,
                ) -> Result<(), &'static str> {
                    if x >= N {
                        Err("x outside of vec bounds")
                    } else if y >= N {
                        Err("y outside of vec bounds")
                    } else if z >= N {
                        Err("z outside of vec bounds")
                    } else if w >= N {
                        Err("w outside of vec bounds")
                    } else if y == x {
                        Err("y conflicts with x")
                    } else if z == x {
                        Err("z conflicts with x")
                    } else if w == x {
                        Err("w conflicts with x")
                    } else if z == y {
                        Err("z conflicts with y")
                    } else if w == y {
                        Err("w conflicts with y")
                    } else if w == z {
                        Err("w conflicts with z")
                    } else {
                        unsafe {
                            self.set4_unchecked(x, y, z, w, value);
                        }
                        Ok(())
                    }
                }
                #[inline(always)]
                pub unsafe fn set_unchecked(&mut self, x: usize, value: T) {
                    *self.get_unchecked_mut(x) = value;
                }
                #[inline(always)]
                pub unsafe fn set2_unchecked(
                    &mut self,
                    x: usize,
                    y: usize,
                    value: Vec2<T>,
                ) {
                    *self.get_unchecked_mut(x) = value.x();
                    *self.get_unchecked_mut(y) = value.y();
                }
                #[inline(always)]
                pub unsafe fn set3_unchecked(
                    &mut self,
                    x: usize,
                    y: usize,
                    z: usize,
                    value: Vec3<T>,
                ) {
                    *self.get_unchecked_mut(x) = value.x();
                    *self.get_unchecked_mut(y) = value.y();
                    *self.get_unchecked_mut(z) = value.z();
                }
                #[inline(always)]
                pub unsafe fn set4_unchecked(
                    &mut self,
                    x: usize,
                    y: usize,
                    z: usize,
                    w: usize,
                    value: Vec4<T>,
                ) {
                    *self.get_unchecked_mut(x) = value.x();
                    *self.get_unchecked_mut(y) = value.y();
                    *self.get_unchecked_mut(z) = value.z();
                    *self.get_unchecked_mut(w) = value.w();
                }
            }
        }
        mod with {
            use super::*;
            pub trait ElementVecWith<const N: usize>: ElementVecInner
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn vec_with(
                    vec: InnerVec<N, Self>,
                    index: usize,
                    value: Self,
                ) -> Result<InnerVec<N, Self>, &'static str>;
                fn vec_with2(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 2],
                    value: Self::InnerVec2,
                ) -> Result<InnerVec<N, Self>, &'static str>;
                fn vec_with3(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 3],
                    value: Self::InnerVec3,
                ) -> Result<InnerVec<N, Self>, &'static str>;
                fn vec_with4(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 4],
                    value: Self::InnerVec4,
                ) -> Result<InnerVec<N, Self>, &'static str>;
                unsafe fn vec_with_unchecked(
                    vec: InnerVec<N, Self>,
                    index: usize,
                    value: Self,
                ) -> InnerVec<N, Self>;
                unsafe fn vec_with2_unchecked(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 2],
                    value: Self::InnerVec2,
                ) -> InnerVec<N, Self>;
                unsafe fn vec_with3_unchecked(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 3],
                    value: Self::InnerVec3,
                ) -> InnerVec<N, Self>;
                unsafe fn vec_with4_unchecked(
                    vec: InnerVec<N, Self>,
                    indicies: [usize; 4],
                    value: Self::InnerVec4,
                ) -> InnerVec<N, Self>;
            }
            impl<const N: usize, T: Element> VecN<N, T>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                pub fn with(self, index: usize, value: T) -> Result<Self, &'static str> {
                    MaybeVecNum::<N>::with
                }
                pub fn with2(
                    self,
                    indicies: [usize; 2],
                    value: Vec2<T>,
                ) -> Result<Self, &'static str> {}
                pub fn with3(
                    self,
                    indicies: [usize; 3],
                    value: Vec3<T>,
                ) -> Result<Self, &'static str> {}
                pub fn with4(
                    self,
                    indicies: [usize; 4],
                    value: Vec4<T>,
                ) -> Result<Self, &'static str> {}
                pub unsafe fn with_unchecked(self, index: usize, value: T) -> Self {}
                pub unsafe fn with2_unchecked(
                    self,
                    indicies: [usize; 2],
                    value: Vec2<T>,
                ) -> Self {}
                pub unsafe fn with3_unchecked(
                    self,
                    indicies: [usize; 3],
                    value: Vec3<T>,
                ) -> Self {}
                pub unsafe fn with4_unchecked(
                    self,
                    indicies: [usize; 4],
                    value: Vec4<T>,
                ) -> Self {}
            }
            const _: &'static str = "pub(super) trait VecNumWith < const N : usize > where MaybeVecNum < N > :\nVecNum < N >\n{\n    fn with < T : Element > (vec : VecN < N, T > , index : usize, value : T,)\n    -> Result < VecN < N, T > , & \'static str > ; fn with2 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 2], value : Vec2 < T > ,) ->\n    Result < VecN < N, T > , & \'static str > ; fn with3 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 3], value : Vec3 < T > ,) ->\n    Result < VecN < N, T > , & \'static str > ; fn with4 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 4], value : Vec4 < T > ,) ->\n    Result < VecN < N, T > , & \'static str > ; unsafe fn with_unchecked < T :\n    Element > (vec : VecN < N, T > , index : usize, value : T) -> Self; unsafe\n    fn with2_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 2], value : Vec2 < T > ,) ->\n    VecN < N, T > ; unsafe fn with3_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 3], value : Vec3 < T > ,) ->\n    VecN < N, T > ; unsafe fn with4_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 4], value : Vec4 < T > ,) ->\n    VecN < N, T > ;\n} impl VecNumWith < 2 > for MaybeVecNum < 2 >\n{\n    fn with < T : Element > (vec : VecN < N, T > , index : usize, value : T,)\n    -> Result < VecN < N, T > , & \'static str >\n    { T :: vec_with < > (vec, index, value) } fn with2 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 2], value : Vec2 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with2 < > (vec, indicies, value) } fn with3 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 3], value : Vec3 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with3 < > (vec, indicies, value) } fn with4 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 4], value : Vec4 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with4 < > (vec, indicies, value) } unsafe fn with_unchecked < T\n    : Element > (vec : VecN < N, T > , index : usize, value : T) -> Self\n    { T :: vec_with_unchecked < > (vec, index, value) } unsafe fn\n    with2_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 2], value : Vec2 < T > ,) ->\n    VecN < N, T > { T :: vec_with2_unchecked < > (vec, indicies, value) }\n    unsafe fn with3_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 3], value : Vec3 < T > ,) ->\n    VecN < N, T > { T :: vec_with3_unchecked < > (vec, indicies, value) }\n    unsafe fn with4_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 4], value : Vec4 < T > ,) ->\n    VecN < N, T > { T :: vec_with4_unchecked < > (vec, indicies, value) }\n} impl VecNumWith < 3 > for MaybeVecNum < 3 >\n{\n    fn with < T : Element > (vec : VecN < N, T > , index : usize, value : T,)\n    -> Result < VecN < N, T > , & \'static str >\n    { T :: vec_with < > (vec, index, value) } fn with2 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 2], value : Vec2 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with2 < > (vec, indicies, value) } fn with3 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 3], value : Vec3 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with3 < > (vec, indicies, value) } fn with4 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 4], value : Vec4 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with4 < > (vec, indicies, value) } unsafe fn with_unchecked < T\n    : Element > (vec : VecN < N, T > , index : usize, value : T) -> Self\n    { T :: vec_with_unchecked < > (vec, index, value) } unsafe fn\n    with2_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 2], value : Vec2 < T > ,) ->\n    VecN < N, T > { T :: vec_with2_unchecked < > (vec, indicies, value) }\n    unsafe fn with3_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 3], value : Vec3 < T > ,) ->\n    VecN < N, T > { T :: vec_with3_unchecked < > (vec, indicies, value) }\n    unsafe fn with4_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 4], value : Vec4 < T > ,) ->\n    VecN < N, T > { T :: vec_with4_unchecked < > (vec, indicies, value) }\n} impl VecNumWith < 4 > for MaybeVecNum < 4 >\n{\n    fn with < T : Element > (vec : VecN < N, T > , index : usize, value : T,)\n    -> Result < VecN < N, T > , & \'static str >\n    { T :: vec_with < > (vec, index, value) } fn with2 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 2], value : Vec2 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with2 < > (vec, indicies, value) } fn with3 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 3], value : Vec3 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with3 < > (vec, indicies, value) } fn with4 < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 4], value : Vec4 < T > ,) ->\n    Result < VecN < N, T > , & \'static str >\n    { T :: vec_with4 < > (vec, indicies, value) } unsafe fn with_unchecked < T\n    : Element > (vec : VecN < N, T > , index : usize, value : T) -> Self\n    { T :: vec_with_unchecked < > (vec, index, value) } unsafe fn\n    with2_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 2], value : Vec2 < T > ,) ->\n    VecN < N, T > { T :: vec_with2_unchecked < > (vec, indicies, value) }\n    unsafe fn with3_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 3], value : Vec3 < T > ,) ->\n    VecN < N, T > { T :: vec_with3_unchecked < > (vec, indicies, value) }\n    unsafe fn with4_unchecked < T : Element >\n    (vec : VecN < N, T > , indicies : [usize; 4], value : Vec4 < T > ,) ->\n    VecN < N, T > { T :: vec_with4_unchecked < > (vec, indicies, value) }\n} const _ : () = {};";
        }
        pub use get::*;
        pub use with::*;
        pub trait ElementVecSwizzle: ElementVecGet<
                2,
            > + ElementVecGet<
                3,
            > + ElementVecGet<
                4,
            > + ElementVecWith<2> + ElementVecWith<3> + ElementVecWith<4> {}
        const _: &'static str = "pub(super) trait VecNumSwizzle < const N : usize > : VecNumGet < N > +\nVecNumWith < N > where MaybeVecNum < N > : VecNum < N > {} impl VecNumSwizzle\n< 2 > for MaybeVecNum < 2 > {} impl VecNumSwizzle < 3 > for MaybeVecNum < 3 >\n{} impl VecNumSwizzle < 4 > for MaybeVecNum < 4 > {} const _ : () = {};";
    }
    #[repr(transparent)]
    pub struct VecN<const N: usize, T: Element>
    where
        MaybeVecNum<N>: VecNum<N>,
    {
        inner: InnerVec<N, T>,
    }
    #[automatically_derived]
    impl<const N: usize, T: ::core::fmt::Debug + Element> ::core::fmt::Debug
    for VecN<N, T>
    where
        MaybeVecNum<N>: VecNum<N>,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "VecN",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl<const N: usize, T: ::core::clone::Clone + Element> ::core::clone::Clone
    for VecN<N, T>
    where
        MaybeVecNum<N>: VecNum<N>,
    {
        #[inline]
        fn clone(&self) -> VecN<N, T> {
            VecN {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl<const N: usize, T: ::core::marker::Copy + Element> ::core::marker::Copy
    for VecN<N, T>
    where
        MaybeVecNum<N>: VecNum<N>,
    {}
    #[automatically_derived]
    impl<const N: usize, T: Element> ::core::marker::StructuralPartialEq for VecN<N, T>
    where
        MaybeVecNum<N>: VecNum<N>,
    {}
    #[automatically_derived]
    impl<const N: usize, T: ::core::cmp::PartialEq + Element> ::core::cmp::PartialEq
    for VecN<N, T>
    where
        MaybeVecNum<N>: VecNum<N>,
    {
        #[inline]
        fn eq(&self, other: &VecN<N, T>) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl<const N: usize, T: ::core::cmp::PartialOrd + Element> ::core::cmp::PartialOrd
    for VecN<N, T>
    where
        MaybeVecNum<N>: VecNum<N>,
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &VecN<N, T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    pub type Vec2<T = f32> = VecN<2, T>;
    pub type Vec3<T = f32> = VecN<3, T>;
    pub type Vec4<T = f32> = VecN<4, T>;
    pub trait ElementVec: ElementVecInner + std_impl::ElementVecDefault + const_swizzle::ElementVecConstSwizzle + swizzle::ElementVecSwizzle + splat::ElementVecSplat {}
    pub struct MaybeVecNum<const VALUE: usize>;
    #[automatically_derived]
    impl<const VALUE: usize> ::core::fmt::Debug for MaybeVecNum<VALUE> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "MaybeVecNum")
        }
    }
    #[automatically_derived]
    impl<const VALUE: usize> ::core::clone::Clone for MaybeVecNum<VALUE> {
        #[inline]
        fn clone(&self) -> MaybeVecNum<VALUE> {
            *self
        }
    }
    #[automatically_derived]
    impl<const VALUE: usize> ::core::marker::Copy for MaybeVecNum<VALUE> {}
    #[automatically_derived]
    impl<const VALUE: usize> ::core::marker::StructuralPartialEq for MaybeVecNum<VALUE> {}
    #[automatically_derived]
    impl<const VALUE: usize> ::core::cmp::PartialEq for MaybeVecNum<VALUE> {
        #[inline]
        fn eq(&self, other: &MaybeVecNum<VALUE>) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl<const VALUE: usize> ::core::cmp::Eq for MaybeVecNum<VALUE> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl<const VALUE: usize> ::core::cmp::PartialOrd for MaybeVecNum<VALUE> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MaybeVecNum<VALUE>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    #[automatically_derived]
    impl<const VALUE: usize> ::core::cmp::Ord for MaybeVecNum<VALUE> {
        #[inline]
        fn cmp(&self, other: &MaybeVecNum<VALUE>) -> ::core::cmp::Ordering {
            ::core::cmp::Ordering::Equal
        }
    }
    #[automatically_derived]
    impl<const VALUE: usize> ::core::hash::Hash for MaybeVecNum<VALUE> {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    #[automatically_derived]
    impl<const VALUE: usize> ::core::default::Default for MaybeVecNum<VALUE> {
        #[inline]
        fn default() -> MaybeVecNum<VALUE> {
            MaybeVecNum {}
        }
    }
    const _: &'static str = "pub trait VecNum < const N : usize > : VecNumInner < N > + swizzle ::\nVecNumSwizzle < N > + const_swizzle :: VecNumConstSwizzle < N > where\nMaybeVecNum < N > : VecNum < N > {} impl VecNum < 2 > for MaybeVecNum < 2 > {}\nimpl VecNum < 3 > for MaybeVecNum < 3 > {} impl VecNum < 4 > for MaybeVecNum <\n4 > {} const _ : () = {};";
}
