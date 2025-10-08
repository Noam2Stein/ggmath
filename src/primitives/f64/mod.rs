use crate::{ElementOfVector, Simd, Vector};

// TODO: Add SIMD optimizations to DVec2.
unsafe impl ElementOfVector<2, Simd> for f64 {
    type InnerVectorType = [f64; 2];

    const VECTOR_PADDING: Option<Vector<2, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to DVec3.
unsafe impl ElementOfVector<3, Simd> for f64 {
    type InnerVectorType = [f64; 3];

    const VECTOR_PADDING: Option<Vector<3, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to DVec4.
unsafe impl ElementOfVector<4, Simd> for f64 {
    type InnerVectorType = [f64; 4];

    const VECTOR_PADDING: Option<Vector<4, Self, Simd>> = None;
}

// TODO: Determine if DVec5 benefits from SIMD.
unsafe impl ElementOfVector<5, Simd> for f64 {
    type InnerVectorType = [f64; 5];

    const VECTOR_PADDING: Option<Vector<5, Self, Simd>> = None;
}

// TODO: Determine if DVec6 benefits from SIMD.
unsafe impl ElementOfVector<6, Simd> for f64 {
    type InnerVectorType = [f64; 6];

    const VECTOR_PADDING: Option<Vector<6, Self, Simd>> = None;
}

// TODO: Determine if DVec7 benefits from SIMD.
unsafe impl ElementOfVector<7, Simd> for f64 {
    type InnerVectorType = [f64; 7];

    const VECTOR_PADDING: Option<Vector<7, Self, Simd>> = None;
}

// TODO: Determine if DVec8 benefits from SIMD.
unsafe impl ElementOfVector<8, Simd> for f64 {
    type InnerVectorType = [f64; 8];

    const VECTOR_PADDING: Option<Vector<8, Self, Simd>> = None;
}
