use crate::*;

pub struct Vec3<C: Component> {
	pub x: C,
	pub y: C,
	pub z: C,
}

pub const fn vec3<C: Component>(x: C, y: C, z: C) -> Vec3<C> {
	Vec3::new(x, y, z)
}

impl<C: Component> Vec3<C> {
	pub const fn new(x: C, y: C, z: C) -> Self {
		Self {
			x,
			y,
			z,

		}
	}

	pub fn xx(&self) -> Vec2<C> {
		Vec2::new(self.x, self.x)
	}
	pub fn yx(&self) -> Vec2<C> {
		Vec2::new(self.y, self.x)
	}
	pub fn zx(&self) -> Vec2<C> {
		Vec2::new(self.z, self.x)
	}
	pub fn xy(&self) -> Vec2<C> {
		Vec2::new(self.x, self.y)
	}
	pub fn yy(&self) -> Vec2<C> {
		Vec2::new(self.y, self.y)
	}
	pub fn zy(&self) -> Vec2<C> {
		Vec2::new(self.z, self.y)
	}
	pub fn xz(&self) -> Vec2<C> {
		Vec2::new(self.x, self.z)
	}
	pub fn yz(&self) -> Vec2<C> {
		Vec2::new(self.y, self.z)
	}
	pub fn zz(&self) -> Vec2<C> {
		Vec2::new(self.z, self.z)
	}
	pub fn xxx(&self) -> Vec3<C> {
		Vec3::new(self.x, self.x, self.x)
	}
	pub fn xxx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.x, self.x)
	}
	pub fn yxx(&self) -> Vec3<C> {
		Vec3::new(self.y, self.x, self.x)
	}
	pub fn yxx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.x, self.x)
	}
	pub fn zxx(&self) -> Vec3<C> {
		Vec3::new(self.z, self.x, self.x)
	}
	pub fn zxx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.x, self.x)
	}
	pub fn xyx(&self) -> Vec3<C> {
		Vec3::new(self.x, self.y, self.x)
	}
	pub fn xyx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.y, self.x)
	}
	pub fn yyx(&self) -> Vec3<C> {
		Vec3::new(self.y, self.y, self.x)
	}
	pub fn yyx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.y, self.x)
	}
	pub fn zyx(&self) -> Vec3<C> {
		Vec3::new(self.z, self.y, self.x)
	}
	pub fn zyx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.y, self.x)
	}
	pub fn xzx(&self) -> Vec3<C> {
		Vec3::new(self.x, self.z, self.x)
	}
	pub fn xzx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.z, self.x)
	}
	pub fn yzx(&self) -> Vec3<C> {
		Vec3::new(self.y, self.z, self.x)
	}
	pub fn yzx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.z, self.x)
	}
	pub fn zzx(&self) -> Vec3<C> {
		Vec3::new(self.z, self.z, self.x)
	}
	pub fn zzx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.z, self.x)
	}
	pub fn xxy(&self) -> Vec3<C> {
		Vec3::new(self.x, self.x, self.y)
	}
	pub fn xxy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.x, self.y)
	}
	pub fn yxy(&self) -> Vec3<C> {
		Vec3::new(self.y, self.x, self.y)
	}
	pub fn yxy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.x, self.y)
	}
	pub fn zxy(&self) -> Vec3<C> {
		Vec3::new(self.z, self.x, self.y)
	}
	pub fn zxy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.x, self.y)
	}
	pub fn xyy(&self) -> Vec3<C> {
		Vec3::new(self.x, self.y, self.y)
	}
	pub fn xyy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.y, self.y)
	}
	pub fn yyy(&self) -> Vec3<C> {
		Vec3::new(self.y, self.y, self.y)
	}
	pub fn yyy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.y, self.y)
	}
	pub fn zyy(&self) -> Vec3<C> {
		Vec3::new(self.z, self.y, self.y)
	}
	pub fn zyy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.y, self.y)
	}
	pub fn xzy(&self) -> Vec3<C> {
		Vec3::new(self.x, self.z, self.y)
	}
	pub fn xzy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.z, self.y)
	}
	pub fn yzy(&self) -> Vec3<C> {
		Vec3::new(self.y, self.z, self.y)
	}
	pub fn yzy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.z, self.y)
	}
	pub fn zzy(&self) -> Vec3<C> {
		Vec3::new(self.z, self.z, self.y)
	}
	pub fn zzy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.z, self.y)
	}
	pub fn xxz(&self) -> Vec3<C> {
		Vec3::new(self.x, self.x, self.z)
	}
	pub fn xxz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.x, self.z)
	}
	pub fn yxz(&self) -> Vec3<C> {
		Vec3::new(self.y, self.x, self.z)
	}
	pub fn yxz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.x, self.z)
	}
	pub fn zxz(&self) -> Vec3<C> {
		Vec3::new(self.z, self.x, self.z)
	}
	pub fn zxz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.x, self.z)
	}
	pub fn xyz(&self) -> Vec3<C> {
		Vec3::new(self.x, self.y, self.z)
	}
	pub fn xyz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.y, self.z)
	}
	pub fn yyz(&self) -> Vec3<C> {
		Vec3::new(self.y, self.y, self.z)
	}
	pub fn yyz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.y, self.z)
	}
	pub fn zyz(&self) -> Vec3<C> {
		Vec3::new(self.z, self.y, self.z)
	}
	pub fn zyz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.y, self.z)
	}
	pub fn xzz(&self) -> Vec3<C> {
		Vec3::new(self.x, self.z, self.z)
	}
	pub fn xzz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.z, self.z)
	}
	pub fn yzz(&self) -> Vec3<C> {
		Vec3::new(self.y, self.z, self.z)
	}
	pub fn yzz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.z, self.z)
	}
	pub fn zzz(&self) -> Vec3<C> {
		Vec3::new(self.z, self.z, self.z)
	}
	pub fn zzz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.z, self.z)
	}
	pub fn xxxx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.x)
	}
	pub fn yxxx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.x)
	}
	pub fn zxxx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.x, self.x)
	}
	pub fn xyxx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.x)
	}
	pub fn yyxx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.x)
	}
	pub fn zyxx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.x, self.x)
	}
	pub fn xzxx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.x, self.x)
	}
	pub fn yzxx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.x, self.x)
	}
	pub fn zzxx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.x, self.x)
	}
	pub fn xxyx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.x)
	}
	pub fn yxyx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.x)
	}
	pub fn zxyx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.y, self.x)
	}
	pub fn xyyx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.x)
	}
	pub fn yyyx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.x)
	}
	pub fn zyyx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.y, self.x)
	}
	pub fn xzyx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.y, self.x)
	}
	pub fn yzyx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.y, self.x)
	}
	pub fn zzyx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.y, self.x)
	}
	pub fn xxzx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.z, self.x)
	}
	pub fn yxzx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.z, self.x)
	}
	pub fn zxzx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.z, self.x)
	}
	pub fn xyzx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.z, self.x)
	}
	pub fn yyzx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.z, self.x)
	}
	pub fn zyzx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.z, self.x)
	}
	pub fn xzzx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.z, self.x)
	}
	pub fn yzzx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.z, self.x)
	}
	pub fn zzzx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.z, self.x)
	}
	pub fn xxxy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.y)
	}
	pub fn yxxy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.y)
	}
	pub fn zxxy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.x, self.y)
	}
	pub fn xyxy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.y)
	}
	pub fn yyxy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.y)
	}
	pub fn zyxy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.x, self.y)
	}
	pub fn xzxy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.x, self.y)
	}
	pub fn yzxy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.x, self.y)
	}
	pub fn zzxy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.x, self.y)
	}
	pub fn xxyy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.y)
	}
	pub fn yxyy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.y)
	}
	pub fn zxyy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.y, self.y)
	}
	pub fn xyyy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.y)
	}
	pub fn yyyy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.y)
	}
	pub fn zyyy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.y, self.y)
	}
	pub fn xzyy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.y, self.y)
	}
	pub fn yzyy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.y, self.y)
	}
	pub fn zzyy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.y, self.y)
	}
	pub fn xxzy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.z, self.y)
	}
	pub fn yxzy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.z, self.y)
	}
	pub fn zxzy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.z, self.y)
	}
	pub fn xyzy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.z, self.y)
	}
	pub fn yyzy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.z, self.y)
	}
	pub fn zyzy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.z, self.y)
	}
	pub fn xzzy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.z, self.y)
	}
	pub fn yzzy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.z, self.y)
	}
	pub fn zzzy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.z, self.y)
	}
	pub fn xxxz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.z)
	}
	pub fn yxxz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.z)
	}
	pub fn zxxz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.x, self.z)
	}
	pub fn xyxz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.z)
	}
	pub fn yyxz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.z)
	}
	pub fn zyxz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.x, self.z)
	}
	pub fn xzxz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.x, self.z)
	}
	pub fn yzxz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.x, self.z)
	}
	pub fn zzxz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.x, self.z)
	}
	pub fn xxyz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.z)
	}
	pub fn yxyz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.z)
	}
	pub fn zxyz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.y, self.z)
	}
	pub fn xyyz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.z)
	}
	pub fn yyyz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.z)
	}
	pub fn zyyz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.y, self.z)
	}
	pub fn xzyz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.y, self.z)
	}
	pub fn yzyz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.y, self.z)
	}
	pub fn zzyz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.y, self.z)
	}
	pub fn xxzz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.z, self.z)
	}
	pub fn yxzz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.z, self.z)
	}
	pub fn zxzz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.z, self.z)
	}
	pub fn xyzz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.z, self.z)
	}
	pub fn yyzz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.z, self.z)
	}
	pub fn zyzz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.z, self.z)
	}
	pub fn xzzz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.z, self.z)
	}
	pub fn yzzz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.z, self.z)
	}
	pub fn zzzz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.z, self.z)
	}
}