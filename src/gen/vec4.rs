use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4<C: Component> {
	pub(crate) x: C,
	pub(crate) y: C,
	pub(crate) z: C,
	pub(crate) w: C,
}

#[inline(always)]
pub const fn vec4<C: Component>(x: C, y: C, z: C, w: C) -> Vec4<C> {
	Vec4::new(x, y, z, w)
}
impl<C: Component> Vec4<C> {
	#[inline(always)]
	pub const fn new(x: C, y: C, z: C, w: C) -> Self {
		Self {
			x,
			y,
			z,
			w,

		}
	}
}

impl<C: Component> std::fmt::Display for Vec4<C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
	}
}

impl<C: Component> Vec4<C> {
	#[inline(always)] pub const fn xx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn xy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 2]) } }
	#[inline(always)] pub const fn xz(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 1, z -> y * 1]) } }
	#[inline(always)] pub const fn xw(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 1, w -> y * 1]) } }
	#[inline(always)] pub const fn yx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn yy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 1, y -> y * 1]) } }
	#[inline(always)] pub const fn yz(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 2]) } }
	#[inline(always)] pub const fn yw(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 1, w -> y * 1]) } }
	#[inline(always)] pub const fn zx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [z -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn zy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [z -> x * 1, y -> y * 1]) } }
	#[inline(always)] pub const fn zz(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [z -> x * 1, z -> y * 1]) } }
	#[inline(always)] pub const fn zw(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [z -> x * 2]) } }
	#[inline(always)] pub const fn wx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [w -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn wy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [w -> x * 1, y -> y * 1]) } }
	#[inline(always)] pub const fn wz(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [w -> x * 1, z -> y * 1]) } }
	#[inline(always)] pub const fn ww(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [w -> x * 1, w -> y * 1]) } }
	#[inline(always)] pub const fn xxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn xxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn xxz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xxz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xxw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn xxw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn xyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn xyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn xyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn xyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn xyz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 3]) } }
	#[inline(always)] pub const fn xyz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 3]) } }
	#[inline(always)] pub const fn xyw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 2, w -> z * 2]) } }
	#[inline(always)] pub const fn xyw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 2, w -> z * 2]) } }
	#[inline(always)] pub const fn xzx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xzx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xzy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn xzy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn xzz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xzz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xzw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, z -> y * 2]) } }
	#[inline(always)] pub const fn xzw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, z -> y * 2]) } }
	#[inline(always)] pub const fn xwx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, w -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xwx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, w -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xwy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, w -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn xwy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, w -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn xwz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, w -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xwz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, w -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn xww(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, w -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn xww_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, w -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn yxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn yxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn yxz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn yxz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn yxw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn yxw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn yyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn yyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn yyz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn yyz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn yyw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn yyw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn yzx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn yzx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn yzy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn yzy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn yzz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 2, z -> z * 2]) } }
	#[inline(always)] pub const fn yzz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 2, z -> z * 2]) } }
	#[inline(always)] pub const fn yzw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 3]) } }
	#[inline(always)] pub const fn yzw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 3]) } }
	#[inline(always)] pub const fn ywx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, w -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn ywx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, w -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn ywy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, w -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn ywy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, w -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn ywz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, w -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn ywz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, w -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn yww(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, w -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn yww_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, w -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn zxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn zxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn zxz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn zxz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn zxw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, x -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn zxw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, x -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn zyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn zyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn zyz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn zyz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn zyw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, y -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn zyw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, y -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn zzx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zzx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn zzy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn zzy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn zzz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn zzz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn zzw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 1, z -> y * 2]) } }
	#[inline(always)] pub const fn zzw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 1, z -> y * 2]) } }
	#[inline(always)] pub const fn zwx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn zwx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn zwy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn zwy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn zwz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 2, z -> z * 2]) } }
	#[inline(always)] pub const fn zwz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 2, z -> z * 2]) } }
	#[inline(always)] pub const fn zww(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [z -> x * 2, w -> z * 2]) } }
	#[inline(always)] pub const fn zww_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [z -> x * 2, w -> z * 2]) } }
	#[inline(always)] pub const fn wxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn wxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn wxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn wxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn wxz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn wxz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, x -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn wxw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, x -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn wxw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, x -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn wyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn wyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn wyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn wyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn wyz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn wyz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, y -> y * 2]) } }
	#[inline(always)] pub const fn wyw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, y -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn wyw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, y -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn wzx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn wzx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, z -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn wzy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn wzy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, z -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn wzz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn wzz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, z -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn wzw(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, z -> y * 2]) } }
	#[inline(always)] pub const fn wzw_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, z -> y * 2]) } }
	#[inline(always)] pub const fn wwx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, w -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn wwx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, w -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn wwy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, w -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn wwy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, w -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn wwz(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, w -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn wwz_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, w -> y * 1, z -> z * 1]) } }
	#[inline(always)] pub const fn www(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [w -> x * 1, w -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn www_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [w -> x * 1, w -> y * 1, w -> z * 1]) } }
	#[inline(always)] pub const fn xxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn xxxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xxxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn xxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn xxyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 3]) } }
	#[inline(always)] pub const fn xxyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn xxzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xxzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xxzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xxzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn xxwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xxwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xxwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xxww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn xyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 3]) } }
	#[inline(always)] pub const fn xyxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn xyxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn xyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn xyyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 3]) } }
	#[inline(always)] pub const fn xyyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn xyzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 3, x -> w * 3]) } }
	#[inline(always)] pub const fn xyzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 3, y -> w * 3]) } }
	#[inline(always)] pub const fn xyzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 3, z -> w * 3]) } }
	#[inline(always)] pub const fn xyzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 4]) } }
	#[inline(always)] pub const fn xywx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, w -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xywy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, w -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn xywz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, w -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn xyww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, w -> z * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn xzxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xzxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn xzxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xzxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn xzyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xzyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xzyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn xzyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn xzzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xzzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xzzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xzzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn xzwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xzwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn xzwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn xzww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, z -> y * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn xwxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xwxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn xwxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xwxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn xwyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xwyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xwyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn xwyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn xwzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xwzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xwzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xwzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn xwwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xwwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn xwwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn xwww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, w -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn yxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn yxxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yxxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn yxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yxyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 3]) } }
	#[inline(always)] pub const fn yxyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn yxzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yxzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn yxzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yxzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn yxwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yxwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn yxwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yxww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn yyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn yyxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yyxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn yyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn yyyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn yyyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn yyzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yyzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yyzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn yyzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 3]) } }
	#[inline(always)] pub const fn yywx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yywy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn yywz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn yyww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn yzxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, x -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yzxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, x -> z * 3]) } }
	#[inline(always)] pub const fn yzxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, x -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn yzxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, x -> z * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn yzyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, y -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yzyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, y -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yzyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, y -> z * 3]) } }
	#[inline(always)] pub const fn yzyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, y -> z * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn yzzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, z -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yzzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, z -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yzzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, z -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn yzzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 2, z -> z * 3]) } }
	#[inline(always)] pub const fn yzwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 3, x -> w * 3]) } }
	#[inline(always)] pub const fn yzwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 3, y -> w * 3]) } }
	#[inline(always)] pub const fn yzwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 3, z -> w * 3]) } }
	#[inline(always)] pub const fn yzww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 3, w -> w * 3]) } }
	#[inline(always)] pub const fn ywxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn ywxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn ywxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn ywxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn ywyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn ywyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn ywyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn ywyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn ywzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn ywzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn ywzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn ywzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn ywwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn ywwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn ywwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn ywww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, w -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn zxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn zxxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zxxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn zxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn zxyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 3]) } }
	#[inline(always)] pub const fn zxyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn zxzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zxzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zxzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zxzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn zxwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zxwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zxwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zxww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, x -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn zyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn zyxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zyxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn zyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zyyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn zyyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn zyzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zyzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn zyzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn zyzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 3]) } }
	#[inline(always)] pub const fn zywx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zywy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zywz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zyww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, y -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn zzxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zzxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn zzxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zzxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn zzyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zzyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zzyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn zzyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn zzzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn zzzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn zzzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn zzzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn zzwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zzwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn zzwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn zzww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 1, z -> y * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn zwxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, x -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zwxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, x -> z * 3]) } }
	#[inline(always)] pub const fn zwxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, x -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn zwxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, x -> z * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn zwyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, y -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zwyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, y -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn zwyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, y -> z * 3]) } }
	#[inline(always)] pub const fn zwyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, y -> z * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn zwzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, z -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zwzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, z -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn zwzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, z -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn zwzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, z -> z * 3]) } }
	#[inline(always)] pub const fn zwwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, w -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn zwwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, w -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn zwwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, w -> z * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn zwww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [z -> x * 2, w -> z * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn wxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn wxxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wxxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn wxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn wxyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 3]) } }
	#[inline(always)] pub const fn wxyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn wxzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wxzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wxzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wxzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn wxwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wxwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wxwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wxww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, x -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn wyxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wyxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wyyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn wyyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wyzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn wyzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn wyzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn wyzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 3]) } }
	#[inline(always)] pub const fn wywx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wywy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wywz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wyww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, y -> y * 1, w -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wzxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wzxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn wzxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wzxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wzyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wzyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wzyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn wzyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wzzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wzzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wzzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wzzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn wzwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn wzwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn wzwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 2, z -> w * 2]) } }
	#[inline(always)] pub const fn wzww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, z -> y * 2, w -> w * 2]) } }
	#[inline(always)] pub const fn wwxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wwxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn wwxz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, x -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wwxw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, x -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wwyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wwyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, y -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wwyz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, y -> z * 2]) } }
	#[inline(always)] pub const fn wwyw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, y -> z * 1, w -> w * 1]) } }
	#[inline(always)] pub const fn wwzx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, z -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wwzy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, z -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wwzz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, z -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wwzw(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, z -> z * 2]) } }
	#[inline(always)] pub const fn wwwx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, w -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn wwwy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, w -> z * 1, y -> w * 1]) } }
	#[inline(always)] pub const fn wwwz(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, w -> z * 1, z -> w * 1]) } }
	#[inline(always)] pub const fn wwww(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [w -> x * 1, w -> y * 1, w -> z * 1, w -> w * 1]) } }
}