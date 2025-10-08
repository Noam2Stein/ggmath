use crate::{ElementOfVector, Simd, Vector};

// FVec2 is only 64 bits, so it doesn't benefit from SIMD.
unsafe impl ElementOfVector<2, Simd> for f32 {
    type InnerVectorType = [f32; 2];

    const VECTOR_PADDING: Option<Vector<2, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to FVec3.
unsafe impl ElementOfVector<3, Simd> for f32 {
    type InnerVectorType = [f32; 3];

    const VECTOR_PADDING: Option<Vector<3, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to FVec4.
unsafe impl ElementOfVector<4, Simd> for f32 {
    type InnerVectorType = [f32; 4];

    const VECTOR_PADDING: Option<Vector<4, Self, Simd>> = None;
}

// TODO: Determine if FVec5 benefits from SIMD.
unsafe impl ElementOfVector<5, Simd> for f32 {
    type InnerVectorType = [f32; 5];

    const VECTOR_PADDING: Option<Vector<5, Self, Simd>> = None;
}

// TODO: Determine if FVec6 benefits from SIMD.
unsafe impl ElementOfVector<6, Simd> for f32 {
    type InnerVectorType = [f32; 6];

    const VECTOR_PADDING: Option<Vector<6, Self, Simd>> = None;
}

// TODO: Determine if FVec7 benefits from SIMD.
unsafe impl ElementOfVector<7, Simd> for f32 {
    type InnerVectorType = [f32; 7];

    const VECTOR_PADDING: Option<Vector<7, Self, Simd>> = None;
}

// TODO: Determine if FVec8 benefits from SIMD.
unsafe impl ElementOfVector<8, Simd> for f32 {
    type InnerVectorType = [f32; 8];

    const VECTOR_PADDING: Option<Vector<8, Self, Simd>> = None;
}
