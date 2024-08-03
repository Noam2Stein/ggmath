use gmath::*;

pub fn main() {
    let vec = vec4((1, 2, 3, 4));
    let vec2 = vec4((4, 3, 2, 1));

    println!("{}", vec.map_with(vec2, |x, x2| x2 > x));
}