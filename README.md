# ggmath

A math crate for games and graphics.

> **Note:** This crate is a work in progress, and several features are still
> missing.

This crate is similar to most math libraries, with types like `Vec2`, `Vec3`,
`IVec3`, and SIMD-aligned types such as `Vec3A`.

```rust
use ggmath::{Vec2, Vec3, Vec3A, Vec4, Vec4A, vec2, vec3, vec4};

let v2: Vec2 = vec2!(1.0, 2.0);
let v3: Vec3 = vec3!(1.0, 2.0, 3.0);
let v4: Vec4 = vec4!(1.0, 2.0, 3.0, 4.0);

// SIMD-aligned vectors
let v3a: Vec3A = vec3!(1.0, 2.0, 3.0);
let v4a: Vec4A = vec4!(1.0, 2.0, 3.0, 4.0);

// Math
let result = v2.abs() * 2.0 + v3.xy();
```

## Generics

Types like `Vec2` and `IVec3` are aliases for the generic type `Vector<N, T,
A>`, which enables:

- Custom element types (e.g., `Vector<N, MyScalar, A>`)
- Generic code over:
  - The number of elements in the vector (`N`)
  - The type of elements in the vector (`T`)
  - Whether or not the vector is SIMD-aligned (`A`)

## SIMD

This crate provides both scalar-backed types and SIMD-aligned types:

- `Vec2`, `Vec3`, `Vec4`, etc., are `Vector<N, T, Unaligned>`
- `Vec3A`, `Vec4A`, etc., are `Vector<N, T, Aligned>`

SIMD types make operations faster, but have higher memory alignment.

## Development Status

### Missing Features

- Vectors:
  - Geometric floats functions
  - Integer functions
  - Bool functions
- Matrices
- Quaternions
- Masks
- `libm` support

### Performance

`ggmath` is designed to match the performance of other SIMD-optimized math
crates (e.g., **[glam](https://github.com/bitshifter/glam-rs)**), although many
optimizations are still missing.

| | x86 | ARM | WASM |
| ----- | --- | --- | ---- |
| `f32` | ✅ | ❌ | ❌ |
| `f64` | ❌ | ❌ | ❌ |
| `i32` | ❌ | ❌ | ❌ |
| `i64` | ❌ | ❌ | ❌ |
| `u32` | ❌ | ❌ | ❌ |
| `u64` | ❌ | ❌ | ❌ |

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ggmath = "0.15.0"
```

Make sure Rust is at least at version `1.90.0`.

### `no_std`

```toml
[dependencies]
ggmath = { version = "0.15.0", default-features = false, features = ["debug_assertions", "debug_overflow_checks"] }
```

> **Note:** `libm` support is currently missing.

### Default Features

- **`std`**: Uses `std` as the backend for float ops.

- **`debug_assertions`**: Enables assertions in debug builds to help catch bugs,
  though they can slow down functions.

- **`debug_overflow_checks`**: Enables overflow checks in debug builds for
  integer types. It should be set to match the `-Z overflow-checks` setting
  passed to `rustc` in order to work properly.

### Optional Features

- **`assertions`**: Enables assertions in both debug and release builds.

- **`overflow_checks`**: Enables overflow checks in both debug and release
  builds.
