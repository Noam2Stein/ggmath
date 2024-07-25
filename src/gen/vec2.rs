use std::fmt::Display;
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<C: Component> {
	pub x: C,
	pub y: C,
}

#[inline(always)]
pub const fn vec2<C: Component>(x: C, y: C) -> Vec2<C> {
	Vec2::new(x, y)
}
impl<C: Component> Vec2<C> {
	#[inline(always)]
	pub const fn new(x: C, y: C) -> Self {
		Self {
			x,
			y,

		}
	}
}

impl<C: Component> Display for Vec2<C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl<C: Component> Vec2<C> {
	#[inline(always)]
	pub fn xx(self) -> Vec2<C> {
		Vec2::new(self.x, self.x)
	}
	#[inline(always)]
	pub fn yx(self) -> Vec2<C> {
		Vec2::new(self.y, self.x)
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
	pub fn xxxx(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.x)
	}
	#[inline(always)]
	pub fn yxxx(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.x)
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
	pub fn xxyx(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.x)
	}
	#[inline(always)]
	pub fn yxyx(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.x)
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
	pub fn xxxy(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.x, self.y)
	}
	#[inline(always)]
	pub fn yxxy(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.x, self.y)
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
	pub fn xxyy(self) -> Vec4<C> {
		Vec4::new(self.x, self.x, self.y, self.y)
	}
	#[inline(always)]
	pub fn yxyy(self) -> Vec4<C> {
		Vec4::new(self.y, self.x, self.y, self.y)
	}
	#[inline(always)]
	pub fn xyyy(self) -> Vec4<C> {
		Vec4::new(self.x, self.y, self.y, self.y)
	}
	#[inline(always)]
	pub fn yyyy(self) -> Vec4<C> {
		Vec4::new(self.y, self.y, self.y, self.y)
	}
}