use gomath::{element::num_traits::*, vec::*};

fn main() {
    let t = test(vec2((0.5, 1.2)), vec2((1, 1)));
    println!("{t}");
}

fn test(a: Vec2<impl NumElement>, b: Vec2<impl NumElement>) -> Vec2<impl NumElement> {
    a + b.map(|c| c.as_num())
}
