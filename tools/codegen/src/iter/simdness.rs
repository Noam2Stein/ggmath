use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Simdness {
    Simd,
    NonSimd,
}

#[expect(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimdBackend {
    X86,
    Arm,
    Wasm,
    Scalar,
}

impl Simdness {
    pub fn postfix_lowercase(&self) -> &'static str {
        match self {
            Simdness::Simd => "",
            Simdness::NonSimd => "s",
        }
    }

    pub fn postfix_uppercase(&self) -> &'static str {
        match self {
            Simdness::Simd => "",
            Simdness::NonSimd => "S",
        }
    }
}
