use ggmath::{
    scalar::{scalar_inner_vectors, Scalar},
    vector::{vec3, vector_aliases},
};

fn main() {
    println!(
        "{:?}",
        Position(vec3!(Meters(1.0), Meters(2.0), Meters(3.0)))
    )
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct Meters(f32);

scalar_inner_vectors!(Meters(4));

impl Scalar for Meters {}

vector_aliases!(pub mod aliases for Meters(M));
use aliases::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Position(MVec3);
