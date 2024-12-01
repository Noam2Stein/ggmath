use derive_more::derive::{Add, Sub};
use ggmath::{scalar::WrapperScalar, vector::vec3};

fn main() {
    println!(
        "{}",
        vec3!(Meters(1.0), Meters(1.0), Meters(3.0)) + vec3!(Meters(0.0), Meters(1.0), Meters(0.0))
    )
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, WrapperScalar, Add, Sub)]
struct Meters(f32);

impl std::fmt::Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}
