#[allow(unused_imports)]
use ggmath::{matrix::*, rectangle::*, scalar::*, vector::*, *};

fn main() {
    if vec4!(1, -2, 3, -4).abs().csum() == 10 {
        println!("Squidward")
    } else {
        println!("Not Squidward")
    }
}
