use super::*;

pub fn vector<const N: usize, T: Scalar, A: VecAlignment>(
    value: impl IntoVector<N, T>,
) -> Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    value.into_vec()
}
pub fn vector2<T: Scalar, A: VecAlignment>(value: impl IntoVector<2, T>) -> Vector2<T, A> {
    value.into_vec()
}
pub fn vector3<T: Scalar, A: VecAlignment>(value: impl IntoVector<3, T>) -> Vector3<T, A> {
    value.into_vec()
}
pub fn vector4<T: Scalar, A: VecAlignment>(value: impl IntoVector<4, T>) -> Vector4<T, A> {
    value.into_vec()
}

pub fn vecn<const N: usize, T: Scalar>(value: impl IntoVector<N, T>) -> VecN<N, T>
where
    ScalarCount<N>: VecLen<N>,
{
    value.into_vec()
}
pub fn vec2<T: Scalar>(value: impl IntoVector<2, T>) -> Vec2<T> {
    value.into_vec()
}
pub fn vec3<T: Scalar>(value: impl IntoVector<3, T>) -> Vec3<T> {
    value.into_vec()
}
pub fn vec4<T: Scalar>(value: impl IntoVector<4, T>) -> Vec4<T> {
    value.into_vec()
}

pub fn vecnp<const N: usize, T: Scalar>(value: impl IntoVector<N, T>) -> VecNP<N, T>
where
    ScalarCount<N>: VecLen<N>,
{
    value.into_vec()
}
pub fn vec2p<T: Scalar>(value: impl IntoVector<2, T>) -> Vec2P<T> {
    value.into_vec()
}
pub fn vec3p<T: Scalar>(value: impl IntoVector<3, T>) -> Vec3P<T> {
    value.into_vec()
}
pub fn vec4p<T: Scalar>(value: impl IntoVector<4, T>) -> Vec4P<T> {
    value.into_vec()
}

pub trait IntoVector<const N: usize, T: Scalar>: Sized
where
    ScalarCount<N>: VecLen<N>,
{
    fn into_vec_array(self) -> [T; N];

    fn into_vec<A: VecAlignment>(self) -> Vector<N, T, A> {
        Vector::from_array(self.into_vec_array())
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment> IntoVector<N, T> for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    fn into_vec_array(self) -> [T; N] {
        self.into_array()
    }
}

impl<T: Scalar> IntoVector<2, T> for (T, T) {
    fn into_vec_array(self) -> [T; 2] {
        [self.0, self.1]
    }
}

impl<T: Scalar> IntoVector<3, T> for (T, T, T) {
    fn into_vec_array(self) -> [T; 3] {
        [self.0, self.1, self.2]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<3, T> for (Vector2<T, A>, T) {
    fn into_vec_array(self) -> [T; 3] {
        [self.0.x(), self.0.y(), self.1]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<3, T> for (T, Vector2<T, A>) {
    fn into_vec_array(self) -> [T; 3] {
        [self.0, self.1.x(), self.1.y()]
    }
}

impl<T: Scalar> IntoVector<4, T> for (T, T, T, T) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0, self.1, self.2, self.3]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (Vector2<T, A>, T, T) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0.x(), self.0.y(), self.1, self.2]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, Vector2<T, A>, T) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0, self.1.x(), self.1.y(), self.2]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, T, Vector2<T, A>) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0, self.1, self.2.x(), self.2.y()]
    }
}
impl<T: Scalar, A0: VecAlignment, A1: VecAlignment> IntoVector<4, T>
    for (Vector2<T, A0>, Vector2<T, A1>)
{
    fn into_vec_array(self) -> [T; 4] {
        [self.0.x(), self.0.y(), self.1.x(), self.1.y()]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (Vector3<T, A>, T) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0.x(), self.0.y(), self.0.z(), self.1]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, Vector3<T, A>) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0, self.1.x(), self.1.y(), self.1.z()]
    }
}
