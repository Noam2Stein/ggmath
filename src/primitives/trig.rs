use super::*;

repetitive! {
    @for prim in ['f32, 'f64] {
        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @prim, A>
        where
            Usize<N>: VecLen,
        {
            /// Vector of the sine values of the input elements.
            #[inline(always)]
            pub fn sin(self) -> Self {
                self.map(@prim::sin)
            }
            /// Vector of the cosine values of the input elements.
            #[inline(always)]
            pub fn cos(self) -> Self {
                self.map(@prim::cos)
            }
            /// Vector of the tangent values of the input elements.
            #[inline(always)]
            pub fn tan(self) -> Self {
                self.map(@prim::tan)
            }

            /// Vector of the arcsine values of the input elements.
            #[inline(always)]
            pub fn asin(self) -> Self {
                self.map(@prim::asin)
            }
            /// Vector of the arccosine values of the input elements.
            #[inline(always)]
            pub fn acos(self) -> Self {
                self.map(@prim::acos)
            }
            /// Vector of the arctangent values of the input elements.
            #[inline(always)]
            pub fn atan(self) -> Self {
                self.map(@prim::atan)
            }

            /// Vector of the hyperbolic sine values of the input elements.
            #[inline(always)]
            pub fn sinh(self) -> Self {
                self.map(@prim::sinh)
            }
            /// Vector of the hyperbolic cosine values of the input elements.
            #[inline(always)]
            pub fn cosh(self) -> Self {
                self.map(@prim::cosh)
            }
            /// Vector of the hyperbolic tangent values of the input elements.
            #[inline(always)]
            pub fn tanh(self) -> Self {
                self.map(@prim::tanh)
            }

            /// Vector of the hyperbolic arcsine values of the input elements.
            #[inline(always)]
            pub fn asinh(self) -> Self {
                self.map(@prim::asinh)
            }
            /// Vector of the hyperbolic arccosine values of the input elements.
            #[inline(always)]
            pub fn acosh(self) -> Self {
                self.map(@prim::acosh)
            }
            /// Vector of the hyperbolic arctangent values of the input elements.
            #[inline(always)]
            pub fn atanh(self) -> Self {
                self.map(@prim::atanh)
            }
        }
    }
}
