use std::mem::{transmute, transmute_copy};

use super::*;

pub type VectorOrT<const N: usize, T, A> = <ScalarCount<N> as VecLenOr1>::VectorOrT<T, A>;

#[allow(private_bounds)]
pub trait VecLenOr1: Seal {
    type VectorOrT<T: Scalar, A: VecAlignment>: Construct;
}

pub enum ResolvedVectorOrT<T: Scalar, A: VecAlignment> {
    T(T),
    Vec2(Vector<2, T, A>),
    Vec3(Vector<3, T, A>),
    Vec4(Vector<4, T, A>),
}
pub enum ResolvedVectorOrTRef<'a, T: Scalar, A: VecAlignment> {
    T(&'a T),
    Vec2(&'a Vector<2, T, A>),
    Vec3(&'a Vector<3, T, A>),
    Vec4(&'a Vector<4, T, A>),
}
pub enum ResolvedVectorOrTMut<'a, T: Scalar, A: VecAlignment> {
    T(&'a mut T),
    Vec2(&'a mut Vector<2, T, A>),
    Vec3(&'a mut Vector<3, T, A>),
    Vec4(&'a mut Vector<4, T, A>),
}

impl<const N: usize> VecLenOr1 for ScalarCount<N>
where
    ScalarCount<N>: VecLen,
{
    type VectorOrT<T: Scalar, A: VecAlignment> = Vector<N, T, A>;
}

pub fn resolve_vector_or_t_length<const N: usize, T: Scalar, A: VecAlignment>(
    vec_or_t: VectorOrT<N, T, A>,
) -> ResolvedVectorOrT<T, A>
where
    ScalarCount<N>: VecLenOr1,
{
    unsafe {
        match N {
            1 => ResolvedVectorOrT::T(transmute_copy(&vec_or_t)),
            2 => ResolvedVectorOrT::Vec2(transmute_copy(&vec_or_t)),
            3 => ResolvedVectorOrT::Vec3(transmute_copy(&vec_or_t)),
            4 => ResolvedVectorOrT::Vec4(transmute_copy(&vec_or_t)),
            _ => panic!("invalid VecLenOr1"),
        }
    }
}
pub fn resolve_vector_or_t_length_ref<const N: usize, T: Scalar, A: VecAlignment>(
    vec_or_scalar: &VectorOrT<N, T, A>,
) -> ResolvedVectorOrTRef<T, A>
where
    ScalarCount<N>: VecLenOr1,
{
    unsafe {
        match N {
            1 => ResolvedVectorOrTRef::T(transmute(vec_or_scalar)),
            2 => ResolvedVectorOrTRef::Vec2(transmute(vec_or_scalar)),
            3 => ResolvedVectorOrTRef::Vec3(transmute(vec_or_scalar)),
            4 => ResolvedVectorOrTRef::Vec4(transmute(vec_or_scalar)),
            _ => panic!("invalid VecLenOr1"),
        }
    }
}
pub fn resolve_vector_or_t_length_mut<const N: usize, T: Scalar, A: VecAlignment>(
    vec_or_scalar: &mut VectorOrT<N, T, A>,
) -> ResolvedVectorOrTMut<T, A>
where
    ScalarCount<N>: VecLenOr1,
{
    unsafe {
        match N {
            1 => ResolvedVectorOrTMut::T(transmute(vec_or_scalar)),
            2 => ResolvedVectorOrTMut::Vec2(transmute(vec_or_scalar)),
            3 => ResolvedVectorOrTMut::Vec3(transmute(vec_or_scalar)),
            4 => ResolvedVectorOrTMut::Vec4(transmute(vec_or_scalar)),
            _ => panic!("invalid VecLenOr1"),
        }
    }
}

impl VecLenOr1 for ScalarCount<1> {
    type VectorOrT<T: Scalar, A: VecAlignment> = T;
}

trait Seal {}
impl<const N: usize> Seal for ScalarCount<N> {}
