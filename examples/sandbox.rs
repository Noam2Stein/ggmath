use gmath::*;

pub fn main() {
    let mut vec = Vec4::new(1, 2, 3, 4).xywz();
    let (xy, zw) = vec.xy_zw_mut();
    println!("{xy} {zw}");

    //let vec = vec4((1, 2, 3, 4));
    //let vec2 = vec4((4, 3, 2, 1));

    //println!("{}", vec.map_with(vec2, |x, x2| x2 > x).to_num::<i32>());
}