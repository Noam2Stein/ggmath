#![allow(unused_imports)]

use ggmath::{matrix::*, rectangle::*, scalar::*, vector::*, *};

fn main() {
    println!("{}", Rect2C::from_min_max(vec2!(1, 1), vec2!(3, 3)));
}
