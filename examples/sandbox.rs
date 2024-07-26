use gmath::*;

pub fn main() {
    let vec = vec2(3, 4).xyx_a().with_x(10);
    println!("{}", vec);
}