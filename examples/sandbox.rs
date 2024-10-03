use gomath::vec::*;

fn main() {
    let mut vec0 = vec3((6, 242, 5));
    let vec0_ref = vec0.x_z_mut();
    *vec0_ref.0 = 0;
    *vec0_ref.1 = 3;
    let vec = vec4((vec0.xz() + vec2((1, -1)), 3, 4));
    println!("{vec}");
}
