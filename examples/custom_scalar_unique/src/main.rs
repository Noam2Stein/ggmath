use ggmath::{
    scalar::{scalar_inner_vectors, Scalar},
    vector::vec2,
};

fn main() {
    println!("{:?}", vec2!(u256(1, 2), u256(3, 4)))
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct u256(u128, u128);

scalar_inner_vectors!(u256(32));

impl Scalar for u256 {}
