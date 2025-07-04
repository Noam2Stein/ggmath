use ggmath::{Rect2M, vec2};

fn main() {
    let rect = Rect2M::from_min_max(vec2!(1, 1), vec2!(3, 3)).moved_x(1);

    println!("{rect}")
}
