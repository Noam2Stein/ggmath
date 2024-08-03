use gmath::*;

pub fn main() {
    let vec = vec4((1, 2, 3, 4)).count(|x| x > 2);

    println!("{}", vec);
}