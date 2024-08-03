use gmath::*;

pub fn main() {
    let vec = vec4((true, false, !vec3((false || vec2((true, false)).all(), false, true)).xy())).with_y(true);

    println!("{}", vec.all());
}