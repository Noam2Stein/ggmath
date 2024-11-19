use ggmath::{
    rectangle::{repr::*, *},
    scalar::*,
    vector::{alignment::*, length::*, *},
};

pub struct World<const D: usize, P: ScalarNum>
where
    ScalarCount<D>: VecLen<D>,
{
    pub bodies: Vec<Body<D, P>>,
    pub gravity_dir: Vector<D, P, VecAligned>,
}

pub struct Body<const D: usize, P: ScalarNum>
where
    ScalarCount<D>: VecLen<D>,
{
    pub rect: Rectangle<D, P, VecAligned, RectCentered>,
    pub velocity: Vector<D, P, VecAligned>,
}

pub type World2D<P> = World<2, P>;
pub type World3D<P> = World<3, P>;
pub type World4D<P> = World<4, P>;