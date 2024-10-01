pub unsafe trait ElementInnerVecs {
    type InnerVec2: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec3: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec4: std::fmt::Debug + Copy + PartialEq + PartialOrd;
}
