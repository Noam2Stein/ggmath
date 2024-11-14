use ggmath::{matrix::*, primitive_aliases::f32::*, rectangle::Rectangle, vector::into_vec::vec2};

fn main() {
    //let a = fvec2((1.0, 0.5)) * 2.0;
    //let b = fvec3((a, 4.0));
    //let c = fvec4((b.yx(), 3.0, b.z()));

    let d = Rectangle::from_min_max(vec2((1, 2)), vec2((3, 4)));

    println!("{d}");
}
