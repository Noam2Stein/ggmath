use ggmath::vector::f32_aliases::FVec2;
#[allow(unused_imports)]
use ggmath::{matrix::*, rectangle::*, scalar::*, vector::*, *};

pub const VEC: FVec2 = FVec2::from_array([1.0, 2.0]);

fn main() {
    println!("{VEC}")
}
