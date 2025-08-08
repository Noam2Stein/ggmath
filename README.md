# GGMath

## Development Status

Usability:

| API         | Stable | 100% Documentation | 100% Tests | 100% Benchmarks |
|-------------|--------|--------------------|------------|-----------------|
| Vectors     | ✅    | ✅                 | ✅        | ❌              |
| Matrices    | ❌    | ✅                 | ❌        | ❌              |
| Quaternions | ❌    | ✅                 | ❌        | ❌              |
| Aabbs       | ✅    | ✅                 | ❌        | ❌              |

Performance:

| API              | Is 100% On Par With `glam` |
|------------------|----------------------------|
| Vector (swizzle) | ❌                        |
| Vector (floats)  | ❌                        |
| Vector (ints)    | ❌                        |
|                  |                            |
| Matrix (swizzle) | ❌                        |
|                  |                            |
| Aabb             | ❌                        |
| Quaternion       | ❌                        |

`ggmath` is benchmarked against `glam`.
If a feature is considered "on par",
it means that all functions have been benchmarked and reached `glam`'s performance.

### GGMath

A *generic graphics math* Rust crate with generic math types and full SIMD support.

`ggmath` has vectors, matrices, quaternions, and aabbs.

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
and whether it's aligned for SIMD, or unaligned to save space.

The `Matrix` type is generic over column count, row count, type, alignment (like vectors),
and whether it's column-major or row-major.

The `Aabb` type is generic over dimension count, type, alignment,
and it's inner representation (represented by min+size, min+max, or center+extents).

### Vectors

```rust
// Vector's declaration
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,

// `A` is a generic marker type that affects the memory alignment of the vector.
// See <https://doc.rust-lang.org/reference/type-layout.html> for rust's type layout.
//
// `A` can be set to either `VecAligned` or `VecPacked`.
//
// Memory alignment can significantly affect performance when processing large datasets.
//
// `VecAligned` vectors are optimized for SIMD instructions,
// which can provide a speed boost for large-scale computations.
//
// `VecPacked` saves space by avoiding padding,
// which is useful for memory-limited applications like asset streaming or network data transmission.

pub type Vec2<T> = Vector<2, T, VecAligned>;
pub type Vec3<T> = Vector<3, T, VecAligned>;
pub type Vec4<T> = Vector<4, T, VecAligned>;

pub type Vec2P<T> = Vector<2, T, VecPacked>;
pub type Vec3P<T> = Vector<3, T, VecPacked>;
pub type Vec4P<T> = Vector<4, T, VecPacked>;

// All vectors share their API, regardless of `VecAligned` / `VecPacked`.
```

Examples:

```rust
// In this scenario, it is more efficient to use `VecAligned` vectors.
// This will ensure that operations on the vectors are fast.
struct MyPhysicsObject {
    position: Vec3<f32>,
    velocity: Vec3<f32>,
}

// In this scenario, it is more efficient to use `VecPacked` vectors.
// This will ensure we don't waste any space on padding.
struct MyVertexBuffer {
    vertices: [Vec3P<f32>; 1000],

    // This struct has a size of `12_000` bytes.
    // If we used `VecAligned` vectors, it would be `16_000` bytes.
    // That would be a 33% memory overhead.
}
```

Vectors support all expected features:
- Swizzling: `vec.xyw()`, `vec4!(1, vec2!(2, 3), 4)`
- Arithmetic: `vec + vec`, `vec * 2.0`
- Math: `vec.mag()`, `vec.dot(vec)`, `vec.cross(vec)`, `vec.lerp(vec, t)`
- Array-like API: `vec[0]`, `vec.map(...)`, `vec.get()`
- Nice type aliases: `FVec3`, `IVec4`, `BVec2P`, etc.

### Matrices

```rust
// Matrix's declaration
pub struct Matrix<
    const C: usize,
    const R: usize,
    T: Scalar,
    A: VecAlignment,
    M: MatMajorAxis,
>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,

// Most libraries only support either column-major or row-major matrices.
// This matrix type supports both, which makes it compatible with all libraries.

pub type Mat2C<T> = Matrix<2, 2, T, VecAligned, ColMajor>;
pub type Mat2x3C<T> = Matrix<2, 3, T, VecAligned, ColMajor>;

pub type Mat2R<T> = Matrix<2, 2, T, VecAligned, RowMajor>;

pub type Mat2CP<T> = Matrix<2, 2, T, VecPacked, ColMajor>;

pub type Mat2RP<T> = Matrix<2, 2, T, VecPacked, RowMajor>;

// ...
```

Examples:

```rust
// In here, i feel like using column-major order.
struct MyTransform {
    transformation_matrix: Mat4C<f32>,
}

// In here, i feel like using row-major order.
struct MyCamera {
    view_matrix: Mat4R<f32>,
    projection_matrix: Mat4R<f32>,
}
```

Matrices are not feature full yet.
They do support swizzling and construction from columns/rows.

### Aabbs (Bounding Boxes) (opt-in feature)

```rust
// Aabb's declaration
pub struct Aabb<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr>
where
    Usize<N>: VecLen,

// `R` is a generic marker type that affects how the aabb is stored in memory.
// This is because an aabb could be stored in multiple ways:
// - By their minimum corner and their size
// - By their center and their size
// - By their minimum and maximum corners
// - Any combination of these things.
//
// `R` can be set to either `AabbCornered` or `AabbCentered` or `AabbMinMaxed`.
//
// `AabbCornered` aabbs are stored by their minimum corner and their size.
// `AabbCentered` aabbs are stored by their center and their extents.
// `AabbMinMaxed` aabbs are stored by their minimum and maximum corners.

// 2D Aliases
pub type Rect<T> = Aabb<2, T, VecAligned, AabbCornered>;
pub type RectP<T> = Aabb<2, T, VecPacked, AabbCornered>;

pub type RectC<T> = Aabb<2, T, VecAligned, AabbCentered>;

pub type RectM<T> = Aabb<2, T, VecAligned, AabbMinMaxed>;

// 3D Aliases
pub type Aabb3<T> = Aabb<3, T, VecAligned, AabbCornered>;

pub type Aabb3M<T> = Aabb<3, T, VecAligned, AabbMinMaxed>;

// 4D Aliases
pub type Aabb4<T> = Aabb<4, T, VecAligned, AabbCornered>;

// ...
```

Examples:

```rust
// Ui rectangles are usually stored by their minimum corner and their size.
struct MyUi {
    // `AabbCornered`
    area: Rect<f32>,
}

// Collision boxes are usually stored by their center and their extents.
struct MyCollider {
    // `AabbCentered`
    aabb: RectC<f32>,
}
```

Aabbs support all expected features:
- Construction: `Aabb::from_min_size`, `Aabb::from_min_max`, `Aabb::from_center_size` etc.
- Accessors: `aabb.min()`, `aabb.max()`, `aabb.center()`, `aabb.extents()` etc.
- Logic: `aabb.contains(point)`, `aabb.intersects(aabb)` etc.
- Swizzling: `aabb.xy()`, `aabb.xyz()` etc.
- Nice type aliases: `FAabb3`, `IRectMP`, etc.

### Quaternions

```rust
// Quaternion's declaration
pub struct Quaternion<T: Scalar, A: VecAlignment>

pub type Quat<T> = Quaternion<T, VecAligned>;
pub type QuatP<T> = Quaternion<T, VecPacked>;
```

Quaternions are not feature full yet.

### Features

default features:
- `vector`: vector type.
- `matrix`: matrix type.
- `quaternion`: quaternion type.
- `primitive_aliases`: primitive type aliases like `FVec3`.

"full" features:
- `aabb`: aabb type.

optional features:
- `right`: `RIGHT` and `LEFT` constants where right is positive.
- `left`: `LEFT` and `RIGHT` constants where left is positive.
- `up`: `UP` and `DOWN` constants where up is positive.
- `down`: `DOWN` and `UP` constants where down is positive.
- `forward`: `FORWARD` and `BACKWARD` constants where forward is positive.
- `backward`: `BACKWARD` and `FORWARD` constants where backward is positive.
- `serde`: enables support for the `serde` crate.
- `crevice`: enables support for the `crevice` crate.
