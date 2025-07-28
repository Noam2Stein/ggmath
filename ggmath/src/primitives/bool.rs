use super::*;

primitive_aliases! { pub B => bool }

impl Scalar for bool {
    type Vec2Alignment = Align<1>;
    type Vec3Alignment = Align<1>;
    type Vec4Alignment = Align<1>;
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
where
    Usize<N>: VecLen,
{
    pub const FALSE: Self = Self::from_array([false; N]);
    pub const TRUE: Self = Self::from_array([true; N]);
}
