use ggmath::primitive_aliases::f32::*;

fn main() {
    let a = fvec2((1.0, 0.5)) * 2.0;
    let b = fvec3((a, 4.0));
    let c = fvec4((b.yx(), 3.0, b.z()));

    println!("{c}");
}
