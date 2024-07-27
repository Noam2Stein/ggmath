use std::fmt::{Debug, Display};

pub trait Element:
Send +
Sync +
Debug +
Clone +
Copy +
PartialEq +
Default +
Display
{

}

impl Element for bool {}
impl Element for u8 {}
impl Element for u16 {}
impl Element for u32 {}
impl Element for u64 {}
impl Element for u128 {}
impl Element for i8 {}
impl Element for i16 {}
impl Element for i32 {}
impl Element for i64 {}
impl Element for i128 {}
impl Element for f32 {}
impl Element for f64 {}