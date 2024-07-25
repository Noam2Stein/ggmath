use std::fmt::Display;
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4<C: Component> {
	pub x: C,
	pub y: C,
	pub z: C,
	pub w: C,
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

impl<C: Component> Display for Vec4<C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
	}
}

impl<C: Component> Vec4<C> {
	#[inline(always)]
	pub fn xx(self) -> Vec2<C> {
		Vec2::new(self.x, self.x)
	}
	#[inline(always)]
	pub fn yx(self) -> Vec2<C> {
		Vec2::new(self.y, self.x)
	}
	#[inline(always)]
	pub fn zx(self) -> Vec2<C> {
		Vec2::new(self.z, self.x)
	}
	#[inline(always)]
	pub fn wx(self) -> Vec2<C> {
		Vec2::new(self.w, self.x)
	}
	#[inline(always)]
	pub fn xy(self) -> Vec2<C> {
		Vec2::new(self.x, self.y)
	}
	#[inline(always)]
	pub fn yy(self) -> Vec2<C> {
		Vec2::new(self.y, self.y)
	}
	#[inline(always)]
	pub fn zy(self) -> Vec2<C> {
		Vec2::new(self.z, self.y)
	}
	#[inline(always)]
	pub fn wy(self) -> Vec2<C> {
		Vec2::new(self.w, self.y)
	}
	#[inline(always)]
	pub fn xz(self) -> Vec2<C> {
		Vec2::new(self.x, self.z)
	}
	#[inline(always)]
	pub fn yz(self) -> Vec2<C> {
		Vec2::new(self.y, self.z)
	}
	#[inline(always)]
	pub fn zz(self) -> Vec2<C> {
		Vec2::new(self.z, self.z)
	}
	#[inline(always)]
	pub fn wz(self) -> Vec2<C> {
		Vec2::new(self.w, self.z)
	}
	#[inline(always)]
	pub fn xw(self) -> Vec2<C> {
		Vec2::new(self.x, self.w)
	}
	#[inline(always)]
	pub fn yw(self) -> Vec2<C> {
		Vec2::new(self.y, self.w)
	}
	#[inline(always)]
	pub fn zw(self) -> Vec2<C> {
		Vec2::new(self.z, self.w)
	}
	#[inline(always)]
	pub fn ww(self) -> Vec2<C> {
		Vec2::new(self.w, self.w)
	}
	#[inline(always)]
	pub fn xxx(self) -> Vec3<C> {
		Vec3::new(self.x, self.x, self.x)
	}
	#[inline(always)]
	pub fn xxx_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.x, self.x)
	}
	#[inline(always)]
	pub fn yxx(self) -> Vec3<C> {
		Vec3::new(self.y, self.x, self.x)
	}
	#[inline(always)]
	pub fn yxx_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.x, self.x)
	}
	#[inline(always)]
	pub fn zxx(self) -> Vec3<C> {
		Vec3::new(self.z, self.x, self.x)
	}
	#[inline(always)]
	pub fn zxx_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.x, self.x)
	}
	#[inline(always)]
	pub fn wxx(self) -> Vec3<C> {
		Vec3::new(self.w, self.x, self.x)
	}
	#[inline(always)]
	pub fn wxx_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.x, self.x)
	}
	#[inline(always)]
	pub fn xyx(self) -> Vec3<C> {
		Vec3::new(self.x, self.y, self.x)
	}
	#[inline(always)]
	pub fn xyx_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.y, self.x)
	}
	#[inline(always)]
	pub fn yyx(self) -> Vec3<C> {
		Vec3::new(self.y, self.y, self.x)
	}
	#[inline(always)]
	pub fn yyx_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.y, self.x)
	}
	#[inline(always)]
	pub fn zyx(self) -> Vec3<C> {
		Vec3::new(self.z, self.y, self.x)
	}
	#[inline(always)]
	pub fn zyx_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.y, self.x)
	}
	#[inline(always)]
	pub fn wyx(self) -> Vec3<C> {
		Vec3::new(self.w, self.y, self.x)
	}
	#[inline(always)]
	pub fn wyx_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.y, self.x)
	}
	#[inline(always)]
	pub fn xzx(self) -> Vec3<C> {
		Vec3::new(self.x, self.z, self.x)
	}
	#[inline(always)]
	pub fn xzx_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.z, self.x)
	}
	#[inline(always)]
	pub fn yzx(self) -> Vec3<C> {
		Vec3::new(self.y, self.z, self.x)
	}
	#[inline(always)]
	pub fn yzx_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.z, self.x)
	}
	#[inline(always)]
	pub fn zzx(self) -> Vec3<C> {
		Vec3::new(self.z, self.z, self.x)
	}
	#[inline(always)]
	pub fn zzx_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.z, self.x)
	}
	#[inline(always)]
	pub fn wzx(self) -> Vec3<C> {
		Vec3::new(self.w, self.z, self.x)
	}
	#[inline(always)]
	pub fn wzx_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.z, self.x)
	}
	#[inline(always)]
	pub fn xwx(self) -> Vec3<C> {
		Vec3::new(self.x, self.w, self.x)
	}
	#[inline(always)]
	pub fn xwx_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.w, self.x)
	}
	#[inline(always)]
	pub fn ywx(self) -> Vec3<C> {
		Vec3::new(self.y, self.w, self.x)
	}
	#[inline(always)]
	pub fn ywx_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.w, self.x)
	}
	#[inline(always)]
	pub fn zwx(self) -> Vec3<C> {
		Vec3::new(self.z, self.w, self.x)
	}
	#[inline(always)]
	pub fn zwx_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.w, self.x)
	}
	#[inline(always)]
	pub fn wwx(self) -> Vec3<C> {
		Vec3::new(self.w, self.w, self.x)
	}
	#[inline(always)]
	pub fn wwx_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.w, self.x)
	}
	#[inline(always)]
	pub fn xxy(self) -> Vec3<C> {
		Vec3::new(self.x, self.x, self.y)
	}
	#[inline(always)]
	pub fn xxy_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.x, self.y)
	}
	#[inline(always)]
	pub fn yxy(self) -> Vec3<C> {
		Vec3::new(self.y, self.x, self.y)
	}
	#[inline(always)]
	pub fn yxy_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.x, self.y)
	}
	#[inline(always)]
	pub fn zxy(self) -> Vec3<C> {
		Vec3::new(self.z, self.x, self.y)
	}
	#[inline(always)]
	pub fn zxy_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.x, self.y)
	}
	#[inline(always)]
	pub fn wxy(self) -> Vec3<C> {
		Vec3::new(self.w, self.x, self.y)
	}
	#[inline(always)]
	pub fn wxy_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.x, self.y)
	}
	#[inline(always)]
	pub fn xyy(self) -> Vec3<C> {
		Vec3::new(self.x, self.y, self.y)
	}
	#[inline(always)]
	pub fn xyy_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.y, self.y)
	}
	#[inline(always)]
	pub fn yyy(self) -> Vec3<C> {
		Vec3::new(self.y, self.y, self.y)
	}
	#[inline(always)]
	pub fn yyy_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.y, self.y)
	}
	#[inline(always)]
	pub fn zyy(self) -> Vec3<C> {
		Vec3::new(self.z, self.y, self.y)
	}
	#[inline(always)]
	pub fn zyy_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.y, self.y)
	}
	#[inline(always)]
	pub fn wyy(self) -> Vec3<C> {
		Vec3::new(self.w, self.y, self.y)
	}
	#[inline(always)]
	pub fn wyy_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.y, self.y)
	}
	#[inline(always)]
	pub fn xzy(self) -> Vec3<C> {
		Vec3::new(self.x, self.z, self.y)
	}
	#[inline(always)]
	pub fn xzy_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.z, self.y)
	}
	#[inline(always)]
	pub fn yzy(self) -> Vec3<C> {
		Vec3::new(self.y, self.z, self.y)
	}
	#[inline(always)]
	pub fn yzy_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.z, self.y)
	}
	#[inline(always)]
	pub fn zzy(self) -> Vec3<C> {
		Vec3::new(self.z, self.z, self.y)
	}
	#[inline(always)]
	pub fn zzy_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.z, self.y)
	}
	#[inline(always)]
	pub fn wzy(self) -> Vec3<C> {
		Vec3::new(self.w, self.z, self.y)
	}
	#[inline(always)]
	pub fn wzy_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.z, self.y)
	}
	#[inline(always)]
	pub fn xwy(self) -> Vec3<C> {
		Vec3::new(self.x, self.w, self.y)
	}
	#[inline(always)]
	pub fn xwy_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.w, self.y)
	}
	#[inline(always)]
	pub fn ywy(self) -> Vec3<C> {
		Vec3::new(self.y, self.w, self.y)
	}
	#[inline(always)]
	pub fn ywy_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.w, self.y)
	}
	#[inline(always)]
	pub fn zwy(self) -> Vec3<C> {
		Vec3::new(self.z, self.w, self.y)
	}
	#[inline(always)]
	pub fn zwy_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.w, self.y)
	}
	#[inline(always)]
	pub fn wwy(self) -> Vec3<C> {
		Vec3::new(self.w, self.w, self.y)
	}
	#[inline(always)]
	pub fn wwy_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.w, self.y)
	}
	#[inline(always)]
	pub fn xxz(self) -> Vec3<C> {
		Vec3::new(self.x, self.x, self.z)
	}
	#[inline(always)]
	pub fn xxz_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.x, self.z)
	}
	#[inline(always)]
	pub fn yxz(self) -> Vec3<C> {
		Vec3::new(self.y, self.x, self.z)
	}
	#[inline(always)]
	pub fn yxz_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.x, self.z)
	}
	#[inline(always)]
	pub fn zxz(self) -> Vec3<C> {
		Vec3::new(self.z, self.x, self.z)
	}
	#[inline(always)]
	pub fn zxz_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.x, self.z)
	}
	#[inline(always)]
	pub fn wxz(self) -> Vec3<C> {
		Vec3::new(self.w, self.x, self.z)
	}
	#[inline(always)]
	pub fn wxz_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.x, self.z)
	}
	#[inline(always)]
	pub fn xyz(self) -> Vec3<C> {
		Vec3::new(self.x, self.y, self.z)
	}
	#[inline(always)]
	pub fn xyz_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.y, self.z)
	}
	#[inline(always)]
	pub fn yyz(self) -> Vec3<C> {
		Vec3::new(self.y, self.y, self.z)
	}
	#[inline(always)]
	pub fn yyz_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.y, self.z)
	}
	#[inline(always)]
	pub fn zyz(self) -> Vec3<C> {
		Vec3::new(self.z, self.y, self.z)
	}
	#[inline(always)]
	pub fn zyz_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.y, self.z)
	}
	#[inline(always)]
	pub fn wyz(self) -> Vec3<C> {
		Vec3::new(self.w, self.y, self.z)
	}
	#[inline(always)]
	pub fn wyz_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.y, self.z)
	}
	#[inline(always)]
	pub fn xzz(self) -> Vec3<C> {
		Vec3::new(self.x, self.z, self.z)
	}
	#[inline(always)]
	pub fn xzz_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.z, self.z)
	}
	#[inline(always)]
	pub fn yzz(self) -> Vec3<C> {
		Vec3::new(self.y, self.z, self.z)
	}
	#[inline(always)]
	pub fn yzz_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.z, self.z)
	}
	#[inline(always)]
	pub fn zzz(self) -> Vec3<C> {
		Vec3::new(self.z, self.z, self.z)
	}
	#[inline(always)]
	pub fn zzz_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.z, self.z)
	}
	#[inline(always)]
	pub fn wzz(self) -> Vec3<C> {
		Vec3::new(self.w, self.z, self.z)
	}
	#[inline(always)]
	pub fn wzz_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.z, self.z)
	}
	#[inline(always)]
	pub fn xwz(self) -> Vec3<C> {
		Vec3::new(self.x, self.w, self.z)
	}
	#[inline(always)]
	pub fn xwz_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.w, self.z)
	}
	#[inline(always)]
	pub fn ywz(self) -> Vec3<C> {
		Vec3::new(self.y, self.w, self.z)
	}
	#[inline(always)]
	pub fn ywz_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.w, self.z)
	}
	#[inline(always)]
	pub fn zwz(self) -> Vec3<C> {
		Vec3::new(self.z, self.w, self.z)
	}
	#[inline(always)]
	pub fn zwz_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.w, self.z)
	}
	#[inline(always)]
	pub fn wwz(self) -> Vec3<C> {
		Vec3::new(self.w, self.w, self.z)
	}
	#[inline(always)]
	pub fn wwz_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.w, self.z)
	}
	#[inline(always)]
	pub fn xxw(self) -> Vec3<C> {
		Vec3::new(self.x, self.x, self.w)
	}
	#[inline(always)]
	pub fn xxw_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.x, self.w)
	}
	#[inline(always)]
	pub fn yxw(self) -> Vec3<C> {
		Vec3::new(self.y, self.x, self.w)
	}
	#[inline(always)]
	pub fn yxw_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.x, self.w)
	}
	#[inline(always)]
	pub fn zxw(self) -> Vec3<C> {
		Vec3::new(self.z, self.x, self.w)
	}
	#[inline(always)]
	pub fn zxw_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.x, self.w)
	}
	#[inline(always)]
	pub fn wxw(self) -> Vec3<C> {
		Vec3::new(self.w, self.x, self.w)
	}
	#[inline(always)]
	pub fn wxw_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.x, self.w)
	}
	#[inline(always)]
	pub fn xyw(self) -> Vec3<C> {
		Vec3::new(self.x, self.y, self.w)
	}
	#[inline(always)]
	pub fn xyw_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.y, self.w)
	}
	#[inline(always)]
	pub fn yyw(self) -> Vec3<C> {
		Vec3::new(self.y, self.y, self.w)
	}
	#[inline(always)]
	pub fn yyw_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.y, self.w)
	}
	#[inline(always)]
	pub fn zyw(self) -> Vec3<C> {
		Vec3::new(self.z, self.y, self.w)
	}
	#[inline(always)]
	pub fn zyw_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.y, self.w)
	}
	#[inline(always)]
	pub fn wyw(self) -> Vec3<C> {
		Vec3::new(self.w, self.y, self.w)
	}
	#[inline(always)]
	pub fn wyw_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.y, self.w)
	}
	#[inline(always)]
	pub fn xzw(self) -> Vec3<C> {
		Vec3::new(self.x, self.z, self.w)
	}
	#[inline(always)]
	pub fn xzw_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.z, self.w)
	}
	#[inline(always)]
	pub fn yzw(self) -> Vec3<C> {
		Vec3::new(self.y, self.z, self.w)
	}
	#[inline(always)]
	pub fn yzw_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.z, self.w)
	}
	#[inline(always)]
	pub fn zzw(self) -> Vec3<C> {
		Vec3::new(self.z, self.z, self.w)
	}
	#[inline(always)]
	pub fn zzw_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.z, self.w)
	}
	#[inline(always)]
	pub fn wzw(self) -> Vec3<C> {
		Vec3::new(self.w, self.z, self.w)
	}
	#[inline(always)]
	pub fn wzw_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.z, self.w)
	}
	#[inline(always)]
	pub fn xww(self) -> Vec3<C> {
		Vec3::new(self.x, self.w, self.w)
	}
	#[inline(always)]
	pub fn xww_a(self) -> Vec3A<C> {
		Vec3A::new(self.x, self.w, self.w)
	}
	#[inline(always)]
	pub fn yww(self) -> Vec3<C> {
		Vec3::new(self.y, self.w, self.w)
	}
	#[inline(always)]
	pub fn yww_a(self) -> Vec3A<C> {
		Vec3A::new(self.y, self.w, self.w)
	}
	#[inline(always)]
	pub fn zww(self) -> Vec3<C> {
		Vec3::new(self.z, self.w, self.w)
	}
	#[inline(always)]
	pub fn zww_a(self) -> Vec3A<C> {
		Vec3A::new(self.z, self.w, self.w)
	}
	#[inline(always)]
	pub fn www(self) -> Vec3<C> {
		Vec3::new(self.w, self.w, self.w)
	}
	#[inline(always)]
	pub fn www_a(self) -> Vec3A<C> {
		Vec3A::new(self.w, self.w, self.w)
	}
	#[inline(always)]
	pub fn xxxx(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.x)
	}
	#[inline(always)]
	pub fn yxxx(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.x)
	}
	#[inline(always)]
	pub fn zxxx(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.x, self.x)
	}
	#[inline(always)]
	pub fn wxxx(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.x, self.x)
	}
	#[inline(always)]
	pub fn xyxx(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.x)
	}
	#[inline(always)]
	pub fn yyxx(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.x)
	}
	#[inline(always)]
	pub fn zyxx(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.x, self.x)
	}
	#[inline(always)]
	pub fn wyxx(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.x, self.x)
	}
	#[inline(always)]
	pub fn xzxx(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.x, self.x)
	}
	#[inline(always)]
	pub fn yzxx(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.x, self.x)
	}
	#[inline(always)]
	pub fn zzxx(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.x, self.x)
	}
	#[inline(always)]
	pub fn wzxx(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.x, self.x)
	}
	#[inline(always)]
	pub fn xwxx(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.x, self.x)
	}
	#[inline(always)]
	pub fn ywxx(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.x, self.x)
	}
	#[inline(always)]
	pub fn zwxx(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.x, self.x)
	}
	#[inline(always)]
	pub fn wwxx(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.x, self.x)
	}
	#[inline(always)]
	pub fn xxyx(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.x)
	}
	#[inline(always)]
	pub fn yxyx(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.x)
	}
	#[inline(always)]
	pub fn zxyx(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.y, self.x)
	}
	#[inline(always)]
	pub fn wxyx(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.y, self.x)
	}
	#[inline(always)]
	pub fn xyyx(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.x)
	}
	#[inline(always)]
	pub fn yyyx(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.x)
	}
	#[inline(always)]
	pub fn zyyx(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.y, self.x)
	}
	#[inline(always)]
	pub fn wyyx(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.y, self.x)
	}
	#[inline(always)]
	pub fn xzyx(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.y, self.x)
	}
	#[inline(always)]
	pub fn yzyx(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.y, self.x)
	}
	#[inline(always)]
	pub fn zzyx(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.y, self.x)
	}
	#[inline(always)]
	pub fn wzyx(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.y, self.x)
	}
	#[inline(always)]
	pub fn xwyx(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.y, self.x)
	}
	#[inline(always)]
	pub fn ywyx(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.y, self.x)
	}
	#[inline(always)]
	pub fn zwyx(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.y, self.x)
	}
	#[inline(always)]
	pub fn wwyx(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.y, self.x)
	}
	#[inline(always)]
	pub fn xxzx(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.z, self.x)
	}
	#[inline(always)]
	pub fn yxzx(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.z, self.x)
	}
	#[inline(always)]
	pub fn zxzx(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.z, self.x)
	}
	#[inline(always)]
	pub fn wxzx(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.z, self.x)
	}
	#[inline(always)]
	pub fn xyzx(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.z, self.x)
	}
	#[inline(always)]
	pub fn yyzx(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.z, self.x)
	}
	#[inline(always)]
	pub fn zyzx(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.z, self.x)
	}
	#[inline(always)]
	pub fn wyzx(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.z, self.x)
	}
	#[inline(always)]
	pub fn xzzx(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.z, self.x)
	}
	#[inline(always)]
	pub fn yzzx(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.z, self.x)
	}
	#[inline(always)]
	pub fn zzzx(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.z, self.x)
	}
	#[inline(always)]
	pub fn wzzx(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.z, self.x)
	}
	#[inline(always)]
	pub fn xwzx(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.z, self.x)
	}
	#[inline(always)]
	pub fn ywzx(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.z, self.x)
	}
	#[inline(always)]
	pub fn zwzx(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.z, self.x)
	}
	#[inline(always)]
	pub fn wwzx(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.z, self.x)
	}
	#[inline(always)]
	pub fn xxwx(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.w, self.x)
	}
	#[inline(always)]
	pub fn yxwx(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.w, self.x)
	}
	#[inline(always)]
	pub fn zxwx(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.w, self.x)
	}
	#[inline(always)]
	pub fn wxwx(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.w, self.x)
	}
	#[inline(always)]
	pub fn xywx(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.w, self.x)
	}
	#[inline(always)]
	pub fn yywx(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.w, self.x)
	}
	#[inline(always)]
	pub fn zywx(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.w, self.x)
	}
	#[inline(always)]
	pub fn wywx(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.w, self.x)
	}
	#[inline(always)]
	pub fn xzwx(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.w, self.x)
	}
	#[inline(always)]
	pub fn yzwx(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.w, self.x)
	}
	#[inline(always)]
	pub fn zzwx(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.w, self.x)
	}
	#[inline(always)]
	pub fn wzwx(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.w, self.x)
	}
	#[inline(always)]
	pub fn xwwx(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.w, self.x)
	}
	#[inline(always)]
	pub fn ywwx(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.w, self.x)
	}
	#[inline(always)]
	pub fn zwwx(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.w, self.x)
	}
	#[inline(always)]
	pub fn wwwx(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.w, self.x)
	}
	#[inline(always)]
	pub fn xxxy(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.y)
	}
	#[inline(always)]
	pub fn yxxy(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.y)
	}
	#[inline(always)]
	pub fn zxxy(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.x, self.y)
	}
	#[inline(always)]
	pub fn wxxy(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.x, self.y)
	}
	#[inline(always)]
	pub fn xyxy(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.y)
	}
	#[inline(always)]
	pub fn yyxy(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.y)
	}
	#[inline(always)]
	pub fn zyxy(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.x, self.y)
	}
	#[inline(always)]
	pub fn wyxy(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.x, self.y)
	}
	#[inline(always)]
	pub fn xzxy(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.x, self.y)
	}
	#[inline(always)]
	pub fn yzxy(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.x, self.y)
	}
	#[inline(always)]
	pub fn zzxy(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.x, self.y)
	}
	#[inline(always)]
	pub fn wzxy(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.x, self.y)
	}
	#[inline(always)]
	pub fn xwxy(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.x, self.y)
	}
	#[inline(always)]
	pub fn ywxy(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.x, self.y)
	}
	#[inline(always)]
	pub fn zwxy(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.x, self.y)
	}
	#[inline(always)]
	pub fn wwxy(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.x, self.y)
	}
	#[inline(always)]
	pub fn xxyy(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.y)
	}
	#[inline(always)]
	pub fn yxyy(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.y)
	}
	#[inline(always)]
	pub fn zxyy(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.y, self.y)
	}
	#[inline(always)]
	pub fn wxyy(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.y, self.y)
	}
	#[inline(always)]
	pub fn xyyy(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.y)
	}
	#[inline(always)]
	pub fn yyyy(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.y)
	}
	#[inline(always)]
	pub fn zyyy(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.y, self.y)
	}
	#[inline(always)]
	pub fn wyyy(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.y, self.y)
	}
	#[inline(always)]
	pub fn xzyy(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.y, self.y)
	}
	#[inline(always)]
	pub fn yzyy(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.y, self.y)
	}
	#[inline(always)]
	pub fn zzyy(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.y, self.y)
	}
	#[inline(always)]
	pub fn wzyy(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.y, self.y)
	}
	#[inline(always)]
	pub fn xwyy(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.y, self.y)
	}
	#[inline(always)]
	pub fn ywyy(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.y, self.y)
	}
	#[inline(always)]
	pub fn zwyy(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.y, self.y)
	}
	#[inline(always)]
	pub fn wwyy(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.y, self.y)
	}
	#[inline(always)]
	pub fn xxzy(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.z, self.y)
	}
	#[inline(always)]
	pub fn yxzy(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.z, self.y)
	}
	#[inline(always)]
	pub fn zxzy(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.z, self.y)
	}
	#[inline(always)]
	pub fn wxzy(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.z, self.y)
	}
	#[inline(always)]
	pub fn xyzy(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.z, self.y)
	}
	#[inline(always)]
	pub fn yyzy(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.z, self.y)
	}
	#[inline(always)]
	pub fn zyzy(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.z, self.y)
	}
	#[inline(always)]
	pub fn wyzy(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.z, self.y)
	}
	#[inline(always)]
	pub fn xzzy(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.z, self.y)
	}
	#[inline(always)]
	pub fn yzzy(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.z, self.y)
	}
	#[inline(always)]
	pub fn zzzy(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.z, self.y)
	}
	#[inline(always)]
	pub fn wzzy(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.z, self.y)
	}
	#[inline(always)]
	pub fn xwzy(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.z, self.y)
	}
	#[inline(always)]
	pub fn ywzy(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.z, self.y)
	}
	#[inline(always)]
	pub fn zwzy(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.z, self.y)
	}
	#[inline(always)]
	pub fn wwzy(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.z, self.y)
	}
	#[inline(always)]
	pub fn xxwy(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.w, self.y)
	}
	#[inline(always)]
	pub fn yxwy(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.w, self.y)
	}
	#[inline(always)]
	pub fn zxwy(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.w, self.y)
	}
	#[inline(always)]
	pub fn wxwy(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.w, self.y)
	}
	#[inline(always)]
	pub fn xywy(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.w, self.y)
	}
	#[inline(always)]
	pub fn yywy(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.w, self.y)
	}
	#[inline(always)]
	pub fn zywy(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.w, self.y)
	}
	#[inline(always)]
	pub fn wywy(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.w, self.y)
	}
	#[inline(always)]
	pub fn xzwy(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.w, self.y)
	}
	#[inline(always)]
	pub fn yzwy(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.w, self.y)
	}
	#[inline(always)]
	pub fn zzwy(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.w, self.y)
	}
	#[inline(always)]
	pub fn wzwy(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.w, self.y)
	}
	#[inline(always)]
	pub fn xwwy(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.w, self.y)
	}
	#[inline(always)]
	pub fn ywwy(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.w, self.y)
	}
	#[inline(always)]
	pub fn zwwy(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.w, self.y)
	}
	#[inline(always)]
	pub fn wwwy(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.w, self.y)
	}
	#[inline(always)]
	pub fn xxxz(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.z)
	}
	#[inline(always)]
	pub fn yxxz(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.z)
	}
	#[inline(always)]
	pub fn zxxz(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.x, self.z)
	}
	#[inline(always)]
	pub fn wxxz(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.x, self.z)
	}
	#[inline(always)]
	pub fn xyxz(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.z)
	}
	#[inline(always)]
	pub fn yyxz(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.z)
	}
	#[inline(always)]
	pub fn zyxz(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.x, self.z)
	}
	#[inline(always)]
	pub fn wyxz(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.x, self.z)
	}
	#[inline(always)]
	pub fn xzxz(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.x, self.z)
	}
	#[inline(always)]
	pub fn yzxz(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.x, self.z)
	}
	#[inline(always)]
	pub fn zzxz(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.x, self.z)
	}
	#[inline(always)]
	pub fn wzxz(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.x, self.z)
	}
	#[inline(always)]
	pub fn xwxz(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.x, self.z)
	}
	#[inline(always)]
	pub fn ywxz(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.x, self.z)
	}
	#[inline(always)]
	pub fn zwxz(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.x, self.z)
	}
	#[inline(always)]
	pub fn wwxz(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.x, self.z)
	}
	#[inline(always)]
	pub fn xxyz(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.z)
	}
	#[inline(always)]
	pub fn yxyz(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.z)
	}
	#[inline(always)]
	pub fn zxyz(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.y, self.z)
	}
	#[inline(always)]
	pub fn wxyz(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.y, self.z)
	}
	#[inline(always)]
	pub fn xyyz(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.z)
	}
	#[inline(always)]
	pub fn yyyz(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.z)
	}
	#[inline(always)]
	pub fn zyyz(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.y, self.z)
	}
	#[inline(always)]
	pub fn wyyz(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.y, self.z)
	}
	#[inline(always)]
	pub fn xzyz(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.y, self.z)
	}
	#[inline(always)]
	pub fn yzyz(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.y, self.z)
	}
	#[inline(always)]
	pub fn zzyz(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.y, self.z)
	}
	#[inline(always)]
	pub fn wzyz(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.y, self.z)
	}
	#[inline(always)]
	pub fn xwyz(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.y, self.z)
	}
	#[inline(always)]
	pub fn ywyz(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.y, self.z)
	}
	#[inline(always)]
	pub fn zwyz(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.y, self.z)
	}
	#[inline(always)]
	pub fn wwyz(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.y, self.z)
	}
	#[inline(always)]
	pub fn xxzz(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.z, self.z)
	}
	#[inline(always)]
	pub fn yxzz(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.z, self.z)
	}
	#[inline(always)]
	pub fn zxzz(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.z, self.z)
	}
	#[inline(always)]
	pub fn wxzz(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.z, self.z)
	}
	#[inline(always)]
	pub fn xyzz(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.z, self.z)
	}
	#[inline(always)]
	pub fn yyzz(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.z, self.z)
	}
	#[inline(always)]
	pub fn zyzz(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.z, self.z)
	}
	#[inline(always)]
	pub fn wyzz(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.z, self.z)
	}
	#[inline(always)]
	pub fn xzzz(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.z, self.z)
	}
	#[inline(always)]
	pub fn yzzz(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.z, self.z)
	}
	#[inline(always)]
	pub fn zzzz(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.z, self.z)
	}
	#[inline(always)]
	pub fn wzzz(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.z, self.z)
	}
	#[inline(always)]
	pub fn xwzz(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.z, self.z)
	}
	#[inline(always)]
	pub fn ywzz(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.z, self.z)
	}
	#[inline(always)]
	pub fn zwzz(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.z, self.z)
	}
	#[inline(always)]
	pub fn wwzz(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.z, self.z)
	}
	#[inline(always)]
	pub fn xxwz(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.w, self.z)
	}
	#[inline(always)]
	pub fn yxwz(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.w, self.z)
	}
	#[inline(always)]
	pub fn zxwz(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.w, self.z)
	}
	#[inline(always)]
	pub fn wxwz(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.w, self.z)
	}
	#[inline(always)]
	pub fn xywz(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.w, self.z)
	}
	#[inline(always)]
	pub fn yywz(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.w, self.z)
	}
	#[inline(always)]
	pub fn zywz(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.w, self.z)
	}
	#[inline(always)]
	pub fn wywz(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.w, self.z)
	}
	#[inline(always)]
	pub fn xzwz(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.w, self.z)
	}
	#[inline(always)]
	pub fn yzwz(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.w, self.z)
	}
	#[inline(always)]
	pub fn zzwz(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.w, self.z)
	}
	#[inline(always)]
	pub fn wzwz(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.w, self.z)
	}
	#[inline(always)]
	pub fn xwwz(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.w, self.z)
	}
	#[inline(always)]
	pub fn ywwz(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.w, self.z)
	}
	#[inline(always)]
	pub fn zwwz(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.w, self.z)
	}
	#[inline(always)]
	pub fn wwwz(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.w, self.z)
	}
	#[inline(always)]
	pub fn xxxw(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.w)
	}
	#[inline(always)]
	pub fn yxxw(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.w)
	}
	#[inline(always)]
	pub fn zxxw(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.x, self.w)
	}
	#[inline(always)]
	pub fn wxxw(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.x, self.w)
	}
	#[inline(always)]
	pub fn xyxw(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.w)
	}
	#[inline(always)]
	pub fn yyxw(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.w)
	}
	#[inline(always)]
	pub fn zyxw(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.x, self.w)
	}
	#[inline(always)]
	pub fn wyxw(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.x, self.w)
	}
	#[inline(always)]
	pub fn xzxw(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.x, self.w)
	}
	#[inline(always)]
	pub fn yzxw(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.x, self.w)
	}
	#[inline(always)]
	pub fn zzxw(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.x, self.w)
	}
	#[inline(always)]
	pub fn wzxw(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.x, self.w)
	}
	#[inline(always)]
	pub fn xwxw(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.x, self.w)
	}
	#[inline(always)]
	pub fn ywxw(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.x, self.w)
	}
	#[inline(always)]
	pub fn zwxw(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.x, self.w)
	}
	#[inline(always)]
	pub fn wwxw(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.x, self.w)
	}
	#[inline(always)]
	pub fn xxyw(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.w)
	}
	#[inline(always)]
	pub fn yxyw(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.w)
	}
	#[inline(always)]
	pub fn zxyw(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.y, self.w)
	}
	#[inline(always)]
	pub fn wxyw(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.y, self.w)
	}
	#[inline(always)]
	pub fn xyyw(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.w)
	}
	#[inline(always)]
	pub fn yyyw(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.w)
	}
	#[inline(always)]
	pub fn zyyw(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.y, self.w)
	}
	#[inline(always)]
	pub fn wyyw(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.y, self.w)
	}
	#[inline(always)]
	pub fn xzyw(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.y, self.w)
	}
	#[inline(always)]
	pub fn yzyw(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.y, self.w)
	}
	#[inline(always)]
	pub fn zzyw(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.y, self.w)
	}
	#[inline(always)]
	pub fn wzyw(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.y, self.w)
	}
	#[inline(always)]
	pub fn xwyw(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.y, self.w)
	}
	#[inline(always)]
	pub fn ywyw(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.y, self.w)
	}
	#[inline(always)]
	pub fn zwyw(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.y, self.w)
	}
	#[inline(always)]
	pub fn wwyw(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.y, self.w)
	}
	#[inline(always)]
	pub fn xxzw(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.z, self.w)
	}
	#[inline(always)]
	pub fn yxzw(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.z, self.w)
	}
	#[inline(always)]
	pub fn zxzw(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.z, self.w)
	}
	#[inline(always)]
	pub fn wxzw(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.z, self.w)
	}
	#[inline(always)]
	pub fn xyzw(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.z, self.w)
	}
	#[inline(always)]
	pub fn yyzw(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.z, self.w)
	}
	#[inline(always)]
	pub fn zyzw(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.z, self.w)
	}
	#[inline(always)]
	pub fn wyzw(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.z, self.w)
	}
	#[inline(always)]
	pub fn xzzw(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.z, self.w)
	}
	#[inline(always)]
	pub fn yzzw(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.z, self.w)
	}
	#[inline(always)]
	pub fn zzzw(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.z, self.w)
	}
	#[inline(always)]
	pub fn wzzw(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.z, self.w)
	}
	#[inline(always)]
	pub fn xwzw(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.z, self.w)
	}
	#[inline(always)]
	pub fn ywzw(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.z, self.w)
	}
	#[inline(always)]
	pub fn zwzw(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.z, self.w)
	}
	#[inline(always)]
	pub fn wwzw(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.z, self.w)
	}
	#[inline(always)]
	pub fn xxww(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.w, self.w)
	}
	#[inline(always)]
	pub fn yxww(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.w, self.w)
	}
	#[inline(always)]
	pub fn zxww(self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.w, self.w)
	}
	#[inline(always)]
	pub fn wxww(self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.w, self.w)
	}
	#[inline(always)]
	pub fn xyww(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.w, self.w)
	}
	#[inline(always)]
	pub fn yyww(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.w, self.w)
	}
	#[inline(always)]
	pub fn zyww(self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.w, self.w)
	}
	#[inline(always)]
	pub fn wyww(self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.w, self.w)
	}
	#[inline(always)]
	pub fn xzww(self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.w, self.w)
	}
	#[inline(always)]
	pub fn yzww(self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.w, self.w)
	}
	#[inline(always)]
	pub fn zzww(self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.w, self.w)
	}
	#[inline(always)]
	pub fn wzww(self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.w, self.w)
	}
	#[inline(always)]
	pub fn xwww(self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.w, self.w)
	}
	#[inline(always)]
	pub fn ywww(self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.w, self.w)
	}
	#[inline(always)]
	pub fn zwww(self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.w, self.w)
	}
	#[inline(always)]
	pub fn wwww(self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.w, self.w)
	}
}