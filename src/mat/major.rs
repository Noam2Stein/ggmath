use super::*;

pub trait MatMajor: fmt::Debug + Copy + PartialEq + PartialOrd + Default {
    type Mat2Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat2x3Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat2x4Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat3x2Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat3Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat3x4Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat4x2Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat4x3Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
    type Mat4Inner<T: Element>: fmt::Debug + Copy + PartialEq + PartialOrd + Default;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CM;
impl MatMajor for CM {
    type Mat2Inner<T: Element> = [Vec2<T>; 2];
    type Mat2x3Inner<T: Element> = [Vec3<T>; 2];
    type Mat2x4Inner<T: Element> = [Vec4<T>; 2];
    type Mat3x2Inner<T: Element> = [Vec2<T>; 3];
    type Mat3Inner<T: Element> = [Vec3<T>; 3];
    type Mat3x4Inner<T: Element> = [Vec4<T>; 3];
    type Mat4x2Inner<T: Element> = [Vec2<T>; 4];
    type Mat4x3Inner<T: Element> = [Vec3<T>; 4];
    type Mat4Inner<T: Element> = [Vec4<T>; 4];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RM;
impl MatMajor for RM {
    type Mat2Inner<T: Element> = [Vec2<T>; 2];
    type Mat2x3Inner<T: Element> = [Vec2<T>; 3];
    type Mat2x4Inner<T: Element> = [Vec2<T>; 4];
    type Mat3x2Inner<T: Element> = [Vec3<T>; 2];
    type Mat3Inner<T: Element> = [Vec3<T>; 3];
    type Mat3x4Inner<T: Element> = [Vec3<T>; 4];
    type Mat4x2Inner<T: Element> = [Vec4<T>; 2];
    type Mat4x3Inner<T: Element> = [Vec4<T>; 3];
    type Mat4Inner<T: Element> = [Vec4<T>; 4];
}
