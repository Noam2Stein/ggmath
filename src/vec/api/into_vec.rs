use super::*;

pub trait IntoVec2: Sized {
    type T: Scalar;

    fn into_vec_array(self) -> [Self::T; 2];

    fn into_vec<A: VecAlignment>(self) -> Vector2<Self::T, A> {
        Vector::from_array(self.into_vec_array())
    }
}
pub trait IntoVec3: Sized {
    type T: Scalar;

    fn into_vec_array(self) -> [Self::T; 3];

    fn into_vec<A: VecAlignment>(self) -> Vector3<Self::T, A> {
        Vector::from_array(self.into_vec_array())
    }
}
pub trait IntoVec4: Sized {
    type T: Scalar;

    fn into_vec_array(self) -> [Self::T; 4];

    fn into_vec<A: VecAlignment>(self) -> Vector4<Self::T, A> {
        Vector::from_array(self.into_vec_array())
    }
}

impl<T: Scalar> IntoVec2 for (T, T) {
    type T = T;
    fn into_vec_array(self) -> [T; 2] {
        [self.0, self.1]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVec2 for Vector2<T, A> {
    type T = T;
    fn into_vec_array(self) -> [T; 2] {
        self.into_array()
    }
}

impl<T: Scalar> IntoVec3 for (T, T, T) {
    type T = T;
    fn into_vec_array(self) -> [T; 3] {
        [self.0, self.1, self.2]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVec3 for (Vector2<T, A>, T) {
    type T = T;
    fn into_vec_array(self) -> [T; 3] {
        [self.0.x(), self.0.y(), self.1]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVec3 for Vector3<T, A> {
    type T = T;
    fn into_vec_array(self) -> [T; 3] {
        self.into_array()
    }
}
