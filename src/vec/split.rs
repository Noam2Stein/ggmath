use super::*;

trait Seal {}

pub trait VecSplit {
    type Output: VecN;
    fn into_vec(self) -> Self::Output;
}
impl<V: VecN> VecSplit for V {
    type Output = V;
    fn into_vec(self) -> Self::Output {
        self
    }
}

macro_rules! vec_splits {
    (
        $output:ident:
        $(
            ($($vec:ident), * $(,)?)($fn:ident)
        ), *
        $(,)?
    ) => {
        $(
            impl<T: Element> VecSplit for ($($vec<T>), *) {
                type Output = $output<T>;
                fn into_vec(self) -> Self::Output {
                    Self::Output {
                        inner: <T as ElementVecsFromSplits>::$fn(unsafe { std::mem::transmute(self) })
                    }
                }
            }
        )*
    };
}
vec_splits!(
    Vec2:
    (Vec1, Vec1)(from_x_y),
);
vec_splits!(
    Vec3:
    (Vec1, Vec1, Vec1)(from_x_y_z),
    (Vec2, Vec1)(from_xy_z),
    (Vec1, Vec2)(from_x_yz),
);
vec_splits!(
    Vec4:
    (Vec1, Vec1, Vec1, Vec1)(from_x_y_z_w),
    (Vec2, Vec1, Vec1)(from_xy_z_w),
    (Vec1, Vec2, Vec1)(from_x_yz_w),
    (Vec1, Vec1, Vec2)(from_x_y_zw),
    (Vec2, Vec2)(from_xy_zw),
    (Vec3, Vec1)(from_xyz_w),
    (Vec1, Vec3)(from_x_yzw),
);
