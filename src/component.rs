use std::fmt::{Debug, Display};

pub trait Component:
Debug +
Display +
Clone +
Copy +
PartialEq +
Send +
Sync +
Default
{

}

impl Component for bool {}
impl Component for u8 {}
impl Component for u16 {}
impl Component for u32 {}
impl Component for u64 {}
impl Component for u128 {}
impl Component for i8 {}
impl Component for i16 {}
impl Component for i32 {}
impl Component for i64 {}
impl Component for i128 {}
impl Component for f32 {}
impl Component for f64 {}