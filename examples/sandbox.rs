use ggmath::primitive_aliases::f32::*;

fn main() {
    let vec2 = fvec2((1.0, 3.0));
    let vec3 = fvec3((vec2, 2.0));

    println!("{}", vec3.xzy())
}
