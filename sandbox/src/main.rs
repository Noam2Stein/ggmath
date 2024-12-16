#[allow(unused_imports)]
use ggmath::{matrix::*, rectangle::*, scalar::*, vector::*, *};

fn main() {
    let fun = mat3x2!(1, 2, 3; 4, 5, 6);
    println!("{}", mat3!(fun; 7, 8, 9))
}
