use crate::{ElementOfVector, Simd, Vector};

// TODO: Add SIMD optimizations to I64Vec2.
unsafe impl ElementOfVector<2, Simd> for i64 {
    type InnerVectorType = [i64; 2];

    const VECTOR_PADDING: Option<Vector<2, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to I64Vec3.
unsafe impl ElementOfVector<3, Simd> for i64 {
    type InnerVectorType = [i64; 3];

    const VECTOR_PADDING: Option<Vector<3, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to I64Vec4.
unsafe impl ElementOfVector<4, Simd> for i64 {
    type InnerVectorType = [i64; 4];

    const VECTOR_PADDING: Option<Vector<4, Self, Simd>> = None;
}

// TODO: Determine if I64Vec5 benefits from SIMD.
unsafe impl ElementOfVector<5, Simd> for i64 {
    type InnerVectorType = [i64; 5];

    const VECTOR_PADDING: Option<Vector<5, Self, Simd>> = None;
}

// TODO: Determine if I64Vec6 benefits from SIMD.
unsafe impl ElementOfVector<6, Simd> for i64 {
    type InnerVectorType = [i64; 6];

    const VECTOR_PADDING: Option<Vector<6, Self, Simd>> = None;
}

// TODO: Determine if I64Vec7 benefits from SIMD.
unsafe impl ElementOfVector<7, Simd> for i64 {
    type InnerVectorType = [i64; 7];

    const VECTOR_PADDING: Option<Vector<7, Self, Simd>> = None;
}

// TODO: Determine if I64Vec8 benefits from SIMD.
unsafe impl ElementOfVector<8, Simd> for i64 {
    type InnerVectorType = [i64; 8];

    const VECTOR_PADDING: Option<Vector<8, Self, Simd>> = None;
}
