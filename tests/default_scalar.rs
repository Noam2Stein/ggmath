#![allow(unused)]

use ggmath::{
    Alignment, Length, Scalar, ScalarBackend, ScalarDefault, SupportedLength, Unaligned, Vector,
};

#[derive(Clone, Copy)]
struct Foo(f32);

impl Scalar for Foo {}

impl ScalarDefault for Foo {}
