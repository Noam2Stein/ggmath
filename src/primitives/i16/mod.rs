use crate::{ElementOfVector, Simd, Vector};

// I16Vec2 is only 32 bits, so it doesn't benefit from SIMD.
unsafe impl ElementOfVector<2, Simd> for i16 {
    type InnerVectorType = [i16; 2];

    const VECTOR_PADDING: Option<Vector<2, Self, Simd>> = None;
}

// I16Vec3 is only 48 bits, so it doesn't benefit from SIMD.
unsafe impl ElementOfVector<3, Simd> for i16 {
    type InnerVectorType = [i16; 3];

    const VECTOR_PADDING: Option<Vector<3, Self, Simd>> = None;
}

// I16Vec4 is only 64 bits, so it doesn't benefit from SIMD.
unsafe impl ElementOfVector<4, Simd> for i16 {
    type InnerVectorType = [i16; 4];

    const VECTOR_PADDING: Option<Vector<4, Self, Simd>> = None;
}

// TODO: Determine if I16Vec5 benefits from SIMD.
unsafe impl ElementOfVector<5, Simd> for i16 {
    type InnerVectorType = [i16; 5];

    const VECTOR_PADDING: Option<Vector<5, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to I16Vec6.
unsafe impl ElementOfVector<6, Simd> for i16 {
    type InnerVectorType = [i16; 6];

    const VECTOR_PADDING: Option<Vector<6, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to I16Vec7.
unsafe impl ElementOfVector<7, Simd> for i16 {
    type InnerVectorType = [i16; 7];

    const VECTOR_PADDING: Option<Vector<7, Self, Simd>> = None;
}

// TODO: Add SIMD optimizations to I16Vec8.
unsafe impl ElementOfVector<8, Simd> for i16 {
    type InnerVectorType = [i16; 8];

    const VECTOR_PADDING: Option<Vector<8, Self, Simd>> = None;
}
