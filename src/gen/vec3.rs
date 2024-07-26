use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<C: Component> {
	pub(crate) x: C,
	pub(crate) y: C,
	pub(crate) z: C,
}

#[inline(always)]
pub const fn vec3<C: Component>(x: C, y: C, z: C) -> Vec3<C> {
	Vec3::new(x, y, z)
}
impl<C: Component> Vec3<C> {
	#[inline(always)]
	pub const fn new(x: C, y: C, z: C) -> Self {
		Self {
			x,
			y,
			z,

		}
	}
}

impl<C: Component> std::fmt::Display for Vec3<C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl<C: Component> Vec3<C> {
	#[inline(always)] pub const fn xx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn xy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 2]) } }
	#[inline(always)] pub const fn xz(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 1, z -> y * 1]) } }
	#[inline(always)] pub const fn yx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn yy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 1, y -> y * 1]) } }
	#[inline(always)] pub const fn yz(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 2]) } }
	#[inline(always)] pub const fn zx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [z -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn zy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [z -> x * 1, y -> y * 1]) } }
	#[inline(always)] pub const fn zz(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [z -> x * 1, z -> y * 1]) } }
	#[inline(always)] pub const fn xxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn xxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn xxz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xxz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn xyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn xyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn xyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn xyz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 3]) } }
	#[inline(always)] pub const fn xyz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 3]) } }
	#[inline(always)] pub const fn xzx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xzx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xzy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn xzy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn xzz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xzz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn yxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn yxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn yxz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn yxz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn yyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn yyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn yyz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn yyz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn yzx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn yzx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn yzy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn yzy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn yzz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 2, z -> z * 2]) } }
	#[inline(always)] pub const fn yzz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 2, z -> z * 2]) } }
	#[inline(always)] pub const fn zxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn zxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn zxz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn zxz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn zyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn zyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn zyz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn zyz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn zzx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zzx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zzy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn zzy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn zzz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn zzz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn xxxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn xxyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 3]) } }
	#[inline(always)] pub const fn xxzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xxzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xxzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 3]) } }
	#[inline(always)] pub const fn xyxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn xyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn xyyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 3]) } }
	#[inline(always)] pub const fn xyzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 3, x -> w * 3]) } }
	#[inline(always)] pub const fn xyzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 3, y -> w * 3]) } }
	#[inline(always)] pub const fn xyzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 3, z -> w * 3]) } }
	#[inline(always)] pub const fn xzxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xzxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn xzxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xzyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xzyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xzyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn xzzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xzzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xzzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn yxxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yxyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 3]) } }
	#[inline(always)] pub const fn yxzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yxzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn yxzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn yyxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn yyyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn yyzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yyzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yyzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn yzxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, x -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yzxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, x -> z * 3]) } }
	#[inline(always)] pub const fn yzxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, x -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn yzyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, y -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yzyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, y -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yzyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, y -> z * 3]) } }
	#[inline(always)] pub const fn yzzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, z -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yzzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, z -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yzzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, z -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn zxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn zxxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn zxyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 3]) } }
	#[inline(always)] pub const fn zxzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zxzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zxzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn zyxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zyyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn zyzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zyzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn zyzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn zzxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zzxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn zzxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zzyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zzyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zzyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn zzzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zzzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zzzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, z -> z * 1, z -> w * 1]) } }
}