use gomath::vec::*;

fn main() {
    let mut vec0 = vec3((6, 242, 5));
    let vec0_ref = vec0.x_z_mut();
    *vec0_ref.0 = 1;
    *vec0_ref.1 = 2;
    let vec = vec4((vec0.xz(), 3, 4));
    println!("{vec}");
}
