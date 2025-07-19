use ggmath::*;

fn main() {
    let rect = Rect2M::from_min_max(vec2!(1, 1), vec2!(2, 2)).moved(vec2!(1, 1));

    println!("{rect}")
}
