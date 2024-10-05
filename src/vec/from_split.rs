use super::*;

#[inline(always)]
pub fn vec2<T: Element>(value: impl VecSplit<2, T>) -> Vec2<T> {
    value.into_vec()
}
#[inline(always)]
pub fn vec3<T: Element>(value: impl VecSplit<3, T>) -> Vec3<T> {
    value.into_vec()
}
#[inline(always)]
pub fn vec4<T: Element>(value: impl VecSplit<4, T>) -> Vec4<T> {
    value.into_vec()
}
impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    #[inline(always)]
    pub fn from_split<S: VecSplit<N, T>>(value: S) -> Self {
        value.into_vec()
    }
}

pub trait VecSplit<const N: usize, T: Element>
where
    MaybeVecNum<N>: VecNum<N>,
{
    fn into_vec(self) -> VecN<N, T>;
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
            impl<T: Element> VecSplit<$n, T> for ($($vec<T>), *) {
                fn into_vec($input) -> VecN<$n, T> {
                    $output
                }
            }
        )*
    };
}
vec_splits!(
    Vec2(2):
    (Vec1, Vec1) self => VecN::from_array([self.0, self.1]),
    (Vec2) self => self,
);
vec_splits!(
    Vec3(3):
    (Vec1, Vec1, Vec1) self => VecN::from_array([self.0, self.1, self.2]),
    (Vec2, Vec1) self => VecN::from_array([self.0[0], self.0[1], self.1]),
    (Vec1, Vec2) self => VecN::from_array([self.0, self.1[0], self.1[1]]),
    (Vec3) self => self,
);
vec_splits!(
    Vec4(4):
    (Vec1, Vec1, Vec1, Vec1) self => VecN::from_array([self.0, self.1, self.2, self.3]),
    (Vec2, Vec1, Vec1) self => VecN::from_array([self.0[0], self.0[1], self.1, self.2]),
    (Vec1, Vec2, Vec1) self => VecN::from_array([self.0, self.1[0], self.1[1], self.2]),
    (Vec1, Vec1, Vec2) self => VecN::from_array([self.0, self.1, self.2[0], self.2[1]]),
    (Vec2, Vec2) self => VecN::from_array([self.0[0], self.0[1], self.1[0], self.1[1]]),
    (Vec3, Vec1) self => VecN::from_array([self.0[0], self.0[1], self.0[2], self.1]),
    (Vec1, Vec3) self => VecN::from_array([self.0, self.1[0], self.1[1], self.1[2]]),
    (Vec4) self => self,
);

type Vec1<T> = T;
