# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

This version changes the set of types provided by the library. This is a big
breaking change, so expect existing code to fail compilation. These changes aim
to simplify the API, see
[this issue](https://github.com/Noam2Stein/ggmath/issues/52).

If you use `Mat4` for 3D projections, or `Mat3` for 2D projections, you should
switch to the new `Proj3` and `Proj2` types. From now on, normal matrices are
only used for linear transformations in `N` dimensions, and not projective
transformations in `N-1` dimensions.

From now on, there should be way less breaking changes. Future breaking changes
will most likely consist of minor function renames, optimizations that are
slightly breaking, and simplifying the API when new Rust features are
stabilized, which should not affect most users.

### Added

- `Projective<N, T, A>` type with `Proj2<T>`, `Proj3<T>`, `Proj2A<T>` and
  `Proj3A<T>` type aliases

- `to_homogeneous` and `from_homogeneous` for 2D vectors

### Changed

- `Mat4` functions for 3D transformations, and `Mat3` functions for 2D
  transformations. These have been removed and moved into the new `Proj3` and
  `Proj2` types

- The affine transform field `submatrix` has been renamed to `matrix`. Many
  related functions have also been renamed with `submatrix` to `matrix`

- Most functions for converting between transform types. The function you
  want has most likely been renamed into one of these new functions:
  - `to/from_matrix`
  - `to/from_affine`
  - `to/from_projective`
  - `to/from_homogeneous`

- `rustdoc` documentation now shows all integer vector functions as generic over
  `T`, even though some of them (ones with names that conflict with float
  functions) are not actually generic.

### Fixed

- Previously, the padding of `Affine2A<f32>` (which contains four elements for
  `Mat2A`, two elements for `Vec2A`, and two padding elements) contained
  uninitialized memory. This made the implementations of `bytemuck` traits
  incorrect, whicn could easily lead to **undefined behavior**. That padding is
  now guaranteed to be initialized, by making the definition of `Affine`
  more similar to `Vector<N, T, A>`.

- Improved the order of `impl` blocks in the `docs.rs` page. Due to a `rustdoc`
  bug, the order is still not perfect.

- Added missing `#[inline]` and `#[must_use]` attributes for
  `Vector::from_homogeneous`

### Removed

- Removed the optional dependency `num-primitive`. If you want access to both
  `ggmath` primitive functionality and the `num-primitive` API, the better
  pattern is to define your own `Primitive*` traits.

## [0.17.1] - 2026-07-26

### Changed

- Renamed conversion functions:
  - `as_array_ref` to `as_array`
  - `as_array_mut` to `as_mut_array`
  - `as_rows_mut` to `as_mut_rows`
  - `as_vector_ref` to `as_vector`
  - `as_vector_mut` to `as_mut_vector`

## [0.17.0] - 2026-07-10

### Added

- Functions:
  - `Vector::wedge`
  - `Vector::to_homogeneous`
  - `Vector::from_homogeneous`
  - `Vector::slerp`
  - `Vector::rotate_towards`
  - `Vector::angle_to`
  - `Vector::angle_from`
  - `Vector::to_bits`
  - `Vector::from_bits`
  - `Vector::is_positive`
  - `Vector::is_negative`
  - `Vector::cast_unsigned`
  - `Vector::cast_signed`
  - `Matrix::translation`
  - `Mat3::to_scale_angle`
  - `Mat4::to_scale_rotation`
  - `Affine::from_column_fn`
  - `Affine2::to_scale_angle`
  - `Affine3::to_scale_rotation`
  - `FloatExt::move_towards`

- Primitive traits:
  - `PrimitiveFloat`
  - `PrimitiveInteger`
  - `PrimitiveSigned`
  - `PrimitiveUnsigned`

- Implementation of common traits for `Alignment` marker types.

- Functions for all `wide` types:
  - `from_lanes`
  - `from_lane_fn`
  - `to_lanes`
  - `lane`
  - `set_lane`
  - `simd_eq`
  - `simd_ne`

- All remaining `wide` functions.

- Support for crate `num-primitive`.

### Changed

- Changed the panic conditions of many functions and improved panic messages.

- Changed `Vector::normalize_and_length` to not handle the zero-vector case.

- Made `Matrix::transform_point/vector` specific to floats.

- Moved traits `Zero`, `One` and `NegOne` to the crate root and removed their
  `Scalar` super-trait.

- Renamed `Affine::to_matrix` to `Matrix::from_affine`.

- Changed matrix `to_scale_angle_translation` and
  `to_scale_rotation_translation` to be consistent with submatrix functions.

- Changed conventions from column-major to row-major. This includes function
  renames and trait implementations.

- Removed panics from all `wide` functions.

- Made vector functions generic over `N`:
  - `any_orthogonal_vector`
  - `any_orthonormal_vector`

- Removed `ScalarBackend` and added `CustomScalar`.

- Made `Vector::distance_squared` generic over `T`.

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

- Fixed rare panic in quaternion function `from_scaled_axis`.

- Fixed the behavior of quaternion functions `from_rotation_arc` and
  `from_rotation_arc_colinear` for specific edge case.

- Fixed `Vector/Quaternion::angle_between` edge case.

- Corrected value of `NAN` constant for `wide` types was previously incorrect.

- Fixed `Vec4::<{wide-float}>::rotate_towards` edge case.

### Removed

- The `std` feature flag.

- `Quaternion::canonical`.

- Constant traits `Min`, `Max`, `Nan`, `Infinity`, `NegInfinity`, `False` and
  `True`.

- `wide` functions `try_normalize` and `try_inverse`.

- Support for crate `fixp`.

- Feature flags `assertions` and `no-assertions`. Debug assertions can be
  enabled and disabled per crate through Cargo.

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

[unreleased]: https://github.com/Noam2Stein/ggmath/compare/v0.17.1...HEAD
[0.17.1]: https://github.com/Noam2Stein/ggmath/compare/v0.17.0...v0.17.1
[0.17.0]: https://github.com/Noam2Stein/ggmath/compare/v0.16.7...v0.17.0
[0.16.7]: https://github.com/Noam2Stein/ggmath/compare/v0.16.6...v0.16.7
[0.16.6]: https://github.com/Noam2Stein/ggmath/compare/v0.16.5...v0.16.6
[0.16.5]: https://github.com/Noam2Stein/ggmath/compare/v0.16.4...v0.16.5
[0.16.4]: https://github.com/Noam2Stein/ggmath/compare/v0.16.3...v0.16.4
[0.16.3]: https://github.com/Noam2Stein/ggmath/compare/v0.16.2...v0.16.3
