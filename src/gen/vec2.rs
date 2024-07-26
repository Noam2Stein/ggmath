use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<C: Component> {
	pub(crate) x: C,
	pub(crate) y: C,
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
	#[inline(always)]
	pub const fn splat(value: C) -> Self {
		Self {
			x: value,
			y: value,
		}
	}
}

impl<C: Component> std::fmt::Display for Vec2<C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl<C: Component> Vec2<C> {
	#[inline(always)] pub const fn xx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn xy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [x -> x * 2]) } }
	#[inline(always)] pub const fn yx(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 1, x -> y * 1]) } }
	#[inline(always)] pub const fn yy(self) -> Vec2<C> { unsafe { swizzle!(self, Vec2, C, [y -> x * 1, y -> y * 1]) } }
	#[inline(always)] pub const fn xxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn xxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn xxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn xyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn xyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 2, x -> z * 2]) } }
	#[inline(always)] pub const fn xyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [x -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn xyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [x -> x * 2, y -> z * 2]) } }
	#[inline(always)] pub const fn yxx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yxx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yxy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn yxy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, x -> y * 2]) } }
	#[inline(always)] pub const fn yyx(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yyx_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 1, x -> z * 1]) } }
	#[inline(always)] pub const fn yyy(self) -> Vec3<C> { unsafe { swizzle!(self, Vec3, C, [y -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn yyy_a(self) -> Vec3A<C> { unsafe { swizzle!(self, Vec3A, C, [y -> x * 1, y -> y * 1, y -> z * 1]) } }
	#[inline(always)] pub const fn xxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn xxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn xxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn xyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, x -> z * 3]) } }
	#[inline(always)] pub const fn xyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn xyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [x -> x * 2, y -> z * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yxxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yxxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn yxyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 2, x -> w * 2]) } }
	#[inline(always)] pub const fn yxyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, x -> y * 2, y -> w * 2]) } }
	#[inline(always)] pub const fn yyxx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yyxy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, x -> z * 2]) } }
	#[inline(always)] pub const fn yyyx(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 1, x -> w * 1]) } }
	#[inline(always)] pub const fn yyyy(self) -> Vec4<C> { unsafe { swizzle!(self, Vec4, C, [y -> x * 1, y -> y * 1, y -> z * 1, y -> w * 1]) } }
}
impl<C: Component> Vec2<C> {
	#[inline(always)] pub const fn set_xy(&mut self, value: Vec2<C>) { unsafe { set_swizzle!(value, self, Vec2, C, [x -> x * 2]) } }
	#[inline(always)] pub const fn set_yx(&mut self, value: Vec2<C>) { unsafe { set_swizzle!(value, self, Vec2, C, [x -> y * 1, y -> x * 1]) } }
}
impl<C: Component> Vec2<C> {
	#[inline(always)] pub const fn with_x(mut self, value: C) -> Self { self.x = value; self }
	#[inline(always)] pub const fn with_y(mut self, value: C) -> Self { self.y = value; self }
	#[inline(always)] pub const fn with_xy(mut self, value: Vec2<C>) -> Self { unsafe { with_swizzle!(self, value, Vec2, C, [x -> x * 2]) } }
	#[inline(always)] pub const fn with_yx(mut self, value: Vec2<C>) -> Self { unsafe { with_swizzle!(self, value, Vec2, C, [x -> y * 1, y -> x * 1]) } }
}