use rsshader::{
    ir::{LengthIr, TypeIr, VectorIr},
    reflection::{PrimitiveType, ShaderType, VectorType},
};

use crate::{Scalar, Simdness, SupportedVecLen, VecLen, Vector};

impl<const N: usize, T: Scalar + PrimitiveType, S: Simdness> ShaderType for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    const IR: TypeIr = TypeIr::Vector(VectorIr {
        length: match N {
            2 => LengthIr::Two,
            3 => LengthIr::Three,
            4 => LengthIr::Four,
            _ => unreachable!(),
        },
        primitive: match T::IR {
            TypeIr::Primitive(primitive) => primitive,
            _ => unreachable!(),
        },
    });
}

impl<const N: usize, T: Scalar + PrimitiveType, S: Simdness> VectorType<N, T> for Vector<N, T, S> where
    VecLen<N>: SupportedVecLen
{
}
