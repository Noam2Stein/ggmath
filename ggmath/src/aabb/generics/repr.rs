use derive_where::derive_where;

use super::*;

#[allow(private_bounds)]
pub unsafe trait AabbRepr: AabbReprPriv + Sized + 'static {}

pub struct AabbCornered;
pub struct AabbCentered;
pub struct AabbMinMaxed;

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub fn to_cornered(self) -> Aabb<N, T, A, AabbCornered> {
        self.to_layout()
    }

    #[inline(always)]
    pub fn to_centered(self) -> Aabb<N, T, A, AabbCentered> {
        self.to_layout()
    }

    #[inline(always)]
    pub fn to_min_maxed(self) -> Aabb<N, T, A, AabbMinMaxed> {
        self.to_layout()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AabbReprEnum {
    Cornered,
    Centered,
    MinMaxed,
}

pub(crate) trait AabbReprPriv {
    const ENUM: AabbReprEnum;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>: Construct + PartialEq
    where
        Usize<N>: VecLen;
}

unsafe impl AabbRepr for AabbCornered {}
unsafe impl AabbRepr for AabbCentered {}
unsafe impl AabbRepr for AabbMinMaxed {}

impl AabbReprPriv for AabbCornered {
    const ENUM: AabbReprEnum = AabbReprEnum::Cornered;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerCorneredAabb<N, T, A>
    where
        Usize<N>: VecLen;
}
impl AabbReprPriv for AabbCentered {
    const ENUM: AabbReprEnum = AabbReprEnum::Centered;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerCenteredAabb<N, T, A>
    where
        Usize<N>: VecLen;
}
impl AabbReprPriv for AabbMinMaxed {
    const ENUM: AabbReprEnum = AabbReprEnum::MinMaxed;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerMinMaxedAabb<N, T, A>
    where
        Usize<N>: VecLen;
}

#[repr(C)]
#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(crate) struct InnerCorneredAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub size: Vector<N, T, A>,
}

#[repr(C)]
#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(crate) struct InnerMinMaxedAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub max: Vector<N, T, A>,
}

#[repr(C)]
#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(crate) struct InnerCenteredAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub center: Vector<N, T, A>,
    pub extents: Vector<N, T, A>,
}
