use std::{fmt, ops::*};
use crate::*;
#[derive(Debug, Clone, Copy)]
pub struct Vec3A<T: Element> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
    pub(crate) _alignment: [T; 1usize],
}
#[inline(always)]
pub fn vec3a<T: Element>(x: T, y: T, z: T) -> Vec3A<T> {
    Vec3A::new(x, y, z)
}
impl<T: Element> Vec3A<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Self {
        let mut output = unsafe {
            std::mem::MaybeUninit::<Self>::uninit().assume_init()
        };
        output.x = x;
        output.y = y;
        output.z = z;
        output
    }
    #[inline(always)]
    pub fn splat(value: T) -> Self {
        let mut output = unsafe {
            std::mem::MaybeUninit::<Self>::uninit().assume_init()
        };
        output.x = value;
        output.y = value;
        output.z = value;
        output
    }
    #[inline(always)]
    pub fn with_x(mut self, x: T) -> Self {
        self.x = x;
        self
    }
    #[inline(always)]
    pub fn with_y(mut self, y: T) -> Self {
        self.y = y;
        self
    }
    #[inline(always)]
    pub fn with_z(mut self, z: T) -> Self {
        self.z = z;
        self
    }
}
impl<T: Element> fmt::Display for Vec3A<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl<T> Neg for Vec3A<T>
where
    T: Element + Neg<Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn neg(self) -> <Self as Neg>::Output {
        Self::Output::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
}
impl<T> Not for Vec3A<T>
where
    T: Element + Not<Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn not(self) -> <Self as Not>::Output {
        Self::Output::new(self.x.not(), self.y.not(), self.z.not())
    }
}
impl<RhsElement, T> Add<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + Add<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn add(self, rhs: Vec3A<RhsElement>) -> <Self as Add<Vec3A<RhsElement>>>::Output {
        Self::Output::new(self.x.add(rhs.x), self.y.add(rhs.y), self.z.add(rhs.z))
    }
}
impl<RhsElement, T> Sub<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + Sub<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn sub(self, rhs: Vec3A<RhsElement>) -> <Self as Sub<Vec3A<RhsElement>>>::Output {
        Self::Output::new(self.x.sub(rhs.x), self.y.sub(rhs.y), self.z.sub(rhs.z))
    }
}
impl<RhsElement, T> Mul<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + Mul<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn mul(self, rhs: Vec3A<RhsElement>) -> <Self as Mul<Vec3A<RhsElement>>>::Output {
        Self::Output::new(self.x.mul(rhs.x), self.y.mul(rhs.y), self.z.mul(rhs.z))
    }
}
impl<RhsElement, T> Div<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + Div<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn div(self, rhs: Vec3A<RhsElement>) -> <Self as Div<Vec3A<RhsElement>>>::Output {
        Self::Output::new(self.x.div(rhs.x), self.y.div(rhs.y), self.z.div(rhs.z))
    }
}
impl<RhsElement, T> Rem<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + Rem<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn rem(self, rhs: Vec3A<RhsElement>) -> <Self as Rem<Vec3A<RhsElement>>>::Output {
        Self::Output::new(self.x.rem(rhs.x), self.y.rem(rhs.y), self.z.rem(rhs.z))
    }
}
impl<RhsElement, T> BitAnd<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + BitAnd<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn bitand(
        self,
        rhs: Vec3A<RhsElement>,
    ) -> <Self as BitAnd<Vec3A<RhsElement>>>::Output {
        Self::Output::new(
            self.x.bitand(rhs.x),
            self.y.bitand(rhs.y),
            self.z.bitand(rhs.z),
        )
    }
}
impl<RhsElement, T> BitOr<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + BitOr<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn bitor(
        self,
        rhs: Vec3A<RhsElement>,
    ) -> <Self as BitOr<Vec3A<RhsElement>>>::Output {
        Self::Output::new(self.x.bitor(rhs.x), self.y.bitor(rhs.y), self.z.bitor(rhs.z))
    }
}
impl<RhsElement, T> BitXor<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + BitXor<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn bitxor(
        self,
        rhs: Vec3A<RhsElement>,
    ) -> <Self as BitXor<Vec3A<RhsElement>>>::Output {
        Self::Output::new(
            self.x.bitxor(rhs.x),
            self.y.bitxor(rhs.y),
            self.z.bitxor(rhs.z),
        )
    }
}
impl<RhsElement, T> Shl<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + Shl<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn shl(self, rhs: Vec3A<RhsElement>) -> <Self as Shl<Vec3A<RhsElement>>>::Output {
        Self::Output::new(self.x.shl(rhs.x), self.y.shl(rhs.y), self.z.shl(rhs.z))
    }
}
impl<RhsElement, T> Shr<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + Shr<RhsElement, Output: Element>,
{
    type Output = Vec3A<T::Output>;
    #[inline(always)]
    fn shr(self, rhs: Vec3A<RhsElement>) -> <Self as Shr<Vec3A<RhsElement>>>::Output {
        Self::Output::new(self.x.shr(rhs.x), self.y.shr(rhs.y), self.z.shr(rhs.z))
    }
}
impl<RhsElement, T> AddAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + AddAssign<RhsElement>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}
impl<RhsElement, T> SubAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + SubAssign<RhsElement>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}
impl<RhsElement, T> MulAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + MulAssign<RhsElement>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
    }
}
impl<RhsElement, T> DivAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + DivAssign<RhsElement>,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
    }
}
impl<RhsElement, T> RemAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + RemAssign<RhsElement>,
{
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
        self.z.rem_assign(rhs.z);
    }
}
impl<RhsElement, T> BitAndAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + BitAndAssign<RhsElement>,
{
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.bitand_assign(rhs.x);
        self.y.bitand_assign(rhs.y);
        self.z.bitand_assign(rhs.z);
    }
}
impl<RhsElement, T> BitOrAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + BitOrAssign<RhsElement>,
{
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.bitor_assign(rhs.x);
        self.y.bitor_assign(rhs.y);
        self.z.bitor_assign(rhs.z);
    }
}
impl<RhsElement, T> BitXorAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + BitXorAssign<RhsElement>,
{
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.bitxor_assign(rhs.x);
        self.y.bitxor_assign(rhs.y);
        self.z.bitxor_assign(rhs.z);
    }
}
impl<RhsElement, T> ShlAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + ShlAssign<RhsElement>,
{
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.shl_assign(rhs.x);
        self.y.shl_assign(rhs.y);
        self.z.shl_assign(rhs.z);
    }
}
impl<RhsElement, T> ShrAssign<Vec3A<RhsElement>> for Vec3A<T>
where
    RhsElement: Element,
    T: Element + ShrAssign<RhsElement>,
{
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Vec3A<RhsElement>) {
        self.x.shr_assign(rhs.x);
        self.y.shr_assign(rhs.y);
        self.z.shr_assign(rhs.z);
    }
}
impl<T: Element> Vec3A<T> {
    swizzle! {
        Vec2 < T >, [(xx, x -> x, x -> y), (yx, y -> x, x -> y), (zx, z -> x, x -> y),
        (xy, x -> x * 2), (yy, y -> x, y -> y), (zy, z -> x, y -> y), (xz, x -> x, z ->
        y), (yz, y -> x * 2), (zz, z -> x, z -> y),]
    }
    swizzle! {
        Vec3 < T >, [(xxx, x -> x, x -> y, x -> z), (yxx, y -> x, x -> y, x -> z), (zxx,
        z -> x, x -> y, x -> z), (xyx, x -> x * 2, x -> z), (yyx, y -> x, y -> y, x ->
        z), (zyx, z -> x, y -> y, x -> z), (xzx, x -> x, z -> y, x -> z), (yzx, y -> x *
        2, x -> z), (zzx, z -> x, z -> y, x -> z), (xxy, x -> x, x -> y * 2), (yxy, y ->
        x, x -> y * 2), (zxy, z -> x, x -> y * 2), (xyy, x -> x * 2, y -> z), (yyy, y ->
        x, y -> y, y -> z), (zyy, z -> x, y -> y, y -> z), (xzy, x -> x, z -> y, y -> z),
        (yzy, y -> x * 2, y -> z), (zzy, z -> x, z -> y, y -> z), (xxz, x -> x, x -> y, z
        -> z), (yxz, y -> x, x -> y, z -> z), (zxz, z -> x, x -> y, z -> z), (xyz, x -> x
        * 3), (yyz, y -> x, y -> y * 2), (zyz, z -> x, y -> y * 2), (xzz, x -> x, z -> y,
        z -> z), (yzz, y -> x * 2, z -> z), (zzz, z -> x, z -> y, z -> z),]
    }
    swizzle! {
        Vec3A < T >, [(xxx_a, x -> x, x -> y, x -> z), (yxx_a, y -> x, x -> y, x -> z),
        (zxx_a, z -> x, x -> y, x -> z), (xyx_a, x -> x * 2, x -> z), (yyx_a, y -> x, y
        -> y, x -> z), (zyx_a, z -> x, y -> y, x -> z), (xzx_a, x -> x, z -> y, x -> z),
        (yzx_a, y -> x * 2, x -> z), (zzx_a, z -> x, z -> y, x -> z), (xxy_a, x -> x, x
        -> y * 2), (yxy_a, y -> x, x -> y * 2), (zxy_a, z -> x, x -> y * 2), (xyy_a, x ->
        x * 2, y -> z), (yyy_a, y -> x, y -> y, y -> z), (zyy_a, z -> x, y -> y, y -> z),
        (xzy_a, x -> x, z -> y, y -> z), (yzy_a, y -> x * 2, y -> z), (zzy_a, z -> x, z
        -> y, y -> z), (xxz_a, x -> x, x -> y, z -> z), (yxz_a, y -> x, x -> y, z -> z),
        (zxz_a, z -> x, x -> y, z -> z), (xyz_a, x -> x * 3), (yyz_a, y -> x, y -> y *
        2), (zyz_a, z -> x, y -> y * 2), (xzz_a, x -> x, z -> y, z -> z), (yzz_a, y -> x
        * 2, z -> z), (zzz_a, z -> x, z -> y, z -> z),]
    }
    swizzle! {
        Vec4 < T >, [(xxxx, x -> x, x -> y, x -> z, x -> w), (yxxx, y -> x, x -> y, x ->
        z, x -> w), (zxxx, z -> x, x -> y, x -> z, x -> w), (xyxx, x -> x * 2, x -> z, x
        -> w), (yyxx, y -> x, y -> y, x -> z, x -> w), (zyxx, z -> x, y -> y, x -> z, x
        -> w), (xzxx, x -> x, z -> y, x -> z, x -> w), (yzxx, y -> x * 2, x -> z, x ->
        w), (zzxx, z -> x, z -> y, x -> z, x -> w), (xxyx, x -> x, x -> y * 2, x -> w),
        (yxyx, y -> x, x -> y * 2, x -> w), (zxyx, z -> x, x -> y * 2, x -> w), (xyyx, x
        -> x * 2, y -> z, x -> w), (yyyx, y -> x, y -> y, y -> z, x -> w), (zyyx, z -> x,
        y -> y, y -> z, x -> w), (xzyx, x -> x, z -> y, y -> z, x -> w), (yzyx, y -> x *
        2, y -> z, x -> w), (zzyx, z -> x, z -> y, y -> z, x -> w), (xxzx, x -> x, x ->
        y, z -> z, x -> w), (yxzx, y -> x, x -> y, z -> z, x -> w), (zxzx, z -> x, x ->
        y, z -> z, x -> w), (xyzx, x -> x * 3, x -> w), (yyzx, y -> x, y -> y * 2, x ->
        w), (zyzx, z -> x, y -> y * 2, x -> w), (xzzx, x -> x, z -> y, z -> z, x -> w),
        (yzzx, y -> x * 2, z -> z, x -> w), (zzzx, z -> x, z -> y, z -> z, x -> w),
        (xxxy, x -> x, x -> y, x -> z * 2), (yxxy, y -> x, x -> y, x -> z * 2), (zxxy, z
        -> x, x -> y, x -> z * 2), (xyxy, x -> x * 2, x -> z * 2), (yyxy, y -> x, y -> y,
        x -> z * 2), (zyxy, z -> x, y -> y, x -> z * 2), (xzxy, x -> x, z -> y, x -> z *
        2), (yzxy, y -> x * 2, x -> z * 2), (zzxy, z -> x, z -> y, x -> z * 2), (xxyy, x
        -> x, x -> y * 2, y -> w), (yxyy, y -> x, x -> y * 2, y -> w), (zxyy, z -> x, x
        -> y * 2, y -> w), (xyyy, x -> x * 2, y -> z, y -> w), (yyyy, y -> x, y -> y, y
        -> z, y -> w), (zyyy, z -> x, y -> y, y -> z, y -> w), (xzyy, x -> x, z -> y, y
        -> z, y -> w), (yzyy, y -> x * 2, y -> z, y -> w), (zzyy, z -> x, z -> y, y -> z,
        y -> w), (xxzy, x -> x, x -> y, z -> z, y -> w), (yxzy, y -> x, x -> y, z -> z, y
        -> w), (zxzy, z -> x, x -> y, z -> z, y -> w), (xyzy, x -> x * 3, y -> w), (yyzy,
        y -> x, y -> y * 2, y -> w), (zyzy, z -> x, y -> y * 2, y -> w), (xzzy, x -> x, z
        -> y, z -> z, y -> w), (yzzy, y -> x * 2, z -> z, y -> w), (zzzy, z -> x, z -> y,
        z -> z, y -> w), (xxxz, x -> x, x -> y, x -> z, z -> w), (yxxz, y -> x, x -> y, x
        -> z, z -> w), (zxxz, z -> x, x -> y, x -> z, z -> w), (xyxz, x -> x * 2, x -> z,
        z -> w), (yyxz, y -> x, y -> y, x -> z, z -> w), (zyxz, z -> x, y -> y, x -> z, z
        -> w), (xzxz, x -> x, z -> y, x -> z, z -> w), (yzxz, y -> x * 2, x -> z, z ->
        w), (zzxz, z -> x, z -> y, x -> z, z -> w), (xxyz, x -> x, x -> y * 3), (yxyz, y
        -> x, x -> y * 3), (zxyz, z -> x, x -> y * 3), (xyyz, x -> x * 2, y -> z * 2),
        (yyyz, y -> x, y -> y, y -> z * 2), (zyyz, z -> x, y -> y, y -> z * 2), (xzyz, x
        -> x, z -> y, y -> z * 2), (yzyz, y -> x * 2, y -> z * 2), (zzyz, z -> x, z -> y,
        y -> z * 2), (xxzz, x -> x, x -> y, z -> z, z -> w), (yxzz, y -> x, x -> y, z ->
        z, z -> w), (zxzz, z -> x, x -> y, z -> z, z -> w), (xyzz, x -> x * 3, z -> w),
        (yyzz, y -> x, y -> y * 2, z -> w), (zyzz, z -> x, y -> y * 2, z -> w), (xzzz, x
        -> x, z -> y, z -> z, z -> w), (yzzz, y -> x * 2, z -> z, z -> w), (zzzz, z -> x,
        z -> y, z -> z, z -> w),]
    }
    set_swizzle! {
        Vec2 < T >, [(set_yx, x -> y, y -> x), (set_zx, x -> z, y -> x), (set_xy, x -> x
        * 2), (set_zy, x -> z, y -> y), (set_xz, x -> x, y -> z), (set_yz, x -> y * 2),]
    }
    set_swizzle! {
        Vec3 < T >, [(set_zyx, x -> z, y -> y, z -> x), (set_yzx, x -> y * 2, z -> x),
        (set_zxy, x -> z, y -> x * 2), (set_xzy, x -> x, y -> z, z -> y), (set_yxz, x ->
        y, y -> x, z -> z), (set_xyz, x -> x * 3),]
    }
    set_swizzle! {
        Vec3A < T >, [(set_zyx_a, x -> z, y -> y, z -> x), (set_yzx_a, x -> y * 2, z ->
        x), (set_zxy_a, x -> z, y -> x * 2), (set_xzy_a, x -> x, y -> z, z -> y),
        (set_yxz_a, x -> y, y -> x, z -> z), (set_xyz_a, x -> x * 3),]
    }
    with_swizzle! {
        Vec2 < T >, [(with_yx, x -> y, y -> x), (with_zx, x -> z, y -> x), (with_xy, x ->
        x * 2), (with_zy, x -> z, y -> y), (with_xz, x -> x, y -> z), (with_yz, x -> y *
        2),]
    }
    with_swizzle! {
        Vec3 < T >, [(with_zyx, x -> z, y -> y, z -> x), (with_yzx, x -> y * 2, z -> x),
        (with_zxy, x -> z, y -> x * 2), (with_xzy, x -> x, y -> z, z -> y), (with_yxz, x
        -> y, y -> x, z -> z), (with_xyz, x -> x * 3),]
    }
    with_swizzle! {
        Vec3A < T >, [(with_zyx_a, x -> z, y -> y, z -> x), (with_yzx_a, x -> y * 2, z ->
        x), (with_zxy_a, x -> z, y -> x * 2), (with_xzy_a, x -> x, y -> z, z -> y),
        (with_yxz_a, x -> y, y -> x, z -> z), (with_xyz_a, x -> x * 3),]
    }
}
