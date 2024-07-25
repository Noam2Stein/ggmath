use crate::*;

pub struct Vec4<C: Component> {
	pub x: C,
	pub y: C,
	pub z: C,
	pub w: C,
}

pub const fn vec4<C: Component>(x: C, y: C, z: C, w: C) -> Vec4<C> {
	Vec4::new(x, y, z, w)
}

impl<C: Component> Vec4<C> {
	pub const fn new(x: C, y: C, z: C, w: C) -> Self {
		Self {
			x,
			y,
			z,
			w,

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
	pub fn wx(&self) -> Vec2<C> {
		Vec2::new(self.w, self.x)
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
	pub fn wy(&self) -> Vec2<C> {
		Vec2::new(self.w, self.y)
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
	pub fn wz(&self) -> Vec2<C> {
		Vec2::new(self.w, self.z)
	}
	pub fn xw(&self) -> Vec2<C> {
		Vec2::new(self.x, self.w)
	}
	pub fn yw(&self) -> Vec2<C> {
		Vec2::new(self.y, self.w)
	}
	pub fn zw(&self) -> Vec2<C> {
		Vec2::new(self.z, self.w)
	}
	pub fn ww(&self) -> Vec2<C> {
		Vec2::new(self.w, self.w)
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
	pub fn wxx(&self) -> Vec3<C> {
		Vec3::new(self.w, self.x, self.x)
	}
	pub fn wxx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.x, self.x)
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
	pub fn wyx(&self) -> Vec3<C> {
		Vec3::new(self.w, self.y, self.x)
	}
	pub fn wyx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.y, self.x)
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
	pub fn wzx(&self) -> Vec3<C> {
		Vec3::new(self.w, self.z, self.x)
	}
	pub fn wzx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.z, self.x)
	}
	pub fn xwx(&self) -> Vec3<C> {
		Vec3::new(self.x, self.w, self.x)
	}
	pub fn xwx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.w, self.x)
	}
	pub fn ywx(&self) -> Vec3<C> {
		Vec3::new(self.y, self.w, self.x)
	}
	pub fn ywx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.w, self.x)
	}
	pub fn zwx(&self) -> Vec3<C> {
		Vec3::new(self.z, self.w, self.x)
	}
	pub fn zwx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.w, self.x)
	}
	pub fn wwx(&self) -> Vec3<C> {
		Vec3::new(self.w, self.w, self.x)
	}
	pub fn wwx_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.w, self.x)
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
	pub fn wxy(&self) -> Vec3<C> {
		Vec3::new(self.w, self.x, self.y)
	}
	pub fn wxy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.x, self.y)
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
	pub fn wyy(&self) -> Vec3<C> {
		Vec3::new(self.w, self.y, self.y)
	}
	pub fn wyy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.y, self.y)
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
	pub fn wzy(&self) -> Vec3<C> {
		Vec3::new(self.w, self.z, self.y)
	}
	pub fn wzy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.z, self.y)
	}
	pub fn xwy(&self) -> Vec3<C> {
		Vec3::new(self.x, self.w, self.y)
	}
	pub fn xwy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.w, self.y)
	}
	pub fn ywy(&self) -> Vec3<C> {
		Vec3::new(self.y, self.w, self.y)
	}
	pub fn ywy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.w, self.y)
	}
	pub fn zwy(&self) -> Vec3<C> {
		Vec3::new(self.z, self.w, self.y)
	}
	pub fn zwy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.w, self.y)
	}
	pub fn wwy(&self) -> Vec3<C> {
		Vec3::new(self.w, self.w, self.y)
	}
	pub fn wwy_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.w, self.y)
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
	pub fn wxz(&self) -> Vec3<C> {
		Vec3::new(self.w, self.x, self.z)
	}
	pub fn wxz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.x, self.z)
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
	pub fn wyz(&self) -> Vec3<C> {
		Vec3::new(self.w, self.y, self.z)
	}
	pub fn wyz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.y, self.z)
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
	pub fn wzz(&self) -> Vec3<C> {
		Vec3::new(self.w, self.z, self.z)
	}
	pub fn wzz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.z, self.z)
	}
	pub fn xwz(&self) -> Vec3<C> {
		Vec3::new(self.x, self.w, self.z)
	}
	pub fn xwz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.w, self.z)
	}
	pub fn ywz(&self) -> Vec3<C> {
		Vec3::new(self.y, self.w, self.z)
	}
	pub fn ywz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.w, self.z)
	}
	pub fn zwz(&self) -> Vec3<C> {
		Vec3::new(self.z, self.w, self.z)
	}
	pub fn zwz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.w, self.z)
	}
	pub fn wwz(&self) -> Vec3<C> {
		Vec3::new(self.w, self.w, self.z)
	}
	pub fn wwz_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.w, self.z)
	}
	pub fn xxw(&self) -> Vec3<C> {
		Vec3::new(self.x, self.x, self.w)
	}
	pub fn xxw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.x, self.w)
	}
	pub fn yxw(&self) -> Vec3<C> {
		Vec3::new(self.y, self.x, self.w)
	}
	pub fn yxw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.x, self.w)
	}
	pub fn zxw(&self) -> Vec3<C> {
		Vec3::new(self.z, self.x, self.w)
	}
	pub fn zxw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.x, self.w)
	}
	pub fn wxw(&self) -> Vec3<C> {
		Vec3::new(self.w, self.x, self.w)
	}
	pub fn wxw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.x, self.w)
	}
	pub fn xyw(&self) -> Vec3<C> {
		Vec3::new(self.x, self.y, self.w)
	}
	pub fn xyw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.y, self.w)
	}
	pub fn yyw(&self) -> Vec3<C> {
		Vec3::new(self.y, self.y, self.w)
	}
	pub fn yyw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.y, self.w)
	}
	pub fn zyw(&self) -> Vec3<C> {
		Vec3::new(self.z, self.y, self.w)
	}
	pub fn zyw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.y, self.w)
	}
	pub fn wyw(&self) -> Vec3<C> {
		Vec3::new(self.w, self.y, self.w)
	}
	pub fn wyw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.y, self.w)
	}
	pub fn xzw(&self) -> Vec3<C> {
		Vec3::new(self.x, self.z, self.w)
	}
	pub fn xzw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.z, self.w)
	}
	pub fn yzw(&self) -> Vec3<C> {
		Vec3::new(self.y, self.z, self.w)
	}
	pub fn yzw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.z, self.w)
	}
	pub fn zzw(&self) -> Vec3<C> {
		Vec3::new(self.z, self.z, self.w)
	}
	pub fn zzw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.z, self.w)
	}
	pub fn wzw(&self) -> Vec3<C> {
		Vec3::new(self.w, self.z, self.w)
	}
	pub fn wzw_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.z, self.w)
	}
	pub fn xww(&self) -> Vec3<C> {
		Vec3::new(self.x, self.w, self.w)
	}
	pub fn xww_a(&self) -> Vec3A<C> {
		Vec3A::new(self.x, self.w, self.w)
	}
	pub fn yww(&self) -> Vec3<C> {
		Vec3::new(self.y, self.w, self.w)
	}
	pub fn yww_a(&self) -> Vec3A<C> {
		Vec3A::new(self.y, self.w, self.w)
	}
	pub fn zww(&self) -> Vec3<C> {
		Vec3::new(self.z, self.w, self.w)
	}
	pub fn zww_a(&self) -> Vec3A<C> {
		Vec3A::new(self.z, self.w, self.w)
	}
	pub fn www(&self) -> Vec3<C> {
		Vec3::new(self.w, self.w, self.w)
	}
	pub fn www_a(&self) -> Vec3A<C> {
		Vec3A::new(self.w, self.w, self.w)
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
	pub fn wxxx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.x, self.x)
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
	pub fn wyxx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.x, self.x)
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
	pub fn wzxx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.x, self.x)
	}
	pub fn xwxx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.x, self.x)
	}
	pub fn ywxx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.x, self.x)
	}
	pub fn zwxx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.x, self.x)
	}
	pub fn wwxx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.x, self.x)
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
	pub fn wxyx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.y, self.x)
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
	pub fn wyyx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.y, self.x)
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
	pub fn wzyx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.y, self.x)
	}
	pub fn xwyx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.y, self.x)
	}
	pub fn ywyx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.y, self.x)
	}
	pub fn zwyx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.y, self.x)
	}
	pub fn wwyx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.y, self.x)
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
	pub fn wxzx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.z, self.x)
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
	pub fn wyzx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.z, self.x)
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
	pub fn wzzx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.z, self.x)
	}
	pub fn xwzx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.z, self.x)
	}
	pub fn ywzx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.z, self.x)
	}
	pub fn zwzx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.z, self.x)
	}
	pub fn wwzx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.z, self.x)
	}
	pub fn xxwx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.w, self.x)
	}
	pub fn yxwx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.w, self.x)
	}
	pub fn zxwx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.w, self.x)
	}
	pub fn wxwx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.w, self.x)
	}
	pub fn xywx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.w, self.x)
	}
	pub fn yywx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.w, self.x)
	}
	pub fn zywx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.w, self.x)
	}
	pub fn wywx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.w, self.x)
	}
	pub fn xzwx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.w, self.x)
	}
	pub fn yzwx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.w, self.x)
	}
	pub fn zzwx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.w, self.x)
	}
	pub fn wzwx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.w, self.x)
	}
	pub fn xwwx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.w, self.x)
	}
	pub fn ywwx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.w, self.x)
	}
	pub fn zwwx(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.w, self.x)
	}
	pub fn wwwx(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.w, self.x)
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
	pub fn wxxy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.x, self.y)
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
	pub fn wyxy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.x, self.y)
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
	pub fn wzxy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.x, self.y)
	}
	pub fn xwxy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.x, self.y)
	}
	pub fn ywxy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.x, self.y)
	}
	pub fn zwxy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.x, self.y)
	}
	pub fn wwxy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.x, self.y)
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
	pub fn wxyy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.y, self.y)
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
	pub fn wyyy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.y, self.y)
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
	pub fn wzyy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.y, self.y)
	}
	pub fn xwyy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.y, self.y)
	}
	pub fn ywyy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.y, self.y)
	}
	pub fn zwyy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.y, self.y)
	}
	pub fn wwyy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.y, self.y)
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
	pub fn wxzy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.z, self.y)
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
	pub fn wyzy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.z, self.y)
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
	pub fn wzzy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.z, self.y)
	}
	pub fn xwzy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.z, self.y)
	}
	pub fn ywzy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.z, self.y)
	}
	pub fn zwzy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.z, self.y)
	}
	pub fn wwzy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.z, self.y)
	}
	pub fn xxwy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.w, self.y)
	}
	pub fn yxwy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.w, self.y)
	}
	pub fn zxwy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.w, self.y)
	}
	pub fn wxwy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.w, self.y)
	}
	pub fn xywy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.w, self.y)
	}
	pub fn yywy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.w, self.y)
	}
	pub fn zywy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.w, self.y)
	}
	pub fn wywy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.w, self.y)
	}
	pub fn xzwy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.w, self.y)
	}
	pub fn yzwy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.w, self.y)
	}
	pub fn zzwy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.w, self.y)
	}
	pub fn wzwy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.w, self.y)
	}
	pub fn xwwy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.w, self.y)
	}
	pub fn ywwy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.w, self.y)
	}
	pub fn zwwy(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.w, self.y)
	}
	pub fn wwwy(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.w, self.y)
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
	pub fn wxxz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.x, self.z)
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
	pub fn wyxz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.x, self.z)
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
	pub fn wzxz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.x, self.z)
	}
	pub fn xwxz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.x, self.z)
	}
	pub fn ywxz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.x, self.z)
	}
	pub fn zwxz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.x, self.z)
	}
	pub fn wwxz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.x, self.z)
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
	pub fn wxyz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.y, self.z)
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
	pub fn wyyz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.y, self.z)
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
	pub fn wzyz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.y, self.z)
	}
	pub fn xwyz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.y, self.z)
	}
	pub fn ywyz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.y, self.z)
	}
	pub fn zwyz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.y, self.z)
	}
	pub fn wwyz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.y, self.z)
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
	pub fn wxzz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.z, self.z)
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
	pub fn wyzz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.z, self.z)
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
	pub fn wzzz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.z, self.z)
	}
	pub fn xwzz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.z, self.z)
	}
	pub fn ywzz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.z, self.z)
	}
	pub fn zwzz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.z, self.z)
	}
	pub fn wwzz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.z, self.z)
	}
	pub fn xxwz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.w, self.z)
	}
	pub fn yxwz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.w, self.z)
	}
	pub fn zxwz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.w, self.z)
	}
	pub fn wxwz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.w, self.z)
	}
	pub fn xywz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.w, self.z)
	}
	pub fn yywz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.w, self.z)
	}
	pub fn zywz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.w, self.z)
	}
	pub fn wywz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.w, self.z)
	}
	pub fn xzwz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.w, self.z)
	}
	pub fn yzwz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.w, self.z)
	}
	pub fn zzwz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.w, self.z)
	}
	pub fn wzwz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.w, self.z)
	}
	pub fn xwwz(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.w, self.z)
	}
	pub fn ywwz(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.w, self.z)
	}
	pub fn zwwz(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.w, self.z)
	}
	pub fn wwwz(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.w, self.z)
	}
	pub fn xxxw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.w)
	}
	pub fn yxxw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.w)
	}
	pub fn zxxw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.x, self.w)
	}
	pub fn wxxw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.x, self.w)
	}
	pub fn xyxw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.w)
	}
	pub fn yyxw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.w)
	}
	pub fn zyxw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.x, self.w)
	}
	pub fn wyxw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.x, self.w)
	}
	pub fn xzxw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.x, self.w)
	}
	pub fn yzxw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.x, self.w)
	}
	pub fn zzxw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.x, self.w)
	}
	pub fn wzxw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.x, self.w)
	}
	pub fn xwxw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.x, self.w)
	}
	pub fn ywxw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.x, self.w)
	}
	pub fn zwxw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.x, self.w)
	}
	pub fn wwxw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.x, self.w)
	}
	pub fn xxyw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.w)
	}
	pub fn yxyw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.w)
	}
	pub fn zxyw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.y, self.w)
	}
	pub fn wxyw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.y, self.w)
	}
	pub fn xyyw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.w)
	}
	pub fn yyyw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.w)
	}
	pub fn zyyw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.y, self.w)
	}
	pub fn wyyw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.y, self.w)
	}
	pub fn xzyw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.y, self.w)
	}
	pub fn yzyw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.y, self.w)
	}
	pub fn zzyw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.y, self.w)
	}
	pub fn wzyw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.y, self.w)
	}
	pub fn xwyw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.y, self.w)
	}
	pub fn ywyw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.y, self.w)
	}
	pub fn zwyw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.y, self.w)
	}
	pub fn wwyw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.y, self.w)
	}
	pub fn xxzw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.z, self.w)
	}
	pub fn yxzw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.z, self.w)
	}
	pub fn zxzw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.z, self.w)
	}
	pub fn wxzw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.z, self.w)
	}
	pub fn xyzw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.z, self.w)
	}
	pub fn yyzw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.z, self.w)
	}
	pub fn zyzw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.z, self.w)
	}
	pub fn wyzw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.z, self.w)
	}
	pub fn xzzw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.z, self.w)
	}
	pub fn yzzw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.z, self.w)
	}
	pub fn zzzw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.z, self.w)
	}
	pub fn wzzw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.z, self.w)
	}
	pub fn xwzw(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.z, self.w)
	}
	pub fn ywzw(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.z, self.w)
	}
	pub fn zwzw(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.z, self.w)
	}
	pub fn wwzw(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.z, self.w)
	}
	pub fn xxww(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.w, self.w)
	}
	pub fn yxww(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.w, self.w)
	}
	pub fn zxww(&self) -> Vec4<C> {
		Vec4::new(self.z, self.x, self.w, self.w)
	}
	pub fn wxww(&self) -> Vec4<C> {
		Vec4::new(self.w, self.x, self.w, self.w)
	}
	pub fn xyww(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.w, self.w)
	}
	pub fn yyww(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.w, self.w)
	}
	pub fn zyww(&self) -> Vec4<C> {
		Vec4::new(self.z, self.y, self.w, self.w)
	}
	pub fn wyww(&self) -> Vec4<C> {
		Vec4::new(self.w, self.y, self.w, self.w)
	}
	pub fn xzww(&self) -> Vec4<C> {
		Vec4::new(self.x, self.z, self.w, self.w)
	}
	pub fn yzww(&self) -> Vec4<C> {
		Vec4::new(self.y, self.z, self.w, self.w)
	}
	pub fn zzww(&self) -> Vec4<C> {
		Vec4::new(self.z, self.z, self.w, self.w)
	}
	pub fn wzww(&self) -> Vec4<C> {
		Vec4::new(self.w, self.z, self.w, self.w)
	}
	pub fn xwww(&self) -> Vec4<C> {
		Vec4::new(self.x, self.w, self.w, self.w)
	}
	pub fn ywww(&self) -> Vec4<C> {
		Vec4::new(self.y, self.w, self.w, self.w)
	}
	pub fn zwww(&self) -> Vec4<C> {
		Vec4::new(self.z, self.w, self.w, self.w)
	}
	pub fn wwww(&self) -> Vec4<C> {
		Vec4::new(self.w, self.w, self.w, self.w)
	}
}