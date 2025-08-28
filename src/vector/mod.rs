use crate::{Construct, Usize};

#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub inner: A::InnerVector<N, T>,
}

pub trait VecLen {
    type InnerAlignedVector<T: Scalar>: Construct;
}

pub trait Scalar: Construct {
    type InnerAlignedVec2: Construct;
    type InnerAlignedVec3: Construct;
    type InnerAlignedVec4: Construct;

    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2;
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3;
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4;
}

pub trait VecAlignment: 'static {
    type InnerVector<const N: usize, T: Scalar>: Construct
    where
        Usize<N>: VecLen;
}

pub struct VecAligned;
pub struct VecPacked;

impl VecLen for Usize<2> {
    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec2;
}
impl VecLen for Usize<3> {
    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec3;
}
impl VecLen for Usize<4> {
    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec4;
}

impl VecAlignment for VecAligned {
    type InnerVector<const N: usize, T: Scalar>
        = <Usize<N> as VecLen>::InnerAlignedVector<T>
    where
        Usize<N>: VecLen;
}
impl VecAlignment for VecPacked {
    type InnerVector<const N: usize, T: Scalar>
        = [T; N]
    where
        Usize<N>: VecLen;
}

impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where Usize<N>: VecLen {}

#[cfg(test)]
mod tests {
    use super::*;

    fn _verify_construct_impl<const N: usize, T: Scalar, A: VecAlignment>()
    where
        Usize<N>: VecLen,
    {
        fn helper<T: Construct>() {}

        helper::<Vector<N, T, A>>();
    }
}
