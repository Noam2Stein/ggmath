use std::fmt::{self, Display, Formatter};

use ggmath::{scalar::*, vector::inner::inner_vecs};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct Meters(f32);
impl Display for Meters {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        write!(f, "m")
    }
}

inner_vecs!(Meters(4));

impl Scalar for Meters {}

scalar_aliases!(mod meters for Meters(M));
use meters::*;

fn main() {
    let a = mvec2((Meters(2.0), Meters(1.0)));
    let b = mvec3((a, Meters(4.0)));
    let c = mvec4((b.yx(), Meters(3.0), b.z()));

    println!("{c}");
}
