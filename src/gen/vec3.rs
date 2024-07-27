use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Element> {
	pub(crate) x: T,
	pub(crate) y: T,
	pub(crate) z: T,
}

#[inline(always)]
pub const fn vec3<T: Element>(x: T, y: T, z: T) -> Vec3<T> {
	Vec3::new(x, y, z)
}
impl<T: Element> Vec3<T> {
	#[inline(always)]
	pub const fn new(x: T, y: T, z: T) -> Self {
		unsafe {
			copy_elements_init!(Self, T, [x -> x * 1, y -> y * 1, z -> z * 1])
		}
	}
	#[inline(always)]
	pub const fn splat(value: T) -> Self {
		unsafe {
			copy_elements_init!(Self, T, [value -> x * 1, value -> y * 1, value -> z * 1])
		}
	}
}

impl<T: Element> std::fmt::Display for Vec3<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl<T: Element> Vec3<T> {
	swizzle_fns!(Vec2<T>, T, [
			(xx, [x -> x, x -> y]), (yx, [y -> x, x -> y]), (zx, [z -> x, x -> y]), (xy, [x -> x * 2]), (yy, [y -> x, y -> y]), (zy, [z -> x, y -> y]), (xz, [x -> x, z -> y]), (yz, [y -> x * 2]), (zz, [z -> x, z -> y]),
	]);
	swizzle_fns!(Vec3<T>, T, [
			(xxx, [x -> x, x -> y, x -> z]), (yxx, [y -> x, x -> y, x -> z]), (zxx, [z -> x, x -> y, x -> z]), (xyx, [x -> x * 2, x -> z]), (yyx, [y -> x, y -> y, x -> z]), (zyx, [z -> x, y -> y, x -> z]), (xzx, [x -> x, z -> y, x -> z]), (yzx, [y -> x * 2, x -> z]), (zzx, [z -> x, z -> y, x -> z]), (xxy, [x -> x, x -> y * 2]), (yxy, [y -> x, x -> y * 2]), (zxy, [z -> x, x -> y * 2]), (xyy, [x -> x * 2, y -> z]), (yyy, [y -> x, y -> y, y -> z]), (zyy, [z -> x, y -> y, y -> z]), (xzy, [x -> x, z -> y, y -> z]), (yzy, [y -> x * 2, y -> z]), (zzy, [z -> x, z -> y, y -> z]), (xxz, [x -> x, x -> y, z -> z]), (yxz, [y -> x, x -> y, z -> z]), (zxz, [z -> x, x -> y, z -> z]), (xyz, [x -> x * 3]), (yyz, [y -> x, y -> y * 2]), (zyz, [z -> x, y -> y * 2]), (xzz, [x -> x, z -> y, z -> z]), (yzz, [y -> x * 2, z -> z]), (zzz, [z -> x, z -> y, z -> z]),
	]);
	swizzle_fns!(Vec3A<T>, T, [
			(xxx_a, [x -> x, x -> y, x -> z]), (yxx_a, [y -> x, x -> y, x -> z]), (zxx_a, [z -> x, x -> y, x -> z]), (xyx_a, [x -> x * 2, x -> z]), (yyx_a, [y -> x, y -> y, x -> z]), (zyx_a, [z -> x, y -> y, x -> z]), (xzx_a, [x -> x, z -> y, x -> z]), (yzx_a, [y -> x * 2, x -> z]), (zzx_a, [z -> x, z -> y, x -> z]), (xxy_a, [x -> x, x -> y * 2]), (yxy_a, [y -> x, x -> y * 2]), (zxy_a, [z -> x, x -> y * 2]), (xyy_a, [x -> x * 2, y -> z]), (yyy_a, [y -> x, y -> y, y -> z]), (zyy_a, [z -> x, y -> y, y -> z]), (xzy_a, [x -> x, z -> y, y -> z]), (yzy_a, [y -> x * 2, y -> z]), (zzy_a, [z -> x, z -> y, y -> z]), (xxz_a, [x -> x, x -> y, z -> z]), (yxz_a, [y -> x, x -> y, z -> z]), (zxz_a, [z -> x, x -> y, z -> z]), (xyz_a, [x -> x * 3]), (yyz_a, [y -> x, y -> y * 2]), (zyz_a, [z -> x, y -> y * 2]), (xzz_a, [x -> x, z -> y, z -> z]), (yzz_a, [y -> x * 2, z -> z]), (zzz_a, [z -> x, z -> y, z -> z]),
	]);
	swizzle_fns!(Vec4<T>, T, [
			(xxxx, [x -> x, x -> y, x -> z, x -> w]), (yxxx, [y -> x, x -> y, x -> z, x -> w]), (zxxx, [z -> x, x -> y, x -> z, x -> w]), (xyxx, [x -> x * 2, x -> z, x -> w]), (yyxx, [y -> x, y -> y, x -> z, x -> w]), (zyxx, [z -> x, y -> y, x -> z, x -> w]), (xzxx, [x -> x, z -> y, x -> z, x -> w]), (yzxx, [y -> x * 2, x -> z, x -> w]), (zzxx, [z -> x, z -> y, x -> z, x -> w]), (xxyx, [x -> x, x -> y * 2, x -> w]), (yxyx, [y -> x, x -> y * 2, x -> w]), (zxyx, [z -> x, x -> y * 2, x -> w]), (xyyx, [x -> x * 2, y -> z, x -> w]), (yyyx, [y -> x, y -> y, y -> z, x -> w]), (zyyx, [z -> x, y -> y, y -> z, x -> w]), (xzyx, [x -> x, z -> y, y -> z, x -> w]), (yzyx, [y -> x * 2, y -> z, x -> w]), (zzyx, [z -> x, z -> y, y -> z, x -> w]), (xxzx, [x -> x, x -> y, z -> z, x -> w]), (yxzx, [y -> x, x -> y, z -> z, x -> w]), (zxzx, [z -> x, x -> y, z -> z, x -> w]), (xyzx, [x -> x * 3, x -> w]), (yyzx, [y -> x, y -> y * 2, x -> w]), (zyzx, [z -> x, y -> y * 2, x -> w]), (xzzx, [x -> x, z -> y, z -> z, x -> w]), (yzzx, [y -> x * 2, z -> z, x -> w]), (zzzx, [z -> x, z -> y, z -> z, x -> w]), (xxxy, [x -> x, x -> y, x -> z * 2]), (yxxy, [y -> x, x -> y, x -> z * 2]), (zxxy, [z -> x, x -> y, x -> z * 2]), (xyxy, [x -> x * 2, x -> z * 2]), (yyxy, [y -> x, y -> y, x -> z * 2]), (zyxy, [z -> x, y -> y, x -> z * 2]), (xzxy, [x -> x, z -> y, x -> z * 2]), (yzxy, [y -> x * 2, x -> z * 2]), (zzxy, [z -> x, z -> y, x -> z * 2]), (xxyy, [x -> x, x -> y * 2, y -> w]), (yxyy, [y -> x, x -> y * 2, y -> w]), (zxyy, [z -> x, x -> y * 2, y -> w]), (xyyy, [x -> x * 2, y -> z, y -> w]), (yyyy, [y -> x, y -> y, y -> z, y -> w]), (zyyy, [z -> x, y -> y, y -> z, y -> w]), (xzyy, [x -> x, z -> y, y -> z, y -> w]), (yzyy, [y -> x * 2, y -> z, y -> w]), (zzyy, [z -> x, z -> y, y -> z, y -> w]), (xxzy, [x -> x, x -> y, z -> z, y -> w]), (yxzy, [y -> x, x -> y, z -> z, y -> w]), (zxzy, [z -> x, x -> y, z -> z, y -> w]), (xyzy, [x -> x * 3, y -> w]), (yyzy, [y -> x, y -> y * 2, y -> w]), (zyzy, [z -> x, y -> y * 2, y -> w]), (xzzy, [x -> x, z -> y, z -> z, y -> w]), (yzzy, [y -> x * 2, z -> z, y -> w]), (zzzy, [z -> x, z -> y, z -> z, y -> w]), (xxxz, [x -> x, x -> y, x -> z, z -> w]), (yxxz, [y -> x, x -> y, x -> z, z -> w]), (zxxz, [z -> x, x -> y, x -> z, z -> w]), (xyxz, [x -> x * 2, x -> z, z -> w]), (yyxz, [y -> x, y -> y, x -> z, z -> w]), (zyxz, [z -> x, y -> y, x -> z, z -> w]), (xzxz, [x -> x, z -> y, x -> z, z -> w]), (yzxz, [y -> x * 2, x -> z, z -> w]), (zzxz, [z -> x, z -> y, x -> z, z -> w]), (xxyz, [x -> x, x -> y * 3]), (yxyz, [y -> x, x -> y * 3]), (zxyz, [z -> x, x -> y * 3]), (xyyz, [x -> x * 2, y -> z * 2]), (yyyz, [y -> x, y -> y, y -> z * 2]), (zyyz, [z -> x, y -> y, y -> z * 2]), (xzyz, [x -> x, z -> y, y -> z * 2]), (yzyz, [y -> x * 2, y -> z * 2]), (zzyz, [z -> x, z -> y, y -> z * 2]), (xxzz, [x -> x, x -> y, z -> z, z -> w]), (yxzz, [y -> x, x -> y, z -> z, z -> w]), (zxzz, [z -> x, x -> y, z -> z, z -> w]), (xyzz, [x -> x * 3, z -> w]), (yyzz, [y -> x, y -> y * 2, z -> w]), (zyzz, [z -> x, y -> y * 2, z -> w]), (xzzz, [x -> x, z -> y, z -> z, z -> w]), (yzzz, [y -> x * 2, z -> z, z -> w]), (zzzz, [z -> x, z -> y, z -> z, z -> w]),
	]);
}
impl<T: Element> Vec3<T> {
	set_swizzle_fns!(Vec2<T>, T, [
			(set_yx, [x -> y, y -> x]), (set_zx, [x -> z, y -> x]), (set_xy, [x -> x * 2]), (set_zy, [x -> z, y -> y]), (set_xz, [x -> x, y -> z]), (set_yz, [x -> y * 2]),
	]);
	set_swizzle_fns!(Vec3<T>, T, [
			(set_zyx, [x -> z, y -> y, z -> x]), (set_yzx, [x -> y * 2, z -> x]), (set_zxy, [x -> z, y -> x * 2]), (set_xzy, [x -> x, y -> z, z -> y]), (set_yxz, [x -> y, y -> x, z -> z]), (set_xyz, [x -> x * 3]),
	]);
	set_swizzle_fns!(Vec3A<T>, T, [
			(set_zyx_a, [x -> z, y -> y, z -> x]), (set_yzx_a, [x -> y * 2, z -> x]), (set_zxy_a, [x -> z, y -> x * 2]), (set_xzy_a, [x -> x, y -> z, z -> y]), (set_yxz_a, [x -> y, y -> x, z -> z]), (set_xyz_a, [x -> x * 3]),
	]);
}
impl<T: Element> Vec3<T> {
	#[inline(always)] pub const fn with_x(mut self, value: T) -> Self { self.x = value; self }#[inline(always)] pub const fn with_y(mut self, value: T) -> Self { self.y = value; self }#[inline(always)] pub const fn with_z(mut self, value: T) -> Self { self.z = value; self }

	with_swizzle_fns!(Vec2<T>, T, [
			(with_yx, [x -> y, y -> x]), (with_zx, [x -> z, y -> x]), (with_xy, [x -> x * 2]), (with_zy, [x -> z, y -> y]), (with_xz, [x -> x, y -> z]), (with_yz, [x -> y * 2]),
	]);
	with_swizzle_fns!(Vec3<T>, T, [
			(with_zyx, [x -> z, y -> y, z -> x]), (with_yzx, [x -> y * 2, z -> x]), (with_zxy, [x -> z, y -> x * 2]), (with_xzy, [x -> x, y -> z, z -> y]), (with_yxz, [x -> y, y -> x, z -> z]), (with_xyz, [x -> x * 3]),
	]);
	with_swizzle_fns!(Vec3A<T>, T, [
			(with_zyx_a, [x -> z, y -> y, z -> x]), (with_yzx_a, [x -> y * 2, z -> x]), (with_zxy_a, [x -> z, y -> x * 2]), (with_xzy_a, [x -> x, y -> z, z -> y]), (with_yxz_a, [x -> y, y -> x, z -> z]), (with_xyz_a, [x -> x * 3]),
	]);
}