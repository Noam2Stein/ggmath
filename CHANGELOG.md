# Changelog

## 0.16.0 (Unreleased)

Breaking changes:

- Renamed the `NaN` trait to `Nan`.

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
