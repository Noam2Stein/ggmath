use ggmath::{NonSimd, Simd, Vector, f32_aliases::FVec3, vec3, vec4};

fn main() {
    // Vector construction
    let a: Vector<3, f32, Simd> = FVec3::from_array([1.0, 2.0, 3.0]);
    let b: Vector<3, f32, Simd> = vec3!(1.0, 2.0, 3.0);
    let c: Vector<3, f32, Simd> = vec3!(4.0, vec4!(5.0, 6.0, 7.0, 8.0).xy());

    // Simple math operations
    let d: Vector<3, f32, NonSimd> = (a + b * c).as_nonsimd();

    println!("{:?}", d);
}
