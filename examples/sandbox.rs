use gomath::vec::*;

fn main() {
    let mut vec0 = vec3((6, 242, 5));
    vec0.set_xz(vec2((1, 2)));
    let vec = vec4((vec0.xz(), 3, 4));
    println!("{vec}");
}
