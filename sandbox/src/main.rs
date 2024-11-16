use ggmath::{rectangle::*, vector::into_vec::*};

fn main() {
    let d = Rect2C::from_min_max(vec2((1, -1)).max(vec2((0, 2))), vec2((3, 4)));

    println!("{d}");
}
