use genco::{lang::rust::Tokens, quote};
use strum::IntoEnumIterator;

use crate::{
    backend::{TestDir, TestFile, TokensExt},
    iter::{Length, Primitive, PrimitiveInt, Simdness},
};

mod bool_;
mod float;
mod int;
mod primitive;
mod sint;
mod uint;

pub fn testmod() -> TestDir {
    quote! {
        $(for primitive in Primitive::iter() join($['\r']) => mod $primitive;)
    }
    .to_testdir("vector")
    .with_submod_files(Primitive::iter().map(primitive_testmod))
}

#[derive(Default)]
struct PrimitiveTestMod {
    items: Vec<Tokens>,
}

fn primitive_testmod(primitive: Primitive) -> TestFile {
    let mut output = PrimitiveTestMod::default();

    primitive::push_tests(primitive, &mut output);
    match primitive {
        Primitive::Float(primitive) => {
            float::push_tests(primitive, &mut output);
        }
        Primitive::Int(primitive) => {
            int::push_tests(primitive, &mut output);

            match primitive {
                PrimitiveInt::Sint(primitive) => {
                    sint::push_tests(primitive, &mut output);
                }
                PrimitiveInt::Uint(primitive) => {
                    uint::push_tests(primitive, &mut output);
                }
            }
        }
        Primitive::Bool => {
            bool_::push_tests(&mut output);
        }
    }

    quote! {
        $(for item in output.items join($['\n']) => $item)
    }
    .to_testfile(primitive.as_str())
}

impl PrimitiveTestMod {
    fn push_for_vector(
        &mut self,
        primitive: impl Into<Primitive>,
        mut f: impl FnMut(Length, Simdness) -> Tokens,
    ) {
        let primitive = primitive.into();

        for length in Length::iter() {
            for simdness in Simdness::iter() {
                if !primitive.is_primary() && simdness != Simdness::Simd && length != Length::N4 {
                    continue;
                }

                self.items.push(f(length, simdness))
            }
        }
    }
}
