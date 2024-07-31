use std::{fmt, ops::*};
use crate::*;
#[derive(Debug, Clone, Copy)]
pub struct Vec4<T: Element> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
    pub(crate) w: T,
}
#[inline(always)]
pub fn vec4<T: Element>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4::new(x, y, z, w)
}
impl<T: Element> Vec4<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        let mut output = unsafe {
            std::mem::MaybeUninit::<Self>::uninit().assume_init()
        };
        output.x = x;
        output.y = y;
        output.z = z;
        output.w = w;
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
        output.w = value;
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
    #[inline(always)]
    pub fn with_w(mut self, w: T) -> Self {
        self.w = w;
        self
    }
}
impl<T: Element> fmt::Display for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
impl<T> Neg for Vec4<T>
where
    T: Element + Neg<Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn neg(self) -> <Self as Neg>::Output {
        Self::Output::new(self.x.neg(), self.y.neg(), self.z.neg(), self.w.neg())
    }
}
impl<T> Not for Vec4<T>
where
    T: Element + Not<Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn not(self) -> <Self as Not>::Output {
        Self::Output::new(self.x.not(), self.y.not(), self.z.not(), self.w.not())
    }
}
impl<RhsElement, T> Add<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + Add<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn add(self, rhs: Vec4<RhsElement>) -> <Self as Add<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.add(rhs.x),
            self.y.add(rhs.y),
            self.z.add(rhs.z),
            self.w.add(rhs.w),
        )
    }
}
impl<RhsElement, T> Sub<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + Sub<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn sub(self, rhs: Vec4<RhsElement>) -> <Self as Sub<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.sub(rhs.x),
            self.y.sub(rhs.y),
            self.z.sub(rhs.z),
            self.w.sub(rhs.w),
        )
    }
}
impl<RhsElement, T> Mul<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + Mul<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn mul(self, rhs: Vec4<RhsElement>) -> <Self as Mul<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.mul(rhs.x),
            self.y.mul(rhs.y),
            self.z.mul(rhs.z),
            self.w.mul(rhs.w),
        )
    }
}
impl<RhsElement, T> Div<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + Div<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn div(self, rhs: Vec4<RhsElement>) -> <Self as Div<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.div(rhs.x),
            self.y.div(rhs.y),
            self.z.div(rhs.z),
            self.w.div(rhs.w),
        )
    }
}
impl<RhsElement, T> Rem<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + Rem<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn rem(self, rhs: Vec4<RhsElement>) -> <Self as Rem<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.rem(rhs.x),
            self.y.rem(rhs.y),
            self.z.rem(rhs.z),
            self.w.rem(rhs.w),
        )
    }
}
impl<RhsElement, T> BitAnd<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + BitAnd<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn bitand(
        self,
        rhs: Vec4<RhsElement>,
    ) -> <Self as BitAnd<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.bitand(rhs.x),
            self.y.bitand(rhs.y),
            self.z.bitand(rhs.z),
            self.w.bitand(rhs.w),
        )
    }
}
impl<RhsElement, T> BitOr<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + BitOr<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn bitor(self, rhs: Vec4<RhsElement>) -> <Self as BitOr<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.bitor(rhs.x),
            self.y.bitor(rhs.y),
            self.z.bitor(rhs.z),
            self.w.bitor(rhs.w),
        )
    }
}
impl<RhsElement, T> BitXor<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + BitXor<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn bitxor(
        self,
        rhs: Vec4<RhsElement>,
    ) -> <Self as BitXor<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.bitxor(rhs.x),
            self.y.bitxor(rhs.y),
            self.z.bitxor(rhs.z),
            self.w.bitxor(rhs.w),
        )
    }
}
impl<RhsElement, T> Shl<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + Shl<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn shl(self, rhs: Vec4<RhsElement>) -> <Self as Shl<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.shl(rhs.x),
            self.y.shl(rhs.y),
            self.z.shl(rhs.z),
            self.w.shl(rhs.w),
        )
    }
}
impl<RhsElement, T> Shr<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + Shr<RhsElement, Output: Element>,
{
    type Output = Vec4<T::Output>;
    #[inline(always)]
    fn shr(self, rhs: Vec4<RhsElement>) -> <Self as Shr<Vec4<RhsElement>>>::Output {
        Self::Output::new(
            self.x.shr(rhs.x),
            self.y.shr(rhs.y),
            self.z.shr(rhs.z),
            self.w.shr(rhs.w),
        )
    }
}
impl<RhsElement, T> AddAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + AddAssign<RhsElement>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
        self.w.add_assign(rhs.w);
    }
}
impl<RhsElement, T> SubAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + SubAssign<RhsElement>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
        self.w.sub_assign(rhs.w);
    }
}
impl<RhsElement, T> MulAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + MulAssign<RhsElement>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
        self.w.mul_assign(rhs.w);
    }
}
impl<RhsElement, T> DivAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + DivAssign<RhsElement>,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
        self.w.div_assign(rhs.w);
    }
}
impl<RhsElement, T> RemAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + RemAssign<RhsElement>,
{
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
        self.z.rem_assign(rhs.z);
        self.w.rem_assign(rhs.w);
    }
}
impl<RhsElement, T> BitAndAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + BitAndAssign<RhsElement>,
{
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.bitand_assign(rhs.x);
        self.y.bitand_assign(rhs.y);
        self.z.bitand_assign(rhs.z);
        self.w.bitand_assign(rhs.w);
    }
}
impl<RhsElement, T> BitOrAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + BitOrAssign<RhsElement>,
{
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.bitor_assign(rhs.x);
        self.y.bitor_assign(rhs.y);
        self.z.bitor_assign(rhs.z);
        self.w.bitor_assign(rhs.w);
    }
}
impl<RhsElement, T> BitXorAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + BitXorAssign<RhsElement>,
{
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.bitxor_assign(rhs.x);
        self.y.bitxor_assign(rhs.y);
        self.z.bitxor_assign(rhs.z);
        self.w.bitxor_assign(rhs.w);
    }
}
impl<RhsElement, T> ShlAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + ShlAssign<RhsElement>,
{
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.shl_assign(rhs.x);
        self.y.shl_assign(rhs.y);
        self.z.shl_assign(rhs.z);
        self.w.shl_assign(rhs.w);
    }
}
impl<RhsElement, T> ShrAssign<Vec4<RhsElement>> for Vec4<T>
where
    RhsElement: Element,
    T: Element + ShrAssign<RhsElement>,
{
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Vec4<RhsElement>) {
        self.x.shr_assign(rhs.x);
        self.y.shr_assign(rhs.y);
        self.z.shr_assign(rhs.z);
        self.w.shr_assign(rhs.w);
    }
}
impl<T: Element> Vec4<T> {
    swizzle! {
        Vec2 < T >, [(xx, x -> x, x -> y), (yx, y -> x, x -> y), (zx, z -> x, x -> y),
        (wx, w -> x, x -> y), (xy, x -> x * 2), (yy, y -> x, y -> y), (zy, z -> x, y ->
        y), (wy, w -> x, y -> y), (xz, x -> x, z -> y), (yz, y -> x * 2), (zz, z -> x, z
        -> y), (wz, w -> x, z -> y), (xw, x -> x, w -> y), (yw, y -> x, w -> y), (zw, z
        -> x * 2), (ww, w -> x, w -> y),]
    }
    swizzle! {
        Vec3 < T >, [(xxx, x -> x, x -> y, x -> z), (yxx, y -> x, x -> y, x -> z), (zxx,
        z -> x, x -> y, x -> z), (wxx, w -> x, x -> y, x -> z), (xyx, x -> x * 2, x ->
        z), (yyx, y -> x, y -> y, x -> z), (zyx, z -> x, y -> y, x -> z), (wyx, w -> x, y
        -> y, x -> z), (xzx, x -> x, z -> y, x -> z), (yzx, y -> x * 2, x -> z), (zzx, z
        -> x, z -> y, x -> z), (wzx, w -> x, z -> y, x -> z), (xwx, x -> x, w -> y, x ->
        z), (ywx, y -> x, w -> y, x -> z), (zwx, z -> x * 2, x -> z), (wwx, w -> x, w ->
        y, x -> z), (xxy, x -> x, x -> y * 2), (yxy, y -> x, x -> y * 2), (zxy, z -> x, x
        -> y * 2), (wxy, w -> x, x -> y * 2), (xyy, x -> x * 2, y -> z), (yyy, y -> x, y
        -> y, y -> z), (zyy, z -> x, y -> y, y -> z), (wyy, w -> x, y -> y, y -> z),
        (xzy, x -> x, z -> y, y -> z), (yzy, y -> x * 2, y -> z), (zzy, z -> x, z -> y, y
        -> z), (wzy, w -> x, z -> y, y -> z), (xwy, x -> x, w -> y, y -> z), (ywy, y ->
        x, w -> y, y -> z), (zwy, z -> x * 2, y -> z), (wwy, w -> x, w -> y, y -> z),
        (xxz, x -> x, x -> y, z -> z), (yxz, y -> x, x -> y, z -> z), (zxz, z -> x, x ->
        y, z -> z), (wxz, w -> x, x -> y, z -> z), (xyz, x -> x * 3), (yyz, y -> x, y ->
        y * 2), (zyz, z -> x, y -> y * 2), (wyz, w -> x, y -> y * 2), (xzz, x -> x, z ->
        y, z -> z), (yzz, y -> x * 2, z -> z), (zzz, z -> x, z -> y, z -> z), (wzz, w ->
        x, z -> y, z -> z), (xwz, x -> x, w -> y, z -> z), (ywz, y -> x, w -> y, z -> z),
        (zwz, z -> x * 2, z -> z), (wwz, w -> x, w -> y, z -> z), (xxw, x -> x, x -> y, w
        -> z), (yxw, y -> x, x -> y, w -> z), (zxw, z -> x, x -> y, w -> z), (wxw, w ->
        x, x -> y, w -> z), (xyw, x -> x * 2, w -> z), (yyw, y -> x, y -> y, w -> z),
        (zyw, z -> x, y -> y, w -> z), (wyw, w -> x, y -> y, w -> z), (xzw, x -> x, z ->
        y * 2), (yzw, y -> x * 3), (zzw, z -> x, z -> y * 2), (wzw, w -> x, z -> y * 2),
        (xww, x -> x, w -> y, w -> z), (yww, y -> x, w -> y, w -> z), (zww, z -> x * 2, w
        -> z), (www, w -> x, w -> y, w -> z),]
    }
    swizzle! {
        Vec3A < T >, [(xxx_a, x -> x, x -> y, x -> z), (yxx_a, y -> x, x -> y, x -> z),
        (zxx_a, z -> x, x -> y, x -> z), (wxx_a, w -> x, x -> y, x -> z), (xyx_a, x -> x
        * 2, x -> z), (yyx_a, y -> x, y -> y, x -> z), (zyx_a, z -> x, y -> y, x -> z),
        (wyx_a, w -> x, y -> y, x -> z), (xzx_a, x -> x, z -> y, x -> z), (yzx_a, y -> x
        * 2, x -> z), (zzx_a, z -> x, z -> y, x -> z), (wzx_a, w -> x, z -> y, x -> z),
        (xwx_a, x -> x, w -> y, x -> z), (ywx_a, y -> x, w -> y, x -> z), (zwx_a, z -> x
        * 2, x -> z), (wwx_a, w -> x, w -> y, x -> z), (xxy_a, x -> x, x -> y * 2),
        (yxy_a, y -> x, x -> y * 2), (zxy_a, z -> x, x -> y * 2), (wxy_a, w -> x, x -> y
        * 2), (xyy_a, x -> x * 2, y -> z), (yyy_a, y -> x, y -> y, y -> z), (zyy_a, z ->
        x, y -> y, y -> z), (wyy_a, w -> x, y -> y, y -> z), (xzy_a, x -> x, z -> y, y ->
        z), (yzy_a, y -> x * 2, y -> z), (zzy_a, z -> x, z -> y, y -> z), (wzy_a, w -> x,
        z -> y, y -> z), (xwy_a, x -> x, w -> y, y -> z), (ywy_a, y -> x, w -> y, y ->
        z), (zwy_a, z -> x * 2, y -> z), (wwy_a, w -> x, w -> y, y -> z), (xxz_a, x -> x,
        x -> y, z -> z), (yxz_a, y -> x, x -> y, z -> z), (zxz_a, z -> x, x -> y, z ->
        z), (wxz_a, w -> x, x -> y, z -> z), (xyz_a, x -> x * 3), (yyz_a, y -> x, y -> y
        * 2), (zyz_a, z -> x, y -> y * 2), (wyz_a, w -> x, y -> y * 2), (xzz_a, x -> x, z
        -> y, z -> z), (yzz_a, y -> x * 2, z -> z), (zzz_a, z -> x, z -> y, z -> z),
        (wzz_a, w -> x, z -> y, z -> z), (xwz_a, x -> x, w -> y, z -> z), (ywz_a, y -> x,
        w -> y, z -> z), (zwz_a, z -> x * 2, z -> z), (wwz_a, w -> x, w -> y, z -> z),
        (xxw_a, x -> x, x -> y, w -> z), (yxw_a, y -> x, x -> y, w -> z), (zxw_a, z -> x,
        x -> y, w -> z), (wxw_a, w -> x, x -> y, w -> z), (xyw_a, x -> x * 2, w -> z),
        (yyw_a, y -> x, y -> y, w -> z), (zyw_a, z -> x, y -> y, w -> z), (wyw_a, w -> x,
        y -> y, w -> z), (xzw_a, x -> x, z -> y * 2), (yzw_a, y -> x * 3), (zzw_a, z ->
        x, z -> y * 2), (wzw_a, w -> x, z -> y * 2), (xww_a, x -> x, w -> y, w -> z),
        (yww_a, y -> x, w -> y, w -> z), (zww_a, z -> x * 2, w -> z), (www_a, w -> x, w
        -> y, w -> z),]
    }
    swizzle! {
        Vec4 < T >, [(xxxx, x -> x, x -> y, x -> z, x -> w), (yxxx, y -> x, x -> y, x ->
        z, x -> w), (zxxx, z -> x, x -> y, x -> z, x -> w), (wxxx, w -> x, x -> y, x ->
        z, x -> w), (xyxx, x -> x * 2, x -> z, x -> w), (yyxx, y -> x, y -> y, x -> z, x
        -> w), (zyxx, z -> x, y -> y, x -> z, x -> w), (wyxx, w -> x, y -> y, x -> z, x
        -> w), (xzxx, x -> x, z -> y, x -> z, x -> w), (yzxx, y -> x * 2, x -> z, x ->
        w), (zzxx, z -> x, z -> y, x -> z, x -> w), (wzxx, w -> x, z -> y, x -> z, x ->
        w), (xwxx, x -> x, w -> y, x -> z, x -> w), (ywxx, y -> x, w -> y, x -> z, x ->
        w), (zwxx, z -> x * 2, x -> z, x -> w), (wwxx, w -> x, w -> y, x -> z, x -> w),
        (xxyx, x -> x, x -> y * 2, x -> w), (yxyx, y -> x, x -> y * 2, x -> w), (zxyx, z
        -> x, x -> y * 2, x -> w), (wxyx, w -> x, x -> y * 2, x -> w), (xyyx, x -> x * 2,
        y -> z, x -> w), (yyyx, y -> x, y -> y, y -> z, x -> w), (zyyx, z -> x, y -> y, y
        -> z, x -> w), (wyyx, w -> x, y -> y, y -> z, x -> w), (xzyx, x -> x, z -> y, y
        -> z, x -> w), (yzyx, y -> x * 2, y -> z, x -> w), (zzyx, z -> x, z -> y, y -> z,
        x -> w), (wzyx, w -> x, z -> y, y -> z, x -> w), (xwyx, x -> x, w -> y, y -> z, x
        -> w), (ywyx, y -> x, w -> y, y -> z, x -> w), (zwyx, z -> x * 2, y -> z, x ->
        w), (wwyx, w -> x, w -> y, y -> z, x -> w), (xxzx, x -> x, x -> y, z -> z, x ->
        w), (yxzx, y -> x, x -> y, z -> z, x -> w), (zxzx, z -> x, x -> y, z -> z, x ->
        w), (wxzx, w -> x, x -> y, z -> z, x -> w), (xyzx, x -> x * 3, x -> w), (yyzx, y
        -> x, y -> y * 2, x -> w), (zyzx, z -> x, y -> y * 2, x -> w), (wyzx, w -> x, y
        -> y * 2, x -> w), (xzzx, x -> x, z -> y, z -> z, x -> w), (yzzx, y -> x * 2, z
        -> z, x -> w), (zzzx, z -> x, z -> y, z -> z, x -> w), (wzzx, w -> x, z -> y, z
        -> z, x -> w), (xwzx, x -> x, w -> y, z -> z, x -> w), (ywzx, y -> x, w -> y, z
        -> z, x -> w), (zwzx, z -> x * 2, z -> z, x -> w), (wwzx, w -> x, w -> y, z -> z,
        x -> w), (xxwx, x -> x, x -> y, w -> z, x -> w), (yxwx, y -> x, x -> y, w -> z, x
        -> w), (zxwx, z -> x, x -> y, w -> z, x -> w), (wxwx, w -> x, x -> y, w -> z, x
        -> w), (xywx, x -> x * 2, w -> z, x -> w), (yywx, y -> x, y -> y, w -> z, x ->
        w), (zywx, z -> x, y -> y, w -> z, x -> w), (wywx, w -> x, y -> y, w -> z, x ->
        w), (xzwx, x -> x, z -> y * 2, x -> w), (yzwx, y -> x * 3, x -> w), (zzwx, z ->
        x, z -> y * 2, x -> w), (wzwx, w -> x, z -> y * 2, x -> w), (xwwx, x -> x, w ->
        y, w -> z, x -> w), (ywwx, y -> x, w -> y, w -> z, x -> w), (zwwx, z -> x * 2, w
        -> z, x -> w), (wwwx, w -> x, w -> y, w -> z, x -> w), (xxxy, x -> x, x -> y, x
        -> z * 2), (yxxy, y -> x, x -> y, x -> z * 2), (zxxy, z -> x, x -> y, x -> z *
        2), (wxxy, w -> x, x -> y, x -> z * 2), (xyxy, x -> x * 2, x -> z * 2), (yyxy, y
        -> x, y -> y, x -> z * 2), (zyxy, z -> x, y -> y, x -> z * 2), (wyxy, w -> x, y
        -> y, x -> z * 2), (xzxy, x -> x, z -> y, x -> z * 2), (yzxy, y -> x * 2, x -> z
        * 2), (zzxy, z -> x, z -> y, x -> z * 2), (wzxy, w -> x, z -> y, x -> z * 2),
        (xwxy, x -> x, w -> y, x -> z * 2), (ywxy, y -> x, w -> y, x -> z * 2), (zwxy, z
        -> x * 2, x -> z * 2), (wwxy, w -> x, w -> y, x -> z * 2), (xxyy, x -> x, x -> y
        * 2, y -> w), (yxyy, y -> x, x -> y * 2, y -> w), (zxyy, z -> x, x -> y * 2, y ->
        w), (wxyy, w -> x, x -> y * 2, y -> w), (xyyy, x -> x * 2, y -> z, y -> w),
        (yyyy, y -> x, y -> y, y -> z, y -> w), (zyyy, z -> x, y -> y, y -> z, y -> w),
        (wyyy, w -> x, y -> y, y -> z, y -> w), (xzyy, x -> x, z -> y, y -> z, y -> w),
        (yzyy, y -> x * 2, y -> z, y -> w), (zzyy, z -> x, z -> y, y -> z, y -> w),
        (wzyy, w -> x, z -> y, y -> z, y -> w), (xwyy, x -> x, w -> y, y -> z, y -> w),
        (ywyy, y -> x, w -> y, y -> z, y -> w), (zwyy, z -> x * 2, y -> z, y -> w),
        (wwyy, w -> x, w -> y, y -> z, y -> w), (xxzy, x -> x, x -> y, z -> z, y -> w),
        (yxzy, y -> x, x -> y, z -> z, y -> w), (zxzy, z -> x, x -> y, z -> z, y -> w),
        (wxzy, w -> x, x -> y, z -> z, y -> w), (xyzy, x -> x * 3, y -> w), (yyzy, y ->
        x, y -> y * 2, y -> w), (zyzy, z -> x, y -> y * 2, y -> w), (wyzy, w -> x, y -> y
        * 2, y -> w), (xzzy, x -> x, z -> y, z -> z, y -> w), (yzzy, y -> x * 2, z -> z,
        y -> w), (zzzy, z -> x, z -> y, z -> z, y -> w), (wzzy, w -> x, z -> y, z -> z, y
        -> w), (xwzy, x -> x, w -> y, z -> z, y -> w), (ywzy, y -> x, w -> y, z -> z, y
        -> w), (zwzy, z -> x * 2, z -> z, y -> w), (wwzy, w -> x, w -> y, z -> z, y ->
        w), (xxwy, x -> x, x -> y, w -> z, y -> w), (yxwy, y -> x, x -> y, w -> z, y ->
        w), (zxwy, z -> x, x -> y, w -> z, y -> w), (wxwy, w -> x, x -> y, w -> z, y ->
        w), (xywy, x -> x * 2, w -> z, y -> w), (yywy, y -> x, y -> y, w -> z, y -> w),
        (zywy, z -> x, y -> y, w -> z, y -> w), (wywy, w -> x, y -> y, w -> z, y -> w),
        (xzwy, x -> x, z -> y * 2, y -> w), (yzwy, y -> x * 3, y -> w), (zzwy, z -> x, z
        -> y * 2, y -> w), (wzwy, w -> x, z -> y * 2, y -> w), (xwwy, x -> x, w -> y, w
        -> z, y -> w), (ywwy, y -> x, w -> y, w -> z, y -> w), (zwwy, z -> x * 2, w -> z,
        y -> w), (wwwy, w -> x, w -> y, w -> z, y -> w), (xxxz, x -> x, x -> y, x -> z, z
        -> w), (yxxz, y -> x, x -> y, x -> z, z -> w), (zxxz, z -> x, x -> y, x -> z, z
        -> w), (wxxz, w -> x, x -> y, x -> z, z -> w), (xyxz, x -> x * 2, x -> z, z ->
        w), (yyxz, y -> x, y -> y, x -> z, z -> w), (zyxz, z -> x, y -> y, x -> z, z ->
        w), (wyxz, w -> x, y -> y, x -> z, z -> w), (xzxz, x -> x, z -> y, x -> z, z ->
        w), (yzxz, y -> x * 2, x -> z, z -> w), (zzxz, z -> x, z -> y, x -> z, z -> w),
        (wzxz, w -> x, z -> y, x -> z, z -> w), (xwxz, x -> x, w -> y, x -> z, z -> w),
        (ywxz, y -> x, w -> y, x -> z, z -> w), (zwxz, z -> x * 2, x -> z, z -> w),
        (wwxz, w -> x, w -> y, x -> z, z -> w), (xxyz, x -> x, x -> y * 3), (yxyz, y ->
        x, x -> y * 3), (zxyz, z -> x, x -> y * 3), (wxyz, w -> x, x -> y * 3), (xyyz, x
        -> x * 2, y -> z * 2), (yyyz, y -> x, y -> y, y -> z * 2), (zyyz, z -> x, y -> y,
        y -> z * 2), (wyyz, w -> x, y -> y, y -> z * 2), (xzyz, x -> x, z -> y, y -> z *
        2), (yzyz, y -> x * 2, y -> z * 2), (zzyz, z -> x, z -> y, y -> z * 2), (wzyz, w
        -> x, z -> y, y -> z * 2), (xwyz, x -> x, w -> y, y -> z * 2), (ywyz, y -> x, w
        -> y, y -> z * 2), (zwyz, z -> x * 2, y -> z * 2), (wwyz, w -> x, w -> y, y -> z
        * 2), (xxzz, x -> x, x -> y, z -> z, z -> w), (yxzz, y -> x, x -> y, z -> z, z ->
        w), (zxzz, z -> x, x -> y, z -> z, z -> w), (wxzz, w -> x, x -> y, z -> z, z ->
        w), (xyzz, x -> x * 3, z -> w), (yyzz, y -> x, y -> y * 2, z -> w), (zyzz, z ->
        x, y -> y * 2, z -> w), (wyzz, w -> x, y -> y * 2, z -> w), (xzzz, x -> x, z ->
        y, z -> z, z -> w), (yzzz, y -> x * 2, z -> z, z -> w), (zzzz, z -> x, z -> y, z
        -> z, z -> w), (wzzz, w -> x, z -> y, z -> z, z -> w), (xwzz, x -> x, w -> y, z
        -> z, z -> w), (ywzz, y -> x, w -> y, z -> z, z -> w), (zwzz, z -> x * 2, z -> z,
        z -> w), (wwzz, w -> x, w -> y, z -> z, z -> w), (xxwz, x -> x, x -> y, w -> z, z
        -> w), (yxwz, y -> x, x -> y, w -> z, z -> w), (zxwz, z -> x, x -> y, w -> z, z
        -> w), (wxwz, w -> x, x -> y, w -> z, z -> w), (xywz, x -> x * 2, w -> z, z ->
        w), (yywz, y -> x, y -> y, w -> z, z -> w), (zywz, z -> x, y -> y, w -> z, z ->
        w), (wywz, w -> x, y -> y, w -> z, z -> w), (xzwz, x -> x, z -> y * 2, z -> w),
        (yzwz, y -> x * 3, z -> w), (zzwz, z -> x, z -> y * 2, z -> w), (wzwz, w -> x, z
        -> y * 2, z -> w), (xwwz, x -> x, w -> y, w -> z, z -> w), (ywwz, y -> x, w -> y,
        w -> z, z -> w), (zwwz, z -> x * 2, w -> z, z -> w), (wwwz, w -> x, w -> y, w ->
        z, z -> w), (xxxw, x -> x, x -> y, x -> z, w -> w), (yxxw, y -> x, x -> y, x ->
        z, w -> w), (zxxw, z -> x, x -> y, x -> z, w -> w), (wxxw, w -> x, x -> y, x ->
        z, w -> w), (xyxw, x -> x * 2, x -> z, w -> w), (yyxw, y -> x, y -> y, x -> z, w
        -> w), (zyxw, z -> x, y -> y, x -> z, w -> w), (wyxw, w -> x, y -> y, x -> z, w
        -> w), (xzxw, x -> x, z -> y, x -> z, w -> w), (yzxw, y -> x * 2, x -> z, w ->
        w), (zzxw, z -> x, z -> y, x -> z, w -> w), (wzxw, w -> x, z -> y, x -> z, w ->
        w), (xwxw, x -> x, w -> y, x -> z, w -> w), (ywxw, y -> x, w -> y, x -> z, w ->
        w), (zwxw, z -> x * 2, x -> z, w -> w), (wwxw, w -> x, w -> y, x -> z, w -> w),
        (xxyw, x -> x, x -> y * 2, w -> w), (yxyw, y -> x, x -> y * 2, w -> w), (zxyw, z
        -> x, x -> y * 2, w -> w), (wxyw, w -> x, x -> y * 2, w -> w), (xyyw, x -> x * 2,
        y -> z, w -> w), (yyyw, y -> x, y -> y, y -> z, w -> w), (zyyw, z -> x, y -> y, y
        -> z, w -> w), (wyyw, w -> x, y -> y, y -> z, w -> w), (xzyw, x -> x, z -> y, y
        -> z, w -> w), (yzyw, y -> x * 2, y -> z, w -> w), (zzyw, z -> x, z -> y, y -> z,
        w -> w), (wzyw, w -> x, z -> y, y -> z, w -> w), (xwyw, x -> x, w -> y, y -> z, w
        -> w), (ywyw, y -> x, w -> y, y -> z, w -> w), (zwyw, z -> x * 2, y -> z, w ->
        w), (wwyw, w -> x, w -> y, y -> z, w -> w), (xxzw, x -> x, x -> y, z -> z * 2),
        (yxzw, y -> x, x -> y, z -> z * 2), (zxzw, z -> x, x -> y, z -> z * 2), (wxzw, w
        -> x, x -> y, z -> z * 2), (xyzw, x -> x * 4), (yyzw, y -> x, y -> y * 3), (zyzw,
        z -> x, y -> y * 3), (wyzw, w -> x, y -> y * 3), (xzzw, x -> x, z -> y, z -> z *
        2), (yzzw, y -> x * 2, z -> z * 2), (zzzw, z -> x, z -> y, z -> z * 2), (wzzw, w
        -> x, z -> y, z -> z * 2), (xwzw, x -> x, w -> y, z -> z * 2), (ywzw, y -> x, w
        -> y, z -> z * 2), (zwzw, z -> x * 2, z -> z * 2), (wwzw, w -> x, w -> y, z -> z
        * 2), (xxww, x -> x, x -> y, w -> z, w -> w), (yxww, y -> x, x -> y, w -> z, w ->
        w), (zxww, z -> x, x -> y, w -> z, w -> w), (wxww, w -> x, x -> y, w -> z, w ->
        w), (xyww, x -> x * 2, w -> z, w -> w), (yyww, y -> x, y -> y, w -> z, w -> w),
        (zyww, z -> x, y -> y, w -> z, w -> w), (wyww, w -> x, y -> y, w -> z, w -> w),
        (xzww, x -> x, z -> y * 2, w -> w), (yzww, y -> x * 3, w -> w), (zzww, z -> x, z
        -> y * 2, w -> w), (wzww, w -> x, z -> y * 2, w -> w), (xwww, x -> x, w -> y, w
        -> z, w -> w), (ywww, y -> x, w -> y, w -> z, w -> w), (zwww, z -> x * 2, w -> z,
        w -> w), (wwww, w -> x, w -> y, w -> z, w -> w),]
    }
    set_swizzle! {
        Vec2 < T >, [(set_yx, x -> y, y -> x), (set_zx, x -> z, y -> x), (set_wx, x -> w,
        y -> x), (set_xy, x -> x * 2), (set_zy, x -> z, y -> y), (set_wy, x -> w, y ->
        y), (set_xz, x -> x, y -> z), (set_yz, x -> y * 2), (set_wz, x -> w, y -> z),
        (set_xw, x -> x, y -> w), (set_yw, x -> y, y -> w), (set_zw, x -> z * 2),]
    }
    set_swizzle! {
        Vec3 < T >, [(set_zyx, x -> z, y -> y, z -> x), (set_wyx, x -> w, y -> y, z ->
        x), (set_yzx, x -> y * 2, z -> x), (set_wzx, x -> w, y -> z, z -> x), (set_ywx, x
        -> y, y -> w, z -> x), (set_zwx, x -> z * 2, z -> x), (set_zxy, x -> z, y -> x *
        2), (set_wxy, x -> w, y -> x * 2), (set_xzy, x -> x, y -> z, z -> y), (set_wzy, x
        -> w, y -> z, z -> y), (set_xwy, x -> x, y -> w, z -> y), (set_zwy, x -> z * 2, z
        -> y), (set_yxz, x -> y, y -> x, z -> z), (set_wxz, x -> w, y -> x, z -> z),
        (set_xyz, x -> x * 3), (set_wyz, x -> w, y -> y * 2), (set_xwz, x -> x, y -> w, z
        -> z), (set_ywz, x -> y, y -> w, z -> z), (set_yxw, x -> y, y -> x, z -> w),
        (set_zxw, x -> z, y -> x, z -> w), (set_xyw, x -> x * 2, z -> w), (set_zyw, x ->
        z, y -> y, z -> w), (set_xzw, x -> x, y -> z * 2), (set_yzw, x -> y * 3),]
    }
    set_swizzle! {
        Vec3A < T >, [(set_zyx_a, x -> z, y -> y, z -> x), (set_wyx_a, x -> w, y -> y, z
        -> x), (set_yzx_a, x -> y * 2, z -> x), (set_wzx_a, x -> w, y -> z, z -> x),
        (set_ywx_a, x -> y, y -> w, z -> x), (set_zwx_a, x -> z * 2, z -> x), (set_zxy_a,
        x -> z, y -> x * 2), (set_wxy_a, x -> w, y -> x * 2), (set_xzy_a, x -> x, y -> z,
        z -> y), (set_wzy_a, x -> w, y -> z, z -> y), (set_xwy_a, x -> x, y -> w, z ->
        y), (set_zwy_a, x -> z * 2, z -> y), (set_yxz_a, x -> y, y -> x, z -> z),
        (set_wxz_a, x -> w, y -> x, z -> z), (set_xyz_a, x -> x * 3), (set_wyz_a, x -> w,
        y -> y * 2), (set_xwz_a, x -> x, y -> w, z -> z), (set_ywz_a, x -> y, y -> w, z
        -> z), (set_yxw_a, x -> y, y -> x, z -> w), (set_zxw_a, x -> z, y -> x, z -> w),
        (set_xyw_a, x -> x * 2, z -> w), (set_zyw_a, x -> z, y -> y, z -> w), (set_xzw_a,
        x -> x, y -> z * 2), (set_yzw_a, x -> y * 3),]
    }
    set_swizzle! {
        Vec4 < T >, [(set_wzyx, x -> w, y -> z, z -> y, w -> x), (set_zwyx, x -> z * 2, z
        -> y, w -> x), (set_wyzx, x -> w, y -> y * 2, w -> x), (set_ywzx, x -> y, y -> w,
        z -> z, w -> x), (set_zywx, x -> z, y -> y, z -> w, w -> x), (set_yzwx, x -> y *
        3, w -> x), (set_wzxy, x -> w, y -> z, z -> x * 2), (set_zwxy, x -> z * 2, z -> x
        * 2), (set_wxzy, x -> w, y -> x, z -> z, w -> y), (set_xwzy, x -> x, y -> w, z ->
        z, w -> y), (set_zxwy, x -> z, y -> x, z -> w, w -> y), (set_xzwy, x -> x, y -> z
        * 2, w -> y), (set_wyxz, x -> w, y -> y, z -> x, w -> z), (set_ywxz, x -> y, y ->
        w, z -> x, w -> z), (set_wxyz, x -> w, y -> x * 3), (set_xwyz, x -> x, y -> w, z
        -> y * 2), (set_yxwz, x -> y, y -> x, z -> w, w -> z), (set_xywz, x -> x * 2, z
        -> w, w -> z), (set_zyxw, x -> z, y -> y, z -> x, w -> w), (set_yzxw, x -> y * 2,
        z -> x, w -> w), (set_zxyw, x -> z, y -> x * 2, w -> w), (set_xzyw, x -> x, y ->
        z, z -> y, w -> w), (set_yxzw, x -> y, y -> x, z -> z * 2), (set_xyzw, x -> x *
        4),]
    }
    with_swizzle! {
        Vec2 < T >, [(with_yx, x -> y, y -> x), (with_zx, x -> z, y -> x), (with_wx, x ->
        w, y -> x), (with_xy, x -> x * 2), (with_zy, x -> z, y -> y), (with_wy, x -> w, y
        -> y), (with_xz, x -> x, y -> z), (with_yz, x -> y * 2), (with_wz, x -> w, y ->
        z), (with_xw, x -> x, y -> w), (with_yw, x -> y, y -> w), (with_zw, x -> z * 2),]
    }
    with_swizzle! {
        Vec3 < T >, [(with_zyx, x -> z, y -> y, z -> x), (with_wyx, x -> w, y -> y, z ->
        x), (with_yzx, x -> y * 2, z -> x), (with_wzx, x -> w, y -> z, z -> x),
        (with_ywx, x -> y, y -> w, z -> x), (with_zwx, x -> z * 2, z -> x), (with_zxy, x
        -> z, y -> x * 2), (with_wxy, x -> w, y -> x * 2), (with_xzy, x -> x, y -> z, z
        -> y), (with_wzy, x -> w, y -> z, z -> y), (with_xwy, x -> x, y -> w, z -> y),
        (with_zwy, x -> z * 2, z -> y), (with_yxz, x -> y, y -> x, z -> z), (with_wxz, x
        -> w, y -> x, z -> z), (with_xyz, x -> x * 3), (with_wyz, x -> w, y -> y * 2),
        (with_xwz, x -> x, y -> w, z -> z), (with_ywz, x -> y, y -> w, z -> z),
        (with_yxw, x -> y, y -> x, z -> w), (with_zxw, x -> z, y -> x, z -> w),
        (with_xyw, x -> x * 2, z -> w), (with_zyw, x -> z, y -> y, z -> w), (with_xzw, x
        -> x, y -> z * 2), (with_yzw, x -> y * 3),]
    }
    with_swizzle! {
        Vec3A < T >, [(with_zyx_a, x -> z, y -> y, z -> x), (with_wyx_a, x -> w, y -> y,
        z -> x), (with_yzx_a, x -> y * 2, z -> x), (with_wzx_a, x -> w, y -> z, z -> x),
        (with_ywx_a, x -> y, y -> w, z -> x), (with_zwx_a, x -> z * 2, z -> x),
        (with_zxy_a, x -> z, y -> x * 2), (with_wxy_a, x -> w, y -> x * 2), (with_xzy_a,
        x -> x, y -> z, z -> y), (with_wzy_a, x -> w, y -> z, z -> y), (with_xwy_a, x ->
        x, y -> w, z -> y), (with_zwy_a, x -> z * 2, z -> y), (with_yxz_a, x -> y, y ->
        x, z -> z), (with_wxz_a, x -> w, y -> x, z -> z), (with_xyz_a, x -> x * 3),
        (with_wyz_a, x -> w, y -> y * 2), (with_xwz_a, x -> x, y -> w, z -> z),
        (with_ywz_a, x -> y, y -> w, z -> z), (with_yxw_a, x -> y, y -> x, z -> w),
        (with_zxw_a, x -> z, y -> x, z -> w), (with_xyw_a, x -> x * 2, z -> w),
        (with_zyw_a, x -> z, y -> y, z -> w), (with_xzw_a, x -> x, y -> z * 2),
        (with_yzw_a, x -> y * 3),]
    }
    with_swizzle! {
        Vec4 < T >, [(with_wzyx, x -> w, y -> z, z -> y, w -> x), (with_zwyx, x -> z * 2,
        z -> y, w -> x), (with_wyzx, x -> w, y -> y * 2, w -> x), (with_ywzx, x -> y, y
        -> w, z -> z, w -> x), (with_zywx, x -> z, y -> y, z -> w, w -> x), (with_yzwx, x
        -> y * 3, w -> x), (with_wzxy, x -> w, y -> z, z -> x * 2), (with_zwxy, x -> z *
        2, z -> x * 2), (with_wxzy, x -> w, y -> x, z -> z, w -> y), (with_xwzy, x -> x,
        y -> w, z -> z, w -> y), (with_zxwy, x -> z, y -> x, z -> w, w -> y), (with_xzwy,
        x -> x, y -> z * 2, w -> y), (with_wyxz, x -> w, y -> y, z -> x, w -> z),
        (with_ywxz, x -> y, y -> w, z -> x, w -> z), (with_wxyz, x -> w, y -> x * 3),
        (with_xwyz, x -> x, y -> w, z -> y * 2), (with_yxwz, x -> y, y -> x, z -> w, w ->
        z), (with_xywz, x -> x * 2, z -> w, w -> z), (with_zyxw, x -> z, y -> y, z -> x,
        w -> w), (with_yzxw, x -> y * 2, z -> x, w -> w), (with_zxyw, x -> z, y -> x * 2,
        w -> w), (with_xzyw, x -> x, y -> z, z -> y, w -> w), (with_yxzw, x -> y, y -> x,
        z -> z * 2), (with_xyzw, x -> x * 4),]
    }
}
