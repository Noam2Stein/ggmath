use gmath::*;

pub fn main() {
    let vec_0 = vec2((2, 3));
    let mut vec = vec4((1, vec_0, 4));
    println!("{vec:?}");

    let vec_mut = vec.zw_mut();
    vec_mut.x = 5;

    println!("{vec:?}");
}