use rsshader::reflection::{PrimitiveType, ShaderType, VectorType};

use crate::{Scalar, Simdness, SupportedVecLen, VecLen, Vector};

impl<const N: usize, T: Scalar + PrimitiveType, S: Simdness> ShaderType for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    const IR: rsshader::ir::Type = rsshader::ir::Type::Vector(rsshader::ir::Vector {
        length: match N {
            2 => rsshader::ir::Length::Two,
            3 => rsshader::ir::Length::Three,
            4 => rsshader::ir::Length::Four,
            _ => unreachable!(),
        },
        primitive: match T::IR {
            rsshader::ir::Type::Primitive(primitive) => primitive,
            _ => unreachable!(),
        },
    });
}

impl<const N: usize, T: Scalar + PrimitiveType, S: Simdness> VectorType<N, T> for Vector<N, T, S> where
    VecLen<N>: SupportedVecLen
{
}
