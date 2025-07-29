### Development Status

Vectors are mostly stable, but other types are missing some functionality and some are missing documentation.

Unit tests don't cover all functionality yet, and there are no benchmarks.

# GGMath

A *generic graphics math* Rust crate with precise generic math types specialized for graphics.

`ggmath` has vectors, and optionally matrices, quaternions, and aabbs.

```rust
use ggmath::*;

fn main() {
    let mut vec4 = vec4!(1, 2, 3, 4);
    vec4.set_xz(vec2!(5, 6));

    println!("{}", vec4.xyw());
}
```

`ggmath` types are fully generic over absolutely everything.
There is only one type for `Vector` and `Matrix`, and they are both generic over length, type and *alignment*.

`ggmath` types are generic over their memory-layout,
which includes *alignment* (see <https://doc.rust-lang.org/reference/type-layout.html>).

The minimum alignment of a vector is the alignment of its `T`,
but you may want higher alignment so that operators on the vector are faster due to SIMD.
Because of this, all `ggmath` types are generic over `A: VecAlignment` which can be either `VecAligned` (aligned for SIMD) or `VecPacked` (not additionally aligned, like `[T; _]`).

`ggmath` matricies are also generic over whether they are column-major or row-major.

### Vectors

Vectors are generic over:
- **Length** (`N`): 2, 3, or 4 (vector dimension)
- **Type** (`T`): e.g., `f32`, `i32`, `bool`, etc. (element type)
- **Alignment** (`A`): `VecAligned` (aligned for SIMD) or `VecPacked` (not additionally aligned, like `[T; _]`)

```rust
use ggmath::*;

// This type is aligned for optimal SIMD usage.
// On most platforms:
//   SIZE      -> 128 bits
//   ALIGNMENT -> 128 bits
type SimdVec3 = Vector<3, f32, VecAligned>; // Could also just write `FVec3`

// This type is not specially aligned and is identical in memory to [f32; 3].
// On most platforms:
//   SIZE      -> 96 bits
//   ALIGNMENT -> 32 bits (f32's alignment)
type PackedVec3 = Vector<3, f32, VecPacked>; // Could also just write `FVec3P`
```

Vectors have type aliases `Vec{2/3/4}` style where `VecAligned` is the default alignment, and `VecPacked` is `Vec{2/3/4}P`.

The `primitive_aliases` feature adds type aliases for Rust primitives (e.g., `FVec3`).

### Matrices

Matrices are generic over:
- **Columns** (`C`): 2, 3, or 4 (number of columns)
- **Rows** (`R`): 2, 3, or 4 (number of rows)
- **Type** (`T`): e.g., `f32`, `i32`, `bool`, etc. (element type)
- **Alignment** (`A`): `VecAligned` or `VecPacked`
- **Major Axis** (`M`): `ColMajor` or `RowMajor`

```rust
use ggmath::*;

// Column-major, aligned matrix
type Mat4x3F32Col = Matrix<4, 3, f32, VecAligned, ColMajor>; // Could just write `FMat4x3C`

// Row-major, packed matrix
type Mat2x2I32RowPacked = Matrix<2, 2, i32, VecPacked, RowMajor>; // Could just write `IMat2RP`
```

Matricies have type aliases `Mat{2/3/4}{C/R}` style which end with their major axis.

### Bounding Boxes (optional feature)

The `aabb` feature adds support for axis-aligned bounding boxes.
Aabbs can be represented in multiple ways, like by their minimum and maximum corners, or their center and size, etc.
Because of this, `ggmath`'s aabbs are generic over their internal representation.

Aabbs are generic over:
- **Length** (`N`): number of dimensions (e.g., 2 for rectangles, 3 for boxes, etc.)
- **Type** (`T`): e.g., `f32`, `i32`, etc. (element type)
- **Alignment** (`A`): `VecAligned` or `VecPacked`
- **Representation** (`R`): e.g., `AabbCornered`, `AabbMinMaxed`, `AabbCentered`

```rust
use ggmath::*;

// Cornered: minimum corner and size
type RectF32 = Aabb<2, f32, VecAligned, AabbCornered>;

// MinMaxed: minimum and maximum corners
type BoxI32Packed = Aabb<3, i32, VecPacked, AabbMinMaxed>;

// Centered: center and extents (center, extents)
type HyperBoxF64Packed = Aabb<4, f64, VecPacked, AabbCentered>;
```

Aabbs have type aliases where `Aabb<2, ..>` is named `Rect` unlike `Aabb{3/4}`.
`AabbCornered` is the default representation and `Rect{C/M}` stand for `AabbCentered` / `AabbMinMaxed`.
