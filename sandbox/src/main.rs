use derive_more::{
    Add, AddAssign, Display, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
#[allow(unused_imports)]
use ggmath::{matrix::*, num::*, rectangle::*, scalar::*, vector::*, *};

fn main() {
    println!("{}", vec3!(3.0, -2.0, -0.0).signum());
}

#[derive(
    Display,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    Clone,
    Copy,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
struct Fun(i32);

impl Num for Fun {}
