use gomath::vec::{ops::*, *};

fn main() {
    let mut vec0 = vec3((6, 242, 5));
    let vec0_ref = vec0.x_z_mut();
    *vec0_ref.0 = 0;
    *vec0_ref.1 = 1;
    let mut vec0 = vec0.xz();
    vec0 += splat2(vec3((-1, -2, vec3((1, 2, 2)).product())).sum());
    let vec = vec4((vec0, 3, 4));
    println!("{vec}");
}
