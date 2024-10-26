use ggmath::{scalar::aliases::f32::*, vec::*};

fn main() {
    println!("{}", fvecn((vec2p((1.0, 4.0)).with_y(3.0), 2.0)).xzy())
}
