use crate::{ElementOfVector, Simd, Vector};

// TODO: Add SIMD optimizations to U64Vec2.
unsafe impl ElementOfVector<2, Simd> for u64 {
    type InnerVectorType = [u64; 2];

    const VECTOR_PADDING: Option<Vector<2, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to U64Vec3.
unsafe impl ElementOfVector<3, Simd> for u64 {
    type InnerVectorType = [u64; 3];

    const VECTOR_PADDING: Option<Vector<3, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to U64Vec4.
unsafe impl ElementOfVector<4, Simd> for u64 {
    type InnerVectorType = [u64; 4];

    const VECTOR_PADDING: Option<Vector<4, Self, Simd>> = None;
}

// TODO: Determine if U64Vec5 benefits from SIMD.
unsafe impl ElementOfVector<5, Simd> for u64 {
    type InnerVectorType = [u64; 5];

    const VECTOR_PADDING: Option<Vector<5, Self, Simd>> = None;
}

// TODO: Determine if U64Vec6 benefits from SIMD.
unsafe impl ElementOfVector<6, Simd> for u64 {
    type InnerVectorType = [u64; 6];

    const VECTOR_PADDING: Option<Vector<6, Self, Simd>> = None;
}

// TODO: Determine if U64Vec7 benefits from SIMD.
unsafe impl ElementOfVector<7, Simd> for u64 {
    type InnerVectorType = [u64; 7];

    const VECTOR_PADDING: Option<Vector<7, Self, Simd>> = None;
}

// TODO: Determine if U64Vec8 benefits from SIMD.
unsafe impl ElementOfVector<8, Simd> for u64 {
    type InnerVectorType = [u64; 8];

    const VECTOR_PADDING: Option<Vector<8, Self, Simd>> = None;
}
