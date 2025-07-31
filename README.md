# GGMath

## Development Status

Usability:

| API         | Stable | 100% Documentation | 100% Tests | 100% Benchmarks |
|-------------|--------|--------------------|-------------|----------------|
| Vectors     | ✅    | ✅                 | ❌         | ❌             |
| Matrices    | ❌    | ❌                | ❌          | ❌            |
| Quaternions | ❌    | ❌                | ❌          | ❌            |
| Aabbs       | ✅    | ❌                | ❌          | ❌            |

Performance:

| API              | Is On Par With `glam` |
|------------------|-----------------------|
| Vector (swizzle) | ❌                   |
| Vector (floats)  | ❌                   |
| Vector (ints)    | ❌                   |
|                  |                       |
| Matrix (swizzle) | ❌                   |
|                  |                       |
| Aabb             | ❌                   |

### GGMath

A *generic graphics math* Rust crate with precise, flexible generic math types specialized for graphics.

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

The `Vector` type is generic over length, type,
and whether it's aligned for fast operations, or unaligned to save space.

The `Matrix` type is generic over column count, row count, type, alignment,
and whether it's column-major or row-major.

The `Aabb` type is generic over length, type, alignment,
and it's inner representation (by min+size, min+max, or center+extents).

### Vectors

```rust
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,

// `VecAligned` vectors are the default.
// They are aligned for fast operators (SIMD)
pub type Vec2<T> = Vector<2, T, VecAligned>;
pub type Vec3<T> = Vector<3, T, VecAligned>;
pub type Vec4<T> = Vector<4, T, VecAligned>;

// `VecPacked` vectors have the same memory layout as `[T; N]`.
// This saves space, but makes operators slower.
pub type Vec2P<T> = Vector<2, T, VecPacked>;
pub type Vec3P<T> = Vector<3, T, VecPacked>;
pub type Vec4P<T> = Vector<4, T, VecPacked>;
```

Vectors support all expected features:
- Swizzling: `vec.xyw()`, `vec4!(1, vec2!(2, 3), 4)`
- Arithmetic: `vec + vec`, `vec * 2.0`
- Math: `vec.mag()`, `vec.dot(vec)`, `vec.cross(vec)`, `vec.lerp(vec, t)`
- Array-like API: `vec[0]`, `vec.map(...)`, `vec.get()`
- Nice type aliases: `FVec3`, `IVec4`, `BVec2P`, etc.

### Matrices (optional feature)

```rust
pub struct Matrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,

pub type Mat2C<T> = Matrix<2, 2, T, VecAligned, ColMajor>;
pub type Mat2x3C<T> = Matrix<2, 3, T, VecAligned, ColMajor>;

pub type Mat2R<T> = Matrix<2, 2, T, VecAligned, RowMajor>;

pub type Mat2CP<T> = Matrix<2, 2, T, VecPacked, ColMajor>;

pub type Mat2RP<T> = Matrix<2, 2, T, VecPacked, RowMajor>;

// ...
```

Matricies are not feature full yet.
They do support swizzling and construction from columns/rows.

### Aabbs (Bounding Boxes) (optional feature)

```rust
pub struct Aabb<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr>
where
    Usize<N>: VecLen,

// `AabbCornered` is the default aabb representation.
// `AabbCornered` aabbs are stored by their minimum corner and their size.
pub type Rect<T> = Aabb<2, T, VecAligned, AabbCornered>;
pub type RectP<T> = Aabb<2, T, VecPacked, AabbCornered>;

// `AabbCentered` aabbs are stored by their center and their extents.
pub type RectC<T> = Aabb<2, T, VecAligned, AabbCentered>;

// `AabbMinMaxed` aabbs are stored by their minimum and maximum corners.
pub type RectM<T> = Aabb<2, T, VecAligned, AabbMinMaxed>;

// 3D
pub type Aabb3<T> = Aabb<3, T, VecAligned, AabbCornered>;

pub type Aabb3M<T> = Aabb<3, T, VecAligned, AabbMinMaxed>;

// 4D

pub type Aabb4<T> = Aabb<4, T, VecAligned, AabbCornered>;
```

Aabbs support all expected features:
- Construction: `Aabb::from_min_size`, `Aabb::from_min_max`, `Aabb::from_center_size` etc.
- Accessors: `aabb.min()`, `aabb.max()`, `aabb.center()`, `aabb.extents()` etc.
- Logic: `aabb.contains(point)`, `aabb.intersects(aabb)` etc.
- Swizzling: `aabb.xy()`, `aabb.xyz()` etc.
- Nice type aliases: `FAabb3`, `IRectMP`, etc.

### Quaternions (optional feature)

```rust
pub struct Quaternion<T: Scalar, A: VecAlignment>

pub type Quat<T> = Quaternion<T, VecAligned>;
pub type QuatP<T> = Quaternion<T, VecPacked>;

```

### Crate Integration

`ggmath` currently integrates with `serde` and `crevice`.
