use gmath::*;

pub fn main() {
    let mut t = Vec2::new(1, 2);
    t += t + Vec2::new(3, 4);
    print!("{t}");
    println!("{}", t * 2);

    //let vec = vec4((1, 2, 3, 4));
    //let vec2 = vec4((4, 3, 2, 1));

    //println!("{}", vec.map_with(vec2, |x, x2| x2 > x).to_num::<i32>());
}