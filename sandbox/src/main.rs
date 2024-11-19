use ggmath::{
    matrix::{builder::*, *},
    vector::{builder::*, *},
};

fn main() {
    let t = mat3x2!(
        1, 2, 3;
        4, 5, 6;
    );

    let m = mat3c!(
        t;
        t.get_row(0).unwrap().xy(), 9;
    );

    println!("{m}");

    let funny = vec4!(1, 2, 3, 4);
    if funny == vec4!(1, 2, 3, 4) {
        println!("Squidward")
    } else {
        println!("Not Squidward")
    }
}
