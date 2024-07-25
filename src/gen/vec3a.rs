use std::fmt::Display;
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3A<C: Component> {
	pub x: C,
	pub y: C,
	pub z: C,
	_3: C,
}

#[inline(always)]
pub const fn vec3a<C: Component>(x: C, y: C, z: C) -> Vec3A<C> {
	Vec3A::new(x, y, z)
}
impl<C: Component> Vec3A<C> {
	#[inline(always)]
	pub const fn new(x: C, y: C, z: C) -> Self {
		Self {
			x,
			y,
			z,
			_3: unsafe { std::mem::zeroed() },
		}
	}
}

impl<C: Component> Display for Vec3A<C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl<C: Component> Vec3A<C> {
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
}