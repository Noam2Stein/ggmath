use gmath::*;

pub fn main() {
    let vec = vec4((1, 2, 3, 4)).map(|x| x > 2).count();

    println!("{}", vec);
}