*** THIS CRATE HAS NO UNIT TESTS YET AND MAY STILL HAVE BUGS ***

# GGMath

A *generic graphics math* Rust crate with precise generic math types specialized for graphics.

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

Vectors have the type aliases:
- `Vec{2/3/4}<T>` => `Vector<{2/3/4}, T, VecAligned>`
- `Vec{2/3/4}P<T>` => `Vector<{2/3/4}, T, VecPacked>`
- T specifc aliases through the default `primitive_aliases` feature.

### Matrices

Matrices are generic over:
- **Columns** (`C`): 2, 3, or 4 (number of columns)
- **Rows** (`R`): 2, 3, or 4 (number of rows)
- **Type** (`T`): e.g., `f32`, `i32`, `bool`, etc. (element type)
- **Alignment** (`A`): `VecAligned` or `VecPacked`
- **Major Axis** (`M`): `ColumnMajor` or `RowMajor`

```rust
use ggmath::*;

// Column-major, aligned matrix
type Mat4x3F32Col = Matrix<4, 3, f32, VecAligned, ColumnMajor>; // Could just write `FMat4x3C`

// Row-major, packed matrix
type Mat2x2I32RowPacked = Matrix<2, 2, i32, VecPacked, RowMajor>; // Could just write `IMat2RP`
```

Matricies have the type aliases:
- `Mat{2/3/4}C<T>` => `Matrix<{2/3/4}, {2/3/4}, T, VecAligned, ColumnMajor>`
- `Mat{2/3/4}R<T>` => `Matrix<{2/3/4}, {2/3/4}, T, VecAligned, RowMajor>`
- `VecPacked` versions
- type specific versions via the default `primitive_aliases` feature.

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
type RectF32 = Rectangle<2, f32, VecAligned, RectCornered>;

// MinMaxed: min and max (min, max)
type BoxI32Packed = Rectangle<3, i32, VecPacked, RectMinMaxed>;

// Centered: center and extents (center, extents)
type HyperBoxF64Packed = Rectangle<4, f64, VecPacked, RectCentered>;
```

#### Rectangle Type Aliases

```rust
use ggmath::*;

let r: IRect2 = IRect2::from_min_max(vec2!(1, 1), vec2!(3, 3)); // FRect2 = Rectangle<2, f32, VecAligned, RectCornered>
```

---

## Stability & Development Status

- actively updated.
- non final API.
- there are incomplete features (like quaternions).
- there is missing documentation.
- there may be bugs because there are no tests yet.
