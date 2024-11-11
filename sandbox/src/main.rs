use ggmath::{matrix::*, primitive_aliases::f32::*};

fn main() {
    //let a = fvec2((1.0, 0.5)) * 2.0;
    //let b = fvec3((a, 4.0));
    //let c = fvec4((b.yx(), 3.0, b.z()));

    let d = row_major::Matrix::from_columns([fvec2((0.0, 1.0)), fvec2((2.0, 3.0))]);

    println!("{d}");
}
