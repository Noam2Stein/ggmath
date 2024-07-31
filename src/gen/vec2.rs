use std::{fmt, ops::*};
use crate::*;
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T: Element> {
    pub(crate) x: T,
    pub(crate) y: T,
}
#[inline(always)]
pub fn vec2<T: Element>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}
impl<T: Element> Vec2<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        let mut output = unsafe {
            std::mem::MaybeUninit::<Self>::uninit().assume_init()
        };
        output.x = x;
        output.y = y;
        output
    }
    #[inline(always)]
    pub fn splat(value: T) -> Self {
        let mut output = unsafe {
            std::mem::MaybeUninit::<Self>::uninit().assume_init()
        };
        output.x = value;
        output.y = value;
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
}
impl<T: Element> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl<T> Neg for Vec2<T>
where
    T: Element + Neg<Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn neg(self) -> <Self as Neg>::Output {
        Self::Output::new(self.x.neg(), self.y.neg())
    }
}
impl<T> Not for Vec2<T>
where
    T: Element + Not<Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn not(self) -> <Self as Not>::Output {
        Self::Output::new(self.x.not(), self.y.not())
    }
}
impl<RhsElement, T> Add<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + Add<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn add(self, rhs: Vec2<RhsElement>) -> <Self as Add<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.add(rhs.x), self.y.add(rhs.y))
    }
}
impl<RhsElement, T> Sub<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + Sub<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn sub(self, rhs: Vec2<RhsElement>) -> <Self as Sub<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.sub(rhs.x), self.y.sub(rhs.y))
    }
}
impl<RhsElement, T> Mul<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + Mul<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn mul(self, rhs: Vec2<RhsElement>) -> <Self as Mul<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.mul(rhs.x), self.y.mul(rhs.y))
    }
}
impl<RhsElement, T> Div<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + Div<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn div(self, rhs: Vec2<RhsElement>) -> <Self as Div<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.div(rhs.x), self.y.div(rhs.y))
    }
}
impl<RhsElement, T> Rem<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + Rem<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn rem(self, rhs: Vec2<RhsElement>) -> <Self as Rem<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.rem(rhs.x), self.y.rem(rhs.y))
    }
}
impl<RhsElement, T> BitAnd<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + BitAnd<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn bitand(
        self,
        rhs: Vec2<RhsElement>,
    ) -> <Self as BitAnd<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.bitand(rhs.x), self.y.bitand(rhs.y))
    }
}
impl<RhsElement, T> BitOr<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + BitOr<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn bitor(self, rhs: Vec2<RhsElement>) -> <Self as BitOr<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.bitor(rhs.x), self.y.bitor(rhs.y))
    }
}
impl<RhsElement, T> BitXor<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + BitXor<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn bitxor(
        self,
        rhs: Vec2<RhsElement>,
    ) -> <Self as BitXor<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.bitxor(rhs.x), self.y.bitxor(rhs.y))
    }
}
impl<RhsElement, T> Shl<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + Shl<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn shl(self, rhs: Vec2<RhsElement>) -> <Self as Shl<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.shl(rhs.x), self.y.shl(rhs.y))
    }
}
impl<RhsElement, T> Shr<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + Shr<RhsElement, Output: Element>,
{
    type Output = Vec2<T::Output>;
    #[inline(always)]
    fn shr(self, rhs: Vec2<RhsElement>) -> <Self as Shr<Vec2<RhsElement>>>::Output {
        Self::Output::new(self.x.shr(rhs.x), self.y.shr(rhs.y))
    }
}
impl<RhsElement, T> AddAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + AddAssign<RhsElement>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}
impl<RhsElement, T> SubAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + SubAssign<RhsElement>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}
impl<RhsElement, T> MulAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + MulAssign<RhsElement>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}
impl<RhsElement, T> DivAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + DivAssign<RhsElement>,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}
impl<RhsElement, T> RemAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + RemAssign<RhsElement>,
{
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}
impl<RhsElement, T> BitAndAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + BitAndAssign<RhsElement>,
{
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.bitand_assign(rhs.x);
        self.y.bitand_assign(rhs.y);
    }
}
impl<RhsElement, T> BitOrAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + BitOrAssign<RhsElement>,
{
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.bitor_assign(rhs.x);
        self.y.bitor_assign(rhs.y);
    }
}
impl<RhsElement, T> BitXorAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + BitXorAssign<RhsElement>,
{
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.bitxor_assign(rhs.x);
        self.y.bitxor_assign(rhs.y);
    }
}
impl<RhsElement, T> ShlAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + ShlAssign<RhsElement>,
{
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.shl_assign(rhs.x);
        self.y.shl_assign(rhs.y);
    }
}
impl<RhsElement, T> ShrAssign<Vec2<RhsElement>> for Vec2<T>
where
    RhsElement: Element,
    T: Element + ShrAssign<RhsElement>,
{
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Vec2<RhsElement>) {
        self.x.shr_assign(rhs.x);
        self.y.shr_assign(rhs.y);
    }
}
impl<T: Element> Vec2<T> {
    swizzle! {
        Vec2 < T >, [(xx, x -> x, x -> y), (yx, y -> x, x -> y), (xy, x -> x * 2), (yy, y
        -> x, y -> y),]
    }
    swizzle! {
        Vec3 < T >, [(xxx, x -> x, x -> y, x -> z), (yxx, y -> x, x -> y, x -> z), (xyx,
        x -> x * 2, x -> z), (yyx, y -> x, y -> y, x -> z), (xxy, x -> x, x -> y * 2),
        (yxy, y -> x, x -> y * 2), (xyy, x -> x * 2, y -> z), (yyy, y -> x, y -> y, y ->
        z),]
    }
    swizzle! {
        Vec3A < T >, [(xxx_a, x -> x, x -> y, x -> z), (yxx_a, y -> x, x -> y, x -> z),
        (xyx_a, x -> x * 2, x -> z), (yyx_a, y -> x, y -> y, x -> z), (xxy_a, x -> x, x
        -> y * 2), (yxy_a, y -> x, x -> y * 2), (xyy_a, x -> x * 2, y -> z), (yyy_a, y ->
        x, y -> y, y -> z),]
    }
    swizzle! {
        Vec4 < T >, [(xxxx, x -> x, x -> y, x -> z, x -> w), (yxxx, y -> x, x -> y, x ->
        z, x -> w), (xyxx, x -> x * 2, x -> z, x -> w), (yyxx, y -> x, y -> y, x -> z, x
        -> w), (xxyx, x -> x, x -> y * 2, x -> w), (yxyx, y -> x, x -> y * 2, x -> w),
        (xyyx, x -> x * 2, y -> z, x -> w), (yyyx, y -> x, y -> y, y -> z, x -> w),
        (xxxy, x -> x, x -> y, x -> z * 2), (yxxy, y -> x, x -> y, x -> z * 2), (xyxy, x
        -> x * 2, x -> z * 2), (yyxy, y -> x, y -> y, x -> z * 2), (xxyy, x -> x, x -> y
        * 2, y -> w), (yxyy, y -> x, x -> y * 2, y -> w), (xyyy, x -> x * 2, y -> z, y ->
        w), (yyyy, y -> x, y -> y, y -> z, y -> w),]
    }
    set_swizzle! {
        Vec2 < T >, [(set_yx, x -> y, y -> x), (set_xy, x -> x * 2),]
    }
    with_swizzle! {
        Vec2 < T >, [(with_yx, x -> y, y -> x), (with_xy, x -> x * 2),]
    }
}
