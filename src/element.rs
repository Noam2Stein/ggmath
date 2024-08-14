use std::{fmt::{Debug, Display}, ops::*};

use gmath_macros::*;

pub trait Element:
Send +
Sync +
Debug +
Copy +
PartialEq +
PartialOrd +
Display +
Default +
{
    type Vec2Inner: Send + Sync + Debug + Copy + PartialEq + PartialOrd + Default;
    type Vec3Inner: Send + Sync + Debug + Copy + PartialEq + PartialOrd + Default;
    type Vec3AInner: Send + Sync + Debug + Copy + PartialEq + PartialOrd + Default;
    type Vec4Inner: Send + Sync + Debug + Copy + PartialEq + PartialOrd + Default;

    fn new_vec2(x: Self, y: Self) -> Self::Vec2Inner;
    fn new_vec3(x: Self, y: Self, z: Self) -> Self::Vec3Inner;
    fn new_vec3a(x: Self, y: Self, z: Self) -> Self::Vec3AInner;
    fn new_vec4(x: Self, y: Self, z: Self, w: Self) -> Self::Vec4Inner;

    fn splat_vec2(value: Self) -> Self::Vec2Inner;
    fn splat_vec3(value: Self) -> Self::Vec3Inner;
    fn splat_vec3a(value: Self) -> Self::Vec3AInner;
    fn splat_vec4(value: Self) -> Self::Vec4Inner;

    any_vec_any!(
        Self {
            fn #inner_ident(vec: #inner_self) -> #inner_value;
        }
    );
}

pub trait Num:
Element +
Add<Output = Self> +
Sub<Output = Self> +
Mul<Output = Self> +
Div<Output = Self> +
Rem<Output = Self> +
AddAssign +
SubAssign +
MulAssign +
DivAssign +
RemAssign +
{
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
    const MIN_POSITIVE: Self;

    fn from_num<T: Num>(value: T) -> Self;
    #[inline(always)]
    fn into_num<T: Num>(self) -> T {
        T::from_num(self)
    }
    fn into_u8(self) -> u8;
    fn into_u16(self) -> u16;
    fn into_u32(self) -> u32;
    fn into_u64(self) -> u64;
    fn into_u128(self) -> u128;
    fn into_usize(self) -> usize;
    fn into_i8(self) -> i8;
    fn into_i16(self) -> i16;
    fn into_i32(self) -> i32;
    fn into_i64(self) -> i64;
    fn into_i128(self) -> i128;
    fn into_isize(self) -> isize;
    fn into_f32(self) -> f32;
    fn into_f64(self) -> f64;
}

pub trait SignedNum:
Num +
Neg<Output = Self>
{
    const NEG_ONE: Self;
    const MAX_NEG: Self;

    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool;
}

pub trait UnsignedNum:
Num +
{

}

pub trait Int:
Num +
Eq +
Ord +
{

}

pub trait UnsignedInt:
Int +
UnsignedNum +
{
    
}

pub trait SignedInt:
Int +
SignedNum +
{
    
}

pub trait Float:
SignedNum +
{
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NEG_ZERO: Self;
}

macro_rules! impl_num {
    { $into_self:ident } => {
        const MIN: Self = Self::MIN;
        const MAX: Self = Self::MAX;

        #[inline(always)]
        fn from_num<T: Num>(value: T) -> Self {
            value.$into_self()
        }
        #[inline(always)]
        fn into_u8(self) -> u8 {
            self as u8
        }
        #[inline(always)]
        fn into_u16(self) -> u16 {
            self as u16
        }
        #[inline(always)]
        fn into_u32(self) -> u32 {
            self as u32
        }
        #[inline(always)]
        fn into_u64(self) -> u64 {
            self as u64
        }
        #[inline(always)]
        fn into_u128(self) -> u128 {
            self as u128
        }
        #[inline(always)]
        fn into_usize(self) -> usize {
            self as usize
        }
        #[inline(always)]
        fn into_i8(self) -> i8 {
            self as i8
        }
        #[inline(always)]
        fn into_i16(self) -> i16 {
            self as i16
        }
        #[inline(always)]
        fn into_i32(self) -> i32 {
            self as i32
        }
        #[inline(always)]
        fn into_i64(self) -> i64 {
            self as i64
        }
        #[inline(always)]
        fn into_i128(self) -> i128 {
            self as i128
        }
        #[inline(always)]
        fn into_isize(self) -> isize {
            self as isize
        }
        #[inline(always)]
        fn into_f32(self) -> f32 {
            self as f32
        }
        #[inline(always)]
        fn into_f64(self) -> f64 {
            self as f64
        }
    };
}
macro_rules! impl_num_for_int {
    {} => {
        const ZERO: Self = 0;
        const ONE: Self = 1;
        const MIN_POSITIVE: Self = 1;
    };
}
macro_rules! impl_num_for_float {
    {} => {
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
        const MIN_POSITIVE: Self = Self::MIN_POSITIVE;
    };
}
macro_rules! impl_signed_num {
    {} => {
        fn abs(self) -> Self {
            self.abs()
        }
        fn signum(self) -> Self {
            self.signum()
        }    
    };
}
macro_rules! impl_signed_num_for_int {
    {} => {
        const NEG_ONE: Self = -1;
        const MAX_NEG: Self = -1;

        fn is_positive(self) -> bool {
            self.is_positive()
        }
        fn is_negative(self) -> bool {
            self.is_negative()
        }
    };
}
macro_rules! impl_signed_num_for_float {
    {} => {
        const NEG_ONE: Self = -1.0;
        const MAX_NEG: Self = -Self::MIN_POSITIVE;

        fn is_positive(self) -> bool {
            self.is_sign_positive()
        }
        fn is_negative(self) -> bool {
            self.is_sign_negative()
        }
    };
}
macro_rules! impl_float {
    {} => {
        const NAN: Self = Self::NAN;
        const INFINITY: Self = Self::INFINITY;
        const NEG_INFINITY: Self = Self::NEG_INFINITY;
        const NEG_ZERO: Self = -0.0;   
    };
}

macro_rules! default_impl_element {
    {} => {
        type Vec2Inner = [Self; 2];
        type Vec3Inner = [Self; 3];
        type Vec3AInner = [Self; 4];
        type Vec4Inner = [Self; 4];

        #[inline(always)]
        fn new_vec2(x: Self, y: Self) -> Self::Vec2Inner {
            [x, y]
        }
        #[inline(always)]
        fn new_vec3(x: Self, y: Self, z: Self) -> Self::Vec3Inner {
            [x, y, z]
        }
        #[inline(always)]
        fn new_vec3a(x: Self, y: Self, z: Self) -> Self::Vec3AInner {
            [x, y, z, z]
        }
        #[inline(always)]
        fn new_vec4(x: Self, y: Self, z: Self, w: Self) -> Self::Vec4Inner {
            [x, y, z, w]
        }

        #[inline(always)]
        fn splat_vec2(value: Self) -> Self::Vec2Inner {
            [value; 2]
        }
        #[inline(always)]
        fn splat_vec3(value: Self) -> Self::Vec3Inner {
            [value; 3]
        }
        #[inline(always)]
        fn splat_vec3a(value: Self) -> Self::Vec3AInner {
            [value; 4]
        }
        #[inline(always)]
        fn splat_vec4(value: Self) -> Self::Vec4Inner {
            [value; 4]
        }

        any_vec_get!(
            Self {
                #[inline(always)]
                fn #inner_ident(vec: #inner_self) -> #inner_value {
                    let mut output = unsafe { std::mem::MaybeUninit::<#inner_value>::uninit().assume_init() };
                    unsafe {
                        #{
                            let src = &vec[#self_component] as *const _ as *const [Self; #len];
                            let dst = &mut #vec[value_component] as *mut _ as *mut [Self; #len];
                            *dst = *src;
                        }
                    }
                    output
                }
            }
        );
    };
}

impl Element for bool {
    default_impl_element! {}
}
impl Element for u8 {
    default_impl_element! {}
}
impl Element for u16 {
    default_impl_element! {}
}
impl Element for u32 {
    default_impl_element! {}
}
impl Element for u64 {
    default_impl_element! {}
}
impl Element for u128 {
    default_impl_element! {}
}
impl Element for usize {
    default_impl_element! {}
}
impl Element for i8 {
    default_impl_element! {}
}
impl Element for i16 {
    default_impl_element! {}
}
impl Element for i32 {
    default_impl_element! {}
}
impl Element for i64 {
    default_impl_element! {}
}
impl Element for i128 {
    default_impl_element! {}
}
impl Element for isize {
    default_impl_element! {}
}
impl Element for f32 {
    default_impl_element! {}
}
impl Element for f64 {
    default_impl_element! {}
}

impl Num for u8 {
    impl_num! { into_u8 }
    impl_num_for_int! {}
}
impl Num for u16 {
    impl_num! { into_u16 }
    impl_num_for_int! {}
}
impl Num for u32 {
    impl_num! { into_u32 }
    impl_num_for_int! {}
}
impl Num for u64 {
    impl_num! { into_u64 }
    impl_num_for_int! {}
}
impl Num for u128 {
    impl_num! { into_u128 }
    impl_num_for_int! {}
}
impl Num for usize {
    impl_num! { into_usize }
    impl_num_for_int! {}
}
impl Num for i8 {
    impl_num! { into_i8 }
    impl_num_for_int! {}
}
impl Num for i16 {
    impl_num! { into_i16 }
    impl_num_for_int! {}
}
impl Num for i32 {
    impl_num! { into_i32 }
    impl_num_for_int! {}
}
impl Num for i64 {
    impl_num! { into_i64 }
    impl_num_for_int! {}
}
impl Num for i128 {
    impl_num! { into_i128 }
    impl_num_for_int! {}
}
impl Num for isize {
    impl_num! { into_isize }
    impl_num_for_int! {}
}
impl Num for f32 {
    impl_num! { into_f32 }
    impl_num_for_float! {}
}
impl Num for f64 {
    impl_num! { into_f64 }
    impl_num_for_float! {}
}

impl SignedNum for i8 {
    impl_signed_num! {}
    impl_signed_num_for_int! {}
}
impl SignedNum for i16 {
    impl_signed_num! {}
    impl_signed_num_for_int! {}
}
impl SignedNum for i32 {
    impl_signed_num! {}
    impl_signed_num_for_int! {}
}
impl SignedNum for i64 {
    impl_signed_num! {}
    impl_signed_num_for_int! {}
}
impl SignedNum for i128 {
    impl_signed_num! {}
    impl_signed_num_for_int! {}
}
impl SignedNum for isize {
    impl_signed_num! {}
    impl_signed_num_for_int! {}
}
impl SignedNum for f32 {
    impl_signed_num! {}
    impl_signed_num_for_float! {}
}
impl SignedNum for f64 {
    impl_signed_num! {}
    impl_signed_num_for_float! {}
}

impl UnsignedNum for u8 {

}
impl UnsignedNum for u16 {

}
impl UnsignedNum for u32 {

}
impl UnsignedNum for u64 {

}
impl UnsignedNum for u128 {

}
impl UnsignedNum for usize {

}

impl Int for u8 {

}
impl Int for u16 {

}
impl Int for u32 {

}
impl Int for u64 {

}
impl Int for u128 {

}
impl Int for usize {

}
impl Int for i8 {

}
impl Int for i16 {

}
impl Int for i32 {

}
impl Int for i64 {

}
impl Int for i128 {

}
impl Int for isize {

}

impl UnsignedInt for u8 {
    
}
impl UnsignedInt for u16 {
    
}
impl UnsignedInt for u32 {
    
}
impl UnsignedInt for u64 {
    
}
impl UnsignedInt for u128 {
    
}
impl UnsignedInt for usize {
    
}

impl SignedInt for i8 {
    
}
impl SignedInt for i16 {
    
}
impl SignedInt for i32 {
    
}
impl SignedInt for i64 {
    
}
impl SignedInt for i128 {
    
}
impl SignedInt for isize {
    
}

impl Float for f32 {
    impl_float! {}
}
impl Float for f64 {
    impl_float! {}
}