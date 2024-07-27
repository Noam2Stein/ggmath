use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T: Element> {
	pub(crate) x: T,
	pub(crate) y: T,
}

#[inline(always)]
pub const fn vec2<T: Element>(x: T, y: T) -> Vec2<T> {
	Vec2::new(x, y)
}
impl<T: Element> Vec2<T> {
	#[inline(always)]
	pub const fn new(x: T, y: T) -> Self {
		unsafe {
			copy_elements_init!(Self, T, [x -> x * 1, y -> y * 1])
		}
	}
	#[inline(always)]
	pub const fn splat(value: T) -> Self {
		unsafe {
			copy_elements_init!(Self, T, [value -> x * 1, value -> y * 1])
		}
	}
}

impl<T: Element> std::fmt::Display for Vec2<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl<T: Element> Vec2<T> {
	swizzle_fns!(Vec2<T>, T, [
			(xx, [x -> x, x -> y]), (yx, [y -> x, x -> y]), (xy, [x -> x * 2]), (yy, [y -> x, y -> y]),
	]);
	swizzle_fns!(Vec3<T>, T, [
			(xxx, [x -> x, x -> y, x -> z]), (yxx, [y -> x, x -> y, x -> z]), (xyx, [x -> x * 2, x -> z]), (yyx, [y -> x, y -> y, x -> z]), (xxy, [x -> x, x -> y * 2]), (yxy, [y -> x, x -> y * 2]), (xyy, [x -> x * 2, y -> z]), (yyy, [y -> x, y -> y, y -> z]),
	]);
	swizzle_fns!(Vec3A<T>, T, [
			(xxx_a, [x -> x, x -> y, x -> z]), (yxx_a, [y -> x, x -> y, x -> z]), (xyx_a, [x -> x * 2, x -> z]), (yyx_a, [y -> x, y -> y, x -> z]), (xxy_a, [x -> x, x -> y * 2]), (yxy_a, [y -> x, x -> y * 2]), (xyy_a, [x -> x * 2, y -> z]), (yyy_a, [y -> x, y -> y, y -> z]),
	]);
	swizzle_fns!(Vec4<T>, T, [
			(xxxx, [x -> x, x -> y, x -> z, x -> w]), (yxxx, [y -> x, x -> y, x -> z, x -> w]), (xyxx, [x -> x * 2, x -> z, x -> w]), (yyxx, [y -> x, y -> y, x -> z, x -> w]), (xxyx, [x -> x, x -> y * 2, x -> w]), (yxyx, [y -> x, x -> y * 2, x -> w]), (xyyx, [x -> x * 2, y -> z, x -> w]), (yyyx, [y -> x, y -> y, y -> z, x -> w]), (xxxy, [x -> x, x -> y, x -> z * 2]), (yxxy, [y -> x, x -> y, x -> z * 2]), (xyxy, [x -> x * 2, x -> z * 2]), (yyxy, [y -> x, y -> y, x -> z * 2]), (xxyy, [x -> x, x -> y * 2, y -> w]), (yxyy, [y -> x, x -> y * 2, y -> w]), (xyyy, [x -> x * 2, y -> z, y -> w]), (yyyy, [y -> x, y -> y, y -> z, y -> w]),
	]);
}
impl<T: Element> Vec2<T> {
	set_swizzle_fns!(Vec2<T>, T, [
			(set_yx, [x -> y, y -> x]), (set_xy, [x -> x * 2]),
	]);
}
impl<T: Element> Vec2<T> {
	#[inline(always)] pub const fn with_x(mut self, value: T) -> Self { self.x = value; self }#[inline(always)] pub const fn with_y(mut self, value: T) -> Self { self.y = value; self }

	with_swizzle_fns!(Vec2<T>, T, [
			(with_yx, [x -> y, y -> x]), (with_xy, [x -> x * 2]),
	]);
}