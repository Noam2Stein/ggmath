use genco::{lang::rust::Tokens, quote};

use crate::{
    iter::{Primitive, PrimitiveInt},
    module::{TestDir, TestFile, TokensExt},
};

mod bool;
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

fn primitive_testmod(t: Primitive) -> TestFile {
    let mut tokens = Tokens::new();

    primitive::push(t, &mut tokens);
    match t {
        Primitive::Float(primitive) => {
            float::push(primitive, &mut tokens);
        }
        Primitive::Int(primitive) => {
            int::push(primitive, &mut tokens);

            match primitive {
                PrimitiveInt::Sint(primitive) => {
                    sint::push(primitive, &mut tokens);
                }
                PrimitiveInt::Uint(primitive) => {
                    uint::push(primitive, &mut tokens);
                }
            }
        }
        Primitive::Bool => {
            bool::push(&mut tokens);
        }
    }

    tokens.to_testfile(t.as_str())
}
