use crate::{ElementOfVector, Simd, Vector};

// UVec2 is only 64 bits, so it doesn't benefit from SIMD.
unsafe impl ElementOfVector<2, Simd> for u32 {
    type InnerVectorType = [u32; 2];

    const VECTOR_PADDING: Option<Vector<2, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to UVec3.
unsafe impl ElementOfVector<3, Simd> for u32 {
    type InnerVectorType = [u32; 3];

    const VECTOR_PADDING: Option<Vector<3, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to UVec4.
unsafe impl ElementOfVector<4, Simd> for u32 {
    type InnerVectorType = [u32; 4];

    const VECTOR_PADDING: Option<Vector<4, Self, Simd>> = None;
}

// TODO: Determine if UVec5 benefits from SIMD.
unsafe impl ElementOfVector<5, Simd> for u32 {
    type InnerVectorType = [u32; 5];

    const VECTOR_PADDING: Option<Vector<5, Self, Simd>> = None;
}

// TODO: Determine if UVec6 benefits from SIMD.
unsafe impl ElementOfVector<6, Simd> for u32 {
    type InnerVectorType = [u32; 6];

    const VECTOR_PADDING: Option<Vector<6, Self, Simd>> = None;
}

// TODO: Determine if UVec7 benefits from SIMD.
unsafe impl ElementOfVector<7, Simd> for u32 {
    type InnerVectorType = [u32; 7];

    const VECTOR_PADDING: Option<Vector<7, Self, Simd>> = None;
}

// TODO: Determine if UVec8 benefits from SIMD.
unsafe impl ElementOfVector<8, Simd> for u32 {
    type InnerVectorType = [u32; 8];

    const VECTOR_PADDING: Option<Vector<8, Self, Simd>> = None;
}
