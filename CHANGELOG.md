# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Affine function `from_column_fn`.

- `wide` functions:
  - `from_lanes`
  - `from_lane_fn`
  - `to_lanes`
  - `lane`
  - `set_lane`
  - `simd_eq`
  - `simd_ne`

- more `wide` float-vector functions.

### Changed

- Made `Vector::distance_squared` generic over `T`.

- Removed the `std` feature flag.

- Updated minimum supported Rust version to `1.95.0`.

- Renamed many functions to use `vector` and `matrix` instead of
  `vec` and `mat`.

- Renamed affine `matrix` field to `submatrix`.

- Changed the receiver of `Matrix::transpose` from `self` to `&self`.

- Made the `FloatExt` trait sealed.

- Changed the addition/multiplication order of `element_sum` and
  `element_product` to make them cross platform deterministic. This breaks
  promises in the function's documentation. This indirectly modifies the precise
  result of many functions.

- Modified `wide` vector function implementations for `max`, `min`, `clamp`,
  `max_element` and `min_element`. This may change NaN handling.

- Made `wide` vector functions generic:
  - `any`
  - `all`
  - `blend`
  - `simd_eq`
  - `simd_ne`
  - `simd_eq_mask`
  - `simd_ne_mask`
  - `simd_lt_mask`
  - `simd_gt_mask`
  - `simd_le_mask`
  - `simd_ge_mask`

- Updated dependency minor versions.

### Fixed

- `NAN` constant for `wide` types was previously incorrect.

### Removed

- Deprecated items.

- Vector `From<(_,)>` implementations.

- Matrix `From` implementations.

## [0.16.7] - 2026-04-15

### Added

- Quaternion functions:
  - `xyz`
  - `conjugate`
  - `canonical`
  - `dot`
  - `length_squared`
  - `from_rotation_x`
  - `from_rotation_y`
  - `from_rotation_z`
  - `from_axis_angle`
  - `from_scaled_axis`
  - `from_rotation_arc`
  - `from_rotation_arc_colinear`
  - `from_euler`
  - `from_matrix`
  - `look_to_lh`
  - `look_to_rh`
  - `look_at_lh`
  - `look_at_rh`
  - `to_axis_angle`
  - `to_scaled_axis`
  - `to_euler`
  - `is_nan`
  - `is_finite`
  - `inverse`
  - `angle_between`
  - `lerp`
  - `slerp`
  - `rotate_towards`
  - `length`
  - `normalize`
  - `try_normalize`
  - `normalize_or`
  - `normalize_and_length`
  - `is_normalized`

- Quaternion multiplication and scalar division.

- Matrix functions:
  - `to_scale_angle`
  - `to_scale_rotation`

- Vector functions:
  - `extend`
  - `truncate`

- `wide` vector functions:
  - `all`
  - `any`
  - `blend`
  - `is_nan`
  - `nan_mask`
  - `is_finite`
  - `finite_mask`
  - `recip`
  - `simd_eq`
  - `simd_ne`
  - `simd_eq_mask`
  - `simd_ne_mask`
  - `simd_lt_mask`
  - `simd_gt_mask`
  - `simd_le_mask`
  - `simd_ge_mask`
  - `max`
  - `min`
  - `clamp`
  - `max_element`
  - `min_element`

- Support for crate `rand`.

### Changed

- Renamed quaternion functions:
  - `new` to `from_xyzw`
  - `from_vec` to `from_vector`
  - `to_vec` to `to_vector`
  - `as_vec_ref` to `as_vector_ref`
  - `as_vec_mut` to `as_vector_mut`

- Updated dependency minor versions and removed unnecessary features.

- Updated documentation.

### Deprecated

- `Quaternion::ZERO` because it does not represent a valid rotation.

## [0.16.6] - 2026-04-03

### Added

- Affine transform functions:
  - `column`
  - `column_mut`
  - `transform_point`
  - `transform_vector`
  - `from_column_array`
  - `from_scale`
  - `from_matrix`
  - `to_matrix`
  - `is_nan`
  - `is_finite`
  - `inverse`
  - `try_inverse`
  - `inverse_or`
  - `inverse_or_zero`
  - `from_angle`
  - `from_angle_translation`
  - `from_scale_angle`
  - `from_scale_angle_translation`
  - `to_scale_angle_translation`
  - `from_rotation_x`
  - `from_rotation_y`
  - `from_rotation_z`
  - `from_quat`
  - `from_axis_angle`
  - `from_euler`
  - `from_scale_rotation`
  - `from_rotation_translation`
  - `from_scale_rotation_translation`
  - `look_to_lh`
  - `look_to_rh`
  - `look_at_lh`
  - `look_at_rh`
  - `to_euler`
  - `to_scale_rotation_translation`

- Matrix functions:
  - `from_angle_translation`
  - `to_scale_angle_translation`
  - `from_scale_rotation`

- Affine transform multiplication.

### Changed

- Renamed affine transform functions:
  - `from_mat` to `from_submatrix`
  - `from_mat_translation` to `from_submatrix_translation`

- Updated documentation.

## [0.16.5] - 2026-04-01

### Added

- Matrix functions:
  - `transpose_mul_vec`
  - `mul_diagonal`
  - `from_scale`
  - `from_translation`
  - `from_submatrix`
  - `from_submatrix_translation`
  - `submatrix`
  - `remove`
  - `transform_point`
  - `transform_vector`
  - `is_nan`
  - `is_finite`
  - `determinant`
  - `inverse`
  - `try_inverse`
  - `inverse_or`
  - `inverse_or_zero`
  - `recip`
  - `abs`
  - `from_angle`
  - `from_scale_angle`
  - `from_scale_angle_translation`
  - `from_rotation_x`
  - `from_rotation_y`
  - `from_rotation_z`
  - `from_quat`
  - `from_axis_angle`
  - `from_euler`
  - `look_to_lh`
  - `look_to_rh`
  - `look_at_lh`
  - `look_at_rh`
  - `to_euler`
  - `from_rotation_translation`
  - `from_scale_rotation_translation`
  - `perspective_lh`
  - `perspective_rh`
  - `perspective_rh_gl`
  - `perspective_infinite_lh`
  - `perspective_infinite_rh`
  - `perspective_infinite_reverse_lh`
  - `perspective_infinite_reverse_rh`
  - `frustum_lh`
  - `frustum_rh`
  - `frustum_rh_gl`
  - `orthographic_lh`
  - `orthographic_rh`
  - `orthographic_rh_gl`
  - `to_scale_rotation_translation`
  - `project_point`

- `abs_diff_eq` for all floating-point types.

- `EulerRot` enum.

### Changed

- Updated matrix documentation.

## [0.16.4] - 2026-03-22

### Added

- Matrix multiplication and scalar division.

- Matrix functions:
  - `from_column_array`
  - `transpose`

- `Hash` implementation for matrices and affines.

- `Default` implementation for matrices and affines.

- Operator implementations for vector and mask references.

- `Scalar` overridable functions:
  - `vec_element_sum`
  - `vec_element_product`
  - `mat_mul_scalar`
  - `mat_mul_vec`
  - `mat_mul`

### Changed

- Expand primitive-only vector functions to any scalar:
  - `element_sum`
  - `element_product`
  - `dot`
  - `length_squared`
  - `perp`
  - `cross`

## [0.16.3] - 2026-03-14

### Added

- Vector function `new`.

- Quaternion & Affine function `to_repr`.

### Changed

- Renamed matrix and affine functions:
  - `from_col_array` to `from_columns`
  - `from_col_fn` to `from_column_fn`
  - `as_col_array_ref` to `as_columns`
  - `as_col_array_mut` to `as_columns_mut`
  - `col` to `column`
  - `col_mut` to `column_mut`

- Replaced `Affine::from_cols` with `Affine::from_columns`.

- Fused `f32` and `f64` functionality into a single generic implementation. This
  doesn't have much effect except for making generated documentation clearer.

- Updated documentation.

### Deprecated

- Macros `vec2`, `vec3`, `vec4`, `mat2`, `mat3`, `mat4`. Use `Vector::new` and
  `Matrix::from_columns` instead.

- `Matrix::to_col_array`: replaced by `Matrix::as_columns`.

## 0.16.2 (16.2.2026)

- Added affine types:
  - `Affine2`
  - `Affine3`
  - `Affine2U`
  - `Affine3U`
  - `Affine`

- Added partial support for crates: `fixed`, `fixp`, `wide`.

- Modified documentation.

- Updated dependencies.

## 0.16.1 (14.2.2026)

- Added matrix types:
  - `Mat2`
  - `Mat3`
  - `Mat4`
  - `Mat2U`
  - `Mat3U`
  - `Mat4U`
  - `Matrix`

- Added quaternion types:
  - `Quat`
  - `QuatU`
  - `Quaternion`

- Added mask function: `to_repr`.

- Fixed vague safety documentation.

- Improved documentation.

## 0.16.0 (6.2.2026)

Breaking changes:

- Renamed the `NaN` trait to `Nan`.

- Removed vector functions: `get`, `get_mut`.

- Changed signature of vector functions: `from_fn`, `map`.

- Marked the `Scalar` trait unsafe.

- Replaced associated type `ScalarBackend::VectorRepr` with `Scalar::Repr`.
  
- Removed the `ScalarDefault` trait.

- Replaced vector functions `repr` and `from_repr` with `to_repr`.

Non breaking changes:

- Added mask types:
  - `Mask2`
  - `Mask3`
  - `Mask4`
  - `Mask2U`
  - `Mask3U`
  - `Mask4U`
  - `Mask`

- Added vector functions:
  - `eq_mask`
  - `ne_mask`
  - `lt_mask`
  - `gt_mask`
  - `le_mask`
  - `ge_mask`

- Added float vector functions:
  - `nan_mask`
  - `finite_mask`
  - `sign_positive_mask`
  - `sign_negative_mask`

- Added `ScalarBackend` functions:
  - `vec_eq_mask`
  - `vec_ne_mask`
  - `vec_lt_mask`
  - `vec_gt_mask`
  - `vec_le_mask`
  - `vec_ge_mask`

- Improved documentation.

## 0.15.1 (29.1.2025)

- Added a `FloatExt` trait.

- Added float vector functions:
  - `angle_between`
  - `clamp_length`
  - `cross`
  - `distance`
  - `distance_squared`
  - `exp`
  - `exp2`
  - `is_normalized`
  - `lerp`
  - `ln`
  - `log2`
  - `midpoint`
  - `move_towards`
  - `normalize_and_length`
  - `perp`
  - `powf`
  - `rotate`
  - `rotate_x`
  - `rotate_y`
  - `rotate_z`
  - `sin_cos`
  - `with_max_length`
  - `with_min_length`
  - `project_onto`
  - `project_onto_normalized`
  - `reject_from`
  - `reject_from_normalized`
  - `reflect`
  - `refract`
  - `any_orthogonal_vector`
  - `any_orthonormal_vector`
  - `any_orthonormal_pair`

- Added int vector functions:
  - `element_sum`
  - `element_product`
  - `max`
  - `min`
  - `clamp`
  - `max_element`
  - `min_element`
  - `abs`
  - `signum`
  - `dot`
  - `length_squared`
  - `distance_squared`
  - `perp`
  - `cross`
  - `checked_add`
  - `checked_sub`
  - `checked_mul`
  - `checked_div`
  - `checked_rem`
  - `saturating_add`
  - `saturating_sub`
  - `saturating_mul`
  - `saturating_div`
  - `wrapping_add`
  - `wrapping_sub`
  - `wrapping_mul`
  - `wrapping_div`
  - `wrapping_rem`

- Added uint vector functions:
  - `element_sum`
  - `element_product`
  - `max`
  - `min`
  - `clamp`
  - `max_element`
  - `min_element`
  - `dot`
  - `length_squared`
  - `checked_add`
  - `checked_sub`
  - `checked_mul`
  - `checked_div`
  - `checked_rem`
  - `saturating_add`
  - `saturating_sub`
  - `saturating_mul`
  - `wrapping_add`
  - `wrapping_sub`
  - `wrapping_mul`

- Added bool vector functions: `all`, `any`.

- Added support for crates: `bytemuck`, `libm`, `mint`, `serde`.

- Added missing `must_use` to functions.

- Modified documentation.

## 0.15.0

This update attempts to make the crate more stable so that more math
functionality can start being added.

- Moved all items from `crate::vector` to the crate root.

- Replaced primitive type-aliases with generic ones.

- Made `ScalarBackend` unsafe.

- Added `ScalarDefault`.

- Changed vector memory layout rules.

- Added float vector functions: `is_nan`, `is_finite`, `normalize_or_zero`.

- Added bool vector functions: `select`.

- Added constants:
  - `MIN`
  - `MAX`
  - `NAN`
  - `INFINITY`
  - `NEG_INFINITY`
  - `TRUE`
  - `FALSE`

- Removed vector functions:
  - `len`
  - `is_aligned`
  - `swizzle2`
  - `swizzle3`
  - `swizzle4`

- Fixed slight inconsistency between `normalize` and `try_normalize`.

- Removed direction constants (`RIGHT`, `LEFT`, etc..).

- Removed features `overflow_checks` and `debug_overflow_checks`.

- Replaced feature `debug_assertions` with `no-assertions`.

- Improved performance.

- Improved documentation.

## 0.14.0

- Simplified API.

- Added methods to float vectors: `normalize_or`.

- Removed support for element types: `Option<T>`, `MaybeUninit<T>`.

## 0.13.1

- Improved documentation.
- Added methods to float vectors:
  - `dot`
  - `length`
  - `length_squared`
  - `normalize`
  - `try_normalize`

## 0.13.0

This version completely rewrote the crate.

[unreleased]: https://github.com/Noam2Stein/ggmath/compare/v0.16.7...HEAD
[0.16.7]: https://github.com/Noam2Stein/ggmath/compare/v0.16.6...v0.16.7
[0.16.6]: https://github.com/Noam2Stein/ggmath/compare/v0.16.5...v0.16.6
[0.16.5]: https://github.com/Noam2Stein/ggmath/compare/v0.16.4...v0.16.5
[0.16.4]: https://github.com/Noam2Stein/ggmath/compare/v0.16.3...v0.16.4
[0.16.3]: https://github.com/Noam2Stein/ggmath/compare/v0.16.2...v0.16.3
