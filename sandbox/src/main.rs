#[allow(unused_imports)]
use ggmath::{matrix::*, rectangle::*, scalar::*, vector::*, *};

fn main() {
    if vec4!(1.1, -2.4, 3.6, -4.2).abs().floor().csum() == 10.0 {
        println!("Squidward")
    } else {
        println!("Not Squidward")
    }
}
