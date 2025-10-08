use crate::{ElementOfVector, Simd, Vector};

// IVec2 is only 64 bits, so it doesn't benefit from SIMD.
unsafe impl ElementOfVector<2, Simd> for i32 {
    type InnerVectorType = [i32; 2];

    const VECTOR_PADDING: Option<Vector<2, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to IVec3.
unsafe impl ElementOfVector<3, Simd> for i32 {
    type InnerVectorType = [i32; 3];

    const VECTOR_PADDING: Option<Vector<3, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to IVec4.
unsafe impl ElementOfVector<4, Simd> for i32 {
    type InnerVectorType = [i32; 4];

    const VECTOR_PADDING: Option<Vector<4, Self, Simd>> = None;
}

// TODO: Determine if IVec5 benefits from SIMD.
unsafe impl ElementOfVector<5, Simd> for i32 {
    type InnerVectorType = [i32; 5];

    const VECTOR_PADDING: Option<Vector<5, Self, Simd>> = None;
}

// TODO: Determine if IVec6 benefits from SIMD.
unsafe impl ElementOfVector<6, Simd> for i32 {
    type InnerVectorType = [i32; 6];

    const VECTOR_PADDING: Option<Vector<6, Self, Simd>> = None;
}

// TODO: Determine if IVec7 benefits from SIMD.
unsafe impl ElementOfVector<7, Simd> for i32 {
    type InnerVectorType = [i32; 7];

    const VECTOR_PADDING: Option<Vector<7, Self, Simd>> = None;
}

// TODO: Determine if IVec8 benefits from SIMD.
unsafe impl ElementOfVector<8, Simd> for i32 {
    type InnerVectorType = [i32; 8];

    const VECTOR_PADDING: Option<Vector<8, Self, Simd>> = None;
}
