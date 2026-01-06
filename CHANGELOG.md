# Changelog

## 0.15.0

- Improved performance.
- Removed vector functions:
  - `swizzle2`
  - `swizzle3`
  - `swizzle4`
- Removed reexport of `crate::vector::*`
- Renamed type aliases.

## 0.14.0

- Simplified API.
- Added methods to float vectors:
  - `normalize_or`
- Removed support for element types:
  - `Option<T>`
  - `MaybeUninit<T>`

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
