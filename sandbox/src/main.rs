use ggmath::matrix::{builder::*, *};

fn main() {
    let t = mat3x2!(
        1, 2, 3;
        4, 5, 6;
    );

    let m = mat3c!(
        t;
        t.m21() + 1, 8, 9;
    );

    if m == mat3c!(1, 2, 3; 4, 5, 6; 7, 8, 9) {
        println!("Squidward")
    } else {
        println!("Not Squidward")
    }
}
