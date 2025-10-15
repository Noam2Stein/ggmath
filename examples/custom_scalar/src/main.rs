use std::mem::transmute_copy;

use ggmath::*;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct CustomScalar(f32);

declare_vector_aliases!(type C => CustomScalar);

impl Scalar for CustomScalar {}

impl<const N: usize> SimdBehaviour<N> for CustomScalar
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = Vector<N, f32, Simd>;

    #[inline(always)]
    fn vec_from_array(array: [Self; N]) -> Vector<N, Self, Simd> {
        // SAFETY: CustomScalar is transparent to f32
        let f32_array = unsafe { transmute_copy::<[Self; N], [f32; N]>(&array) };

        Vector::from_repr(Vector::from_array(f32_array))
    }
}

unsafe impl ScalarWrapper<f32> for CustomScalar {}

fn main() {
    let v2 = vec2!(CustomScalar(2.0), CustomScalar(3.0));
    let v = vec4!(CustomScalar(1.0), v2, CustomScalar(4.0));

    println!("{:?}", v.yz());
}
