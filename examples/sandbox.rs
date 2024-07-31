use gmath::*;

pub fn main() {
    let vec = vec2(28, 41) % vec2(4, 5).yx();
    println!("{}", vec);
}