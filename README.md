*** UNTESTED CRATE ***

# GGMath

*GGMath* is a **generic graphics math** crate with optimized, flexible math types for Rust.

GGMath types are generic over:
- **Alignment**: Choose between SIMD-optimized (`VecAligned`) or compact (`VecPacked`) memory layouts for vectors and matrices.
- **Memory Representation**: Types are generic over row-major/column-major for matrices, and (optionally, via the `rectangle` feature) over rectangle representations for rectangles.

This allows you to control performance and memory layout with a simple, ergonomic API.

### Vectors

Vectors are generic over:
- **Length** (`N`): 2, 3, or 4 (vector dimension)
- **Type** (`T`): e.g., `f32`, `i32`, `bool`, etc. (element type)
- **Alignment** (`A`): `VecAligned` (SIMD-optimized) or `VecPacked` (compact)

```rust
use ggmath::*;

// This type is aligned for optimal SIMD usage.
// On most platforms:
//   SIZE      -> 128 bits
//   ALIGNMENT -> 128 bits
type SimdVec3 = Vector<3, f32, VecAligned>;

// This type is not specially aligned and is identical to [f32; 3].
// On most platforms:
//   SIZE      -> 96 bits
//   ALIGNMENT -> 32 bits (f32's alignment)
type PackedVec3 = Vector<3, f32, VecPacked>;
```

#### Vector Type Aliases

```rust
use ggmath::*;

let v2: Vec2<f32> = vec2!(1.0, 2.0); // Vec2<T> = Vector<2, T, VecAligned>

let v4: Vec4P<f32> = vec4p!(1.0, v2, 4.0); // Vec4P<T> = Vector<2, T, VecPacked>

let v: FVec3P = vec3p!(v4.yz(), 0.0); // FVec3P = Vector<3, f32, VecPacked>
```

### Matrices

Matrices are generic over:
- **Columns** (`C`): 2, 3, or 4 (number of columns)
- **Rows** (`R`): 2, 3, or 4 (number of rows)
- **Type** (`T`): e.g., `f32`, `i32`, `bool`, etc. (element type)
- **Alignment** (`A`): `VecAligned` or `VecPacked`
- **Major Axis** (`M`): `ColumnMajor` or `RowMajor` (memory layout)

```rust
use ggmath::*;

// Column-major, aligned matrix
type Mat4x3F32Col = Matrix<4, 3, f32, VecAligned, ColumnMajor>;

// Row-major, packed matrix
type Mat2x2I32RowPacked = Matrix<2, 2, i32, VecPacked, RowMajor>;
```

#### Matrix Type Aliases

```rust
use ggmath::*;

let short: FMat4C = FMat4C::default(); // FMat4C = Matrix<4, 4, f32, VecAligned, ColumnMajor>

let loong: UsizeMat4x3RP = UsizeMat4x3RP::default(); // UsizeMat4x3RP = Matrix<4, 3, f32, VecPacked, RowMajor>
```

### Rectangles (optional feature)

Rectangles are generic over:
- **Length** (`N`): number of dimensions (e.g., 2 for rectangles, 3 for boxes, etc.)
- **Type** (`T`): e.g., `f32`, `i32`, etc. (element type)
- **Alignment** (`A`): `VecAligned` or `VecPacked`
- **Representation** (`R`): e.g., `RectCornered`, `RectMinMaxed`, `RectCentered`, etc. (internal layout)

Rectangle support is an optional feature. Enable the `rectangle` feature in your Cargo.toml to use generic rectangles and boxes.

```rust
use ggmath::*;

// Cornered: min coords and size (corner, size)
type RectF32 = Rectangle<f32, VecAligned, RectCornered>;

// MinMaxed: min and max (min, max)
type RectI32Packed = Rectangle<i32, VecPacked, RectMinMaxed>;

// Centered: center and extents (center, extents)
type RectF64Packed = Rectangle<f64, VecPacked, RectCentered>;
```

#### Rectangle Type Aliases

```rust
use ggmath::*;

let r: FRect = FRect::new(0.0, 0.0, 1.0, 1.0); // FRect = Rectangle<f32, VecAligned, RectXYWH>
```

### SIMD Usage

All GGMath types are designed to be zero-cost abstractions in release mode: the compiler will optimize generic math code to use SIMD instructions. In debug mode, generic abstractions may result in slower code due to lack of optimizations, but in release mode, you get full SIMD performance with no overhead.

---

## Stability & Development Status

**GGMath is unstable and under active development.**

- The API is not finalized and may change at any time.
- Every version may introduce breaking changes.
- There may be bugs, incomplete features, or missing documentation.
