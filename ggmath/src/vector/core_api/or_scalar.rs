use std::mem::{transmute, transmute_copy};

use super::*;

pub type VectorOrScalar<const N: usize, T, A> = <ScalarCount<N> as VecLenOr1>::VectorOrScalar<T, A>;

#[allow(private_bounds)]
pub trait VecLenOr1: Seal {
    type VectorOrScalar<T: Scalar, A: VecAlignment>: Construct;
}

pub enum ResolvedVectorOrScalar<T: Scalar, A: VecAlignment> {
    Scalar(T),
    Vec2(Vector<2, T, A>),
    Vec3(Vector<3, T, A>),
    Vec4(Vector<4, T, A>),
}
pub enum ResolvedVectorOrScalarRef<'a, T: Scalar, A: VecAlignment> {
    Scalar(&'a T),
    Vec2(&'a Vector<2, T, A>),
    Vec3(&'a Vector<3, T, A>),
    Vec4(&'a Vector<4, T, A>),
}
pub enum ResolvedVectorOrScalarMut<'a, T: Scalar, A: VecAlignment> {
    Scalar(&'a mut T),
    Vec2(&'a mut Vector<2, T, A>),
    Vec3(&'a mut Vector<3, T, A>),
    Vec4(&'a mut Vector<4, T, A>),
}

impl<const N: usize> VecLenOr1 for ScalarCount<N>
where
    ScalarCount<N>: VecLen,
{
    type VectorOrScalar<T: Scalar, A: VecAlignment> = Vector<N, T, A>;
}

pub fn resolve_vector_or_scalar_length<const N: usize, T: Scalar, A: VecAlignment>(
    vec_or_scalar: VectorOrScalar<N, T, A>,
) -> ResolvedVectorOrScalar<T, A>
where
    ScalarCount<N>: VecLenOr1,
{
    unsafe {
        match N {
            1 => ResolvedVectorOrScalar::Scalar(transmute_copy(&vec_or_scalar)),
            2 => ResolvedVectorOrScalar::Vec2(transmute_copy(&vec_or_scalar)),
            3 => ResolvedVectorOrScalar::Vec3(transmute_copy(&vec_or_scalar)),
            4 => ResolvedVectorOrScalar::Vec4(transmute_copy(&vec_or_scalar)),
            _ => panic!("invalid VecLenOr1"),
        }
    }
}
pub fn resolve_vector_or_scalar_length_ref<const N: usize, T: Scalar, A: VecAlignment>(
    vec_or_scalar: &VectorOrScalar<N, T, A>,
) -> ResolvedVectorOrScalarRef<T, A>
where
    ScalarCount<N>: VecLenOr1,
{
    unsafe {
        match N {
            1 => ResolvedVectorOrScalarRef::Scalar(transmute(vec_or_scalar)),
            2 => ResolvedVectorOrScalarRef::Vec2(transmute(vec_or_scalar)),
            3 => ResolvedVectorOrScalarRef::Vec3(transmute(vec_or_scalar)),
            4 => ResolvedVectorOrScalarRef::Vec4(transmute(vec_or_scalar)),
            _ => panic!("invalid VecLenOr1"),
        }
    }
}
pub fn resolve_vector_or_scalar_length_mut<const N: usize, T: Scalar, A: VecAlignment>(
    vec_or_scalar: &mut VectorOrScalar<N, T, A>,
) -> ResolvedVectorOrScalarMut<T, A>
where
    ScalarCount<N>: VecLenOr1,
{
    unsafe {
        match N {
            1 => ResolvedVectorOrScalarMut::Scalar(transmute(vec_or_scalar)),
            2 => ResolvedVectorOrScalarMut::Vec2(transmute(vec_or_scalar)),
            3 => ResolvedVectorOrScalarMut::Vec3(transmute(vec_or_scalar)),
            4 => ResolvedVectorOrScalarMut::Vec4(transmute(vec_or_scalar)),
            _ => panic!("invalid VecLenOr1"),
        }
    }
}

impl VecLenOr1 for ScalarCount<1> {
    type VectorOrScalar<T: Scalar, A: VecAlignment> = T;
}

trait Seal {}
impl<const N: usize> Seal for ScalarCount<N> {}
