use crate::iter::Primitive;

mod float;

pub fn generate() {
    for t in Primitive::iter() {
        match t {
            Primitive::Float(t) => float::generate(t),
            _ => {}
        }
    }
}
