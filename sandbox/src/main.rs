use ggmath::{rectangle::*, vector::*};

fn main() {
    let d = Rect2C::from_min_max(vec2!(1, 5).clamp(vec2!(0, 0), vec2!(3, 2)), vec2!(3, 4));

    let funny = vec4!(d.min(), d.max());
    if funny == vec4!(1, 2, 3, 4) {
        println!("Squidward")
    } else {
        println!("Not Squidward")
    }
}
