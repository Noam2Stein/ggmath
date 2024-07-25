use crate::*;

pub struct Vec2<C: Component> {
	pub x: C,
	pub y: C,
}

pub const fn vec2<C: Component>(x: C, y: C) -> Vec2<C> {
	Vec2::new(x, y)
}

impl<C: Component> Vec2<C> {
	pub const fn new(x: C, y: C) -> Self {
		Self {
			x,
			y,

		}
	}

	pub fn xx(&self) -> Vec2<C> {
		Vec2::new(self.x, self.x)
	}
	pub fn yx(&self) -> Vec2<C> {
		Vec2::new(self.y, self.x)
	}
	pub fn xy(&self) -> Vec2<C> {
		Vec2::new(self.x, self.y)
	}
	pub fn yy(&self) -> Vec2<C> {
		Vec2::new(self.y, self.y)
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
	pub fn xxxx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.x)
	}
	pub fn yxxx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.x)
	}
	pub fn xyxx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.x)
	}
	pub fn yyxx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.x)
	}
	pub fn xxyx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.x)
	}
	pub fn yxyx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.x)
	}
	pub fn xyyx(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.x)
	}
	pub fn yyyx(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.x)
	}
	pub fn xxxy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.y)
	}
	pub fn yxxy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.y)
	}
	pub fn xyxy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.x, self.y)
	}
	pub fn yyxy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.x, self.y)
	}
	pub fn xxyy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.y)
	}
	pub fn yxyy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.y)
	}
	pub fn xyyy(&self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.y)
	}
	pub fn yyyy(&self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.y)
	}
}