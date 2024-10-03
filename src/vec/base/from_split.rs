use super::*;

#[inline(always)]
pub fn vec2<T: Element>(value: impl VecSplit<T, 2, Output = Vec2<T>>) -> Vec2<T> {
    value.into_vec()
}
#[inline(always)]
pub fn vec3<T: Element>(value: impl VecSplit<T, 3, Output = Vec3<T>>) -> Vec3<T> {
    value.into_vec()
}
#[inline(always)]
pub fn vec4<T: Element>(value: impl VecSplit<T, 4, Output = Vec4<T>>) -> Vec4<T> {
    value.into_vec()
}

impl<T: Element> Vec2<T> {
    #[inline(always)]
    pub fn from_split<S: VecSplit<T, 2, Output = Self>>(value: S) -> Self {
        value.into_vec()
    }
}
impl<T: Element> Vec3<T> {
    #[inline(always)]
    pub fn from_split<S: VecSplit<T, 3, Output = Self>>(value: S) -> Self {
        value.into_vec()
    }
}
impl<T: Element> Vec4<T> {
    #[inline(always)]
    pub fn from_split<S: VecSplit<T, 4, Output = Self>>(value: S) -> Self {
        value.into_vec()
    }
}

pub trait VecSplit<T: Element, const N: usize> {
    type Output: VecN<T, N>;
    fn into_vec(self) -> Self::Output;
}

macro_rules! vec_splits {
    (
        $output_ty:ident($n:literal):
        $(
            ($($vec:ident), * $(,)?) $input:ident => $output:expr
        ), *
        $(,)?
    ) => {
        $(
            #[allow(unused_parens)]
            impl<T: Element> VecSplit<T, $n> for ($($vec<T>), *) {
                type Output = $output_ty<T>;
                fn into_vec($input) -> Self::Output {
                    $output
                }
            }
        )*
    };
}
vec_splits!(
    Vec2(2):
    (Vec1, Vec1) self => Self::Output::from_array([self.0, self.1]),
    (Vec2) self => self,
);
vec_splits!(
    Vec3(3):
    (Vec1, Vec1, Vec1) self => Self::Output::from_array([self.0, self.1, self.2]),
    (Vec2, Vec1) self => Self::Output::from_array([self.0[0], self.0[1], self.1]),
    (Vec1, Vec2) self => Self::Output::from_array([self.0, self.1[0], self.1[1]]),
    (Vec3) self => self,
);
vec_splits!(
    Vec4(4):
    (Vec1, Vec1, Vec1, Vec1) self => Self::Output::from_array([self.0, self.1, self.2, self.3]),
    (Vec2, Vec1, Vec1) self => Self::Output::from_array([self.0[0], self.0[1], self.1, self.2]),
    (Vec1, Vec2, Vec1) self => Self::Output::from_array([self.0, self.1[0], self.1[1], self.2]),
    (Vec1, Vec1, Vec2) self => Self::Output::from_array([self.0, self.1, self.2[0], self.2[1]]),
    (Vec2, Vec2) self => Self::Output::from_array([self.0[0], self.0[1], self.1[0], self.1[1]]),
    (Vec3, Vec1) self => Self::Output::from_array([self.0[0], self.0[1], self.0[2], self.1]),
    (Vec1, Vec3) self => Self::Output::from_array([self.0, self.1[0], self.1[1], self.1[2]]),
    (Vec4) self => self,
);

type Vec1<T> = T;
