# Changelog

## 0.15.1 (Unreleased)

- Added bool vector functions: `all`, `any`.

- Added support for crates: `bytemuck`, `mint`, `serde`.

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
