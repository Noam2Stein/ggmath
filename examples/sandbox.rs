use gmath::*;

pub fn main() {
    let vec_0 = vec2((2, 3));
    let vec = vec4((1, vec_0, 4));
    println!("{vec:?}");
}