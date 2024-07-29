use std::fmt;
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

impl<T: Element> fmt::Display for Vec2<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

vec_op!(Neg, neg, Vec2, x, y);
vec_op!(Not, not, Vec2, x, y);
vec_rhs_op!(Add, add, Vec2, x, y);
vec_assign_op!(AddAssign, add_assign, Vec2, x, y);
vec_rhs_op!(Sub, sub, Vec2, x, y);
vec_assign_op!(SubAssign, sub_assign, Vec2, x, y);
vec_rhs_op!(Mul, mul, Vec2, x, y);
vec_assign_op!(MulAssign, mul_assign, Vec2, x, y);
vec_rhs_op!(Div, div, Vec2, x, y);
vec_assign_op!(DivAssign, div_assign, Vec2, x, y);
vec_rhs_op!(Rem, rem, Vec2, x, y);
vec_assign_op!(RemAssign, rem_assign, Vec2, x, y);
vec_rhs_op!(BitAnd, bitand, Vec2, x, y);
vec_assign_op!(BitAndAssign, bitand_assign, Vec2, x, y);
vec_rhs_op!(BitOr, bitor, Vec2, x, y);
vec_assign_op!(BitOrAssign, bitor_assign, Vec2, x, y);
vec_rhs_op!(BitXor, bitxor, Vec2, x, y);
vec_assign_op!(BitXorAssign, bitxor_assign, Vec2, x, y);
vec_rhs_op!(Shl, shl, Vec2, x, y);
vec_assign_op!(ShlAssign, shl_assign, Vec2, x, y);
vec_rhs_op!(Shr, shr, Vec2, x, y);
vec_assign_op!(ShrAssign, shr_assign, Vec2, x, y);

impl<T: Element> Vec2<T> {
	vec_swizzle!(Vec2<T>, T, [
			(xx, [x -> x, x -> y]), (yx, [y -> x, x -> y]), (xy, [x -> x * 2]), (yy, [y -> x, y -> y]),
	]);
	vec_swizzle!(Vec3<T>, T, [
			(xxx, [x -> x, x -> y, x -> z]), (yxx, [y -> x, x -> y, x -> z]), (xyx, [x -> x * 2, x -> z]), (yyx, [y -> x, y -> y, x -> z]), (xxy, [x -> x, x -> y * 2]), (yxy, [y -> x, x -> y * 2]), (xyy, [x -> x * 2, y -> z]), (yyy, [y -> x, y -> y, y -> z]),
	]);
	vec_swizzle!(Vec3A<T>, T, [
			(xxx_a, [x -> x, x -> y, x -> z]), (yxx_a, [y -> x, x -> y, x -> z]), (xyx_a, [x -> x * 2, x -> z]), (yyx_a, [y -> x, y -> y, x -> z]), (xxy_a, [x -> x, x -> y * 2]), (yxy_a, [y -> x, x -> y * 2]), (xyy_a, [x -> x * 2, y -> z]), (yyy_a, [y -> x, y -> y, y -> z]),
	]);
	vec_swizzle!(Vec4<T>, T, [
			(xxxx, [x -> x, x -> y, x -> z, x -> w]), (yxxx, [y -> x, x -> y, x -> z, x -> w]), (xyxx, [x -> x * 2, x -> z, x -> w]), (yyxx, [y -> x, y -> y, x -> z, x -> w]), (xxyx, [x -> x, x -> y * 2, x -> w]), (yxyx, [y -> x, x -> y * 2, x -> w]), (xyyx, [x -> x * 2, y -> z, x -> w]), (yyyx, [y -> x, y -> y, y -> z, x -> w]), (xxxy, [x -> x, x -> y, x -> z * 2]), (yxxy, [y -> x, x -> y, x -> z * 2]), (xyxy, [x -> x * 2, x -> z * 2]), (yyxy, [y -> x, y -> y, x -> z * 2]), (xxyy, [x -> x, x -> y * 2, y -> w]), (yxyy, [y -> x, x -> y * 2, y -> w]), (xyyy, [x -> x * 2, y -> z, y -> w]), (yyyy, [y -> x, y -> y, y -> z, y -> w]),
	]);
}
impl<T: Element> Vec2<T> {
	vec_set_swizzle!(Vec2<T>, T, [
			(set_yx, [x -> y, y -> x]), (set_xy, [x -> x * 2]),
	]);
}
impl<T: Element> Vec2<T> {
	#[inline(always)] pub const fn with_x(mut self, value: T) -> Self { self.x = value; self }#[inline(always)] pub const fn with_y(mut self, value: T) -> Self { self.y = value; self }
	vec_with_swizzle!(Vec2<T>, T, [
			(with_yx, [x -> y, y -> x]), (with_xy, [x -> x * 2]),
	]);
}