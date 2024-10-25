use ggmath::scalar::aliases::f32::*;

fn main() {
    let vec = FVec2::from_array([1.0, 2.0]);
    println!(
        "size: {}, align: {}",
        align_of::<FVec3>(), // Vector<3, f32, VecAligned>
        size_of::<FVec3>()
    );
    // size: 16, align: 16

    println!(
        "size: {}, align: {}",
        align_of::<FVec3P>(), // Vector<3, f32, VecPacked>
        size_of::<FVec3P>()
    );
    // size: 4, align: 12
}
