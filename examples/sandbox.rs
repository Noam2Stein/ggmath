use gomath::vec::*;

fn main() {
    let vec0 = vec3((1, 242, 2));
    let vec = vec4((vec0.xy(), 3, 4));
    println!("{vec}");
}
