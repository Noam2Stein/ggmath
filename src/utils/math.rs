#[cfg(feature = "libm")]
#[allow(unused_imports, reason = "rustc incorrectly marks this as unused")]
use crate::utils::PrimitiveFloatUtils;

pub trait FloatUtils {
    /// Approximate arc cosine.
    ///
    /// Instead of returning NaN for values outside the range -1 to 1, this
    /// clamps them into the range.
    fn acos_approx(self) -> Self;
}

impl FloatUtils for f32 {
    fn acos_approx(self) -> Self {
        // Ported from `https://docs.rs/glam/0.33.1/src/glam/f32/math.rs.html`.
        // Based on `https://github.com/microsoft/DirectXMath` `XMScalarAcos`.

        // Clamp input to -1..=1.
        let non_negative = self >= 0.0;
        let x = self.abs();
        let mut omx = 1.0 - x;
        if omx < 0.0 {
            omx = 0.0;
        }
        let root = omx.sqrt();

        // 7-degree minimax approximation
        #[allow(clippy::approx_constant)]
        let mut result = ((((((-0.001_262_491_1 * x + 0.006_670_09) * x - 0.017_088_126) * x
            + 0.030_891_88)
            * x
            - 0.050_174_303)
            * x
            + 0.088_978_99)
            * x
            - 0.214_598_8)
            * x
            + 1.570_796_3;
        result *= root;

        // acos(x) = pi - acos(-x) when x < 0
        if non_negative {
            result
        } else {
            core::f32::consts::PI - result
        }
    }
}

impl FloatUtils for f64 {
    #[expect(clippy::excessive_precision)]
    #[expect(clippy::approx_constant)]
    #[expect(clippy::needless_late_init)]
    fn acos_approx(self) -> Self {
        // Based on `https://docs.rs/libm/latest/src/libm/math/acos.rs.html`.

        const PIO2_HI: f64 = 1.57079632679489655800e+00; /* 0x3FF921FB, 0x54442D18 */
        const PIO2_LO: f64 = 6.12323399573676603587e-17; /* 0x3C91A626, 0x33145C07 */
        const PS0: f64 = 1.66666666666666657415e-01; /* 0x3FC55555, 0x55555555 */
        const PS1: f64 = -3.25565818622400915405e-01; /* 0xBFD4D612, 0x03EB6F7D */
        const PS2: f64 = 2.01212532134862925881e-01; /* 0x3FC9C155, 0x0E884455 */
        const PS3: f64 = -4.00555345006794114027e-02; /* 0xBFA48228, 0xB5688F3B */
        const PS4: f64 = 7.91534994289814532176e-04; /* 0x3F49EFE0, 0x7501B288 */
        const PS5: f64 = 3.47933107596021167570e-05; /* 0x3F023DE1, 0x0DFDF709 */
        const QS1: f64 = -2.40339491173441421878e+00; /* 0xC0033A27, 0x1C8A2D4B */
        const QS2: f64 = 2.02094576023350569471e+00; /* 0x40002AE5, 0x9C598AC8 */
        const QS3: f64 = -6.88283971605453293030e-01; /* 0xBFE6066C, 0x1B8D0159 */
        const QS4: f64 = 7.70381505559019352791e-02; /* 0x3FB3B8C5, 0xB12E9282 */

        fn r(z: f64) -> f64 {
            let p: f64 = z * (PS0 + z * (PS1 + z * (PS2 + z * (PS3 + z * (PS4 + z * PS5)))));
            let q: f64 = 1.0 + z * (QS1 + z * (QS2 + z * (QS3 + z * QS4)));
            p / q
        }

        let x1p_120f = f64::from_bits(0x3870000000000000); // 0x1p-120 === 2 ^ -120
        let z: f64;
        let w: f64;
        let s: f64;
        let c: f64;
        let df: f64;
        let hx: u32;
        let ix: u32;

        hx = (self.to_bits() >> 32) as u32;
        ix = hx & 0x7fffffff;

        let non_finite = !self.is_finite();
        if self.abs() >= 1.0 || non_finite {
            return if non_finite {
                f64::NAN
            } else if self.is_sign_negative() {
                2. * PIO2_HI + x1p_120f
            } else {
                0.
            };
        }

        /* |x| < 0.5 */
        if ix < 0x3fe00000 {
            if ix <= 0x3c600000 {
                /* |x| < 2**-57 */
                return PIO2_HI + x1p_120f;
            }
            return PIO2_HI - (self - (PIO2_LO - self * r(self * self)));
        }
        /* x < -0.5 */
        if (hx >> 31) != 0 {
            z = (1.0 + self) * 0.5;
            s = z.sqrt();
            w = r(z) * s - PIO2_LO;
            return 2. * (PIO2_HI - (s + w));
        }
        /* x > 0.5 */
        z = (1.0 - self) * 0.5;
        s = z.sqrt();
        // Set the low 4 bytes to zero
        df = f64::from_bits(s.to_bits() & 0xff_ff_ff_ff_00_00_00_00);

        c = (z - df * df) / (s + df);
        w = r(z) * s + c;
        2. * (df + w)
    }
}

#[cfg(feature = "wide")]
mod wide {
    use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

    use crate::utils::FloatUtils;

    macro_rules! wide_f32_impl {
        ($Wide:ident) => {
            impl FloatUtils for $Wide {
                fn acos_approx(self) -> Self {
                    // Ported from `https://docs.rs/glam/0.33.1/src/glam/f32/math.rs.html`.
                    // Based on `https://github.com/microsoft/DirectXMath` `XMScalarAcos`.

                    // Clamp input to -1..=1.
                    let non_negative = self.simd_ge(0.0);
                    let x = self.abs();
                    let mut omx = 1.0 - x;
                    omx &= omx.simd_ge(0.0);

                    let root = omx.sqrt();

                    // 7-degree minimax approximation
                    #[allow(clippy::approx_constant)]
                    let mut result =
                        ((((((-0.001_262_491_1 * x + 0.006_670_09) * x - 0.017_088_126) * x
                            + 0.030_891_88)
                            * x
                            - 0.050_174_303)
                            * x
                            + 0.088_978_99)
                            * x
                            - 0.214_598_8)
                            * x
                            + 1.570_796_3;
                    result *= root;

                    // acos(x) = pi - acos(-x) when x < 0
                    non_negative.select(result, core::f32::consts::PI - result)
                }
            }
        };
    }
    wide_f32_impl!(f32x4);
    wide_f32_impl!(f32x8);
    wide_f32_impl!(f32x16);

    macro_rules! wide_f64_impl {
        ($Wide:ident) => {
            impl FloatUtils for $Wide {
                #[expect(clippy::excessive_precision)]
                fn acos_approx(self) -> Self {
                    // Based on `https://github.com/Lokathor/wide/blob/main/src/f64x2_.rs`.
                    // Based on the Agner Fog "vector class library":
                    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h.

                    const R4_ASIN: $Wide = $Wide::splat(2.967721961301243206100E-3);
                    const R3_ASIN: $Wide = $Wide::splat(-5.634242780008963776856E-1);
                    const R2_ASIN: $Wide = $Wide::splat(6.968710824104713396794E0);
                    const R1_ASIN: $Wide = $Wide::splat(-2.556901049652824852289E1);
                    const R0_ASIN: $Wide = $Wide::splat(2.853665548261061424989E1);

                    const S3_ASIN: $Wide = $Wide::splat(-2.194779531642920639778E1);
                    const S2_ASIN: $Wide = $Wide::splat(1.470656354026814941758E2);
                    const S1_ASIN: $Wide = $Wide::splat(-3.838770957603691357202E2);
                    const S0_ASIN: $Wide = $Wide::splat(3.424398657913078477438E2);

                    const P5_ASIN: $Wide = $Wide::splat(4.253011369004428248960E-3);
                    const P4_ASIN: $Wide = $Wide::splat(-6.019598008014123785661E-1);
                    const P3_ASIN: $Wide = $Wide::splat(5.444622390564711410273E0);
                    const P2_ASIN: $Wide = $Wide::splat(-1.626247967210700244449E1);
                    const P1_ASIN: $Wide = $Wide::splat(1.956261983317594739197E1);
                    const P0_ASIN: $Wide = $Wide::splat(-8.198089802484824371615E0);

                    const Q4_ASIN: $Wide = $Wide::splat(-1.474091372988853791896E1);
                    const Q3_ASIN: $Wide = $Wide::splat(7.049610280856842141659E1);
                    const Q2_ASIN: $Wide = $Wide::splat(-1.471791292232726029859E2);
                    const Q1_ASIN: $Wide = $Wide::splat(1.395105614657485689735E2);
                    const Q0_ASIN: $Wide = $Wide::splat(-4.918853881490881290097E1);

                    let xa = self.abs();
                    let xa = xa.simd_ge(Self::ONE).select(Self::ONE, xa);

                    let big = xa.simd_ge($Wide::splat(0.625));

                    let x1 = big.select($Wide::ONE - xa, xa * xa);

                    let x2 = x1 * x1;
                    let x3 = x2 * x1;
                    let x4 = x2 * x2;
                    let x5 = x4 * x1;

                    let do_big = big.any();
                    let do_small = !big.all();

                    let mut rx = $Wide::ZERO;
                    let mut sx = $Wide::ZERO;
                    let mut px = $Wide::ZERO;
                    let mut qx = $Wide::ZERO;

                    if do_big {
                        rx = x3.mul_add(R3_ASIN, x2 * R2_ASIN)
                            + x4.mul_add(R4_ASIN, x1.mul_add(R1_ASIN, R0_ASIN));
                        sx = x3.mul_add(S3_ASIN, x4)
                            + x2.mul_add(S2_ASIN, x1.mul_add(S1_ASIN, S0_ASIN));
                    }
                    if do_small {
                        px = x3.mul_add(P3_ASIN, P0_ASIN)
                            + x4.mul_add(P4_ASIN, x1 * P1_ASIN)
                            + x5.mul_add(P5_ASIN, x2 * P2_ASIN);
                        qx = x4.mul_add(Q4_ASIN, x5)
                            + x3.mul_add(Q3_ASIN, x1 * Q1_ASIN)
                            + x2.mul_add(Q2_ASIN, Q0_ASIN);
                    };

                    let vx = big.select(rx, px);
                    let wx = big.select(sx, qx);

                    let y1 = vx / wx * x1;

                    let mut z1 = $Wide::ZERO;
                    let mut z2 = $Wide::ZERO;
                    if do_big {
                        let xb = (x1 + x1).sqrt();
                        z1 = xb.mul_add(y1, xb);
                    }

                    if do_small {
                        z2 = xa.mul_add(y1, xa);
                    }

                    // acos
                    let z3 = self.simd_lt($Wide::ZERO).select($Wide::PI - z1, z1);
                    let z4 = $Wide::FRAC_PI_2 - z2.flip_signs(self);
                    let acos = big.select(z3, z4);

                    acos
                }
            }
        };
    }
    wide_f64_impl!(f64x2);
    wide_f64_impl!(f64x4);
    wide_f64_impl!(f64x8);

    #[cfg(test)]
    mod tests {
        use crate::{
            test_utils::{assert_test_eq, for_types, random_iter},
            utils::FloatUtils,
        };

        #[test]
        fn test_acos_approx() {
            for_types!(|Wide: WideFloat| {
                for x in random_iter::<Wide>() {
                    for lane in 0..LANES {
                        if !x.to_array()[lane].is_finite() {
                            assert!(x.to_array()[lane].acos_approx().is_nan());
                        } else if (-1.0..=1.0).contains(&x.to_array()[lane]) {
                            assert_test_eq!(
                                x.to_array()[lane].acos_approx(),
                                x.to_array()[lane].acos(),
                                abs <= x.to_array()[lane].acos() * 1e-5
                            );
                        } else if (-10.0..=10.0).contains(&x.to_array()[lane]) {
                            assert!(x.to_array()[lane].acos_approx().is_finite());
                        }
                    }
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_utils::{assert_test_eq, for_types, random_iter},
        utils::FloatUtils,
    };

    #[test]
    fn test_acos_approx() {
        for_types!(|T: PrimitiveFloat| {
            for x in random_iter::<T>() {
                if !x.is_finite() {
                    assert!(x.acos_approx().is_nan());
                } else if (-1.0..=1.0).contains(&x) {
                    assert_test_eq!(x.acos_approx(), x.acos(), abs <= x.acos() * 1e-5);
                } else if (-10.0..=10.0).contains(&x) {
                    assert!(x.acos_approx().is_finite());
                }
            }
        });
    }
}
