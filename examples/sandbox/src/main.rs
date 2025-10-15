use ggmath::{f32::*, right::PositiveRightExt, *};

fn main() {
    let f = 1.0f32;
    let v = vec2!(f, 2.0);
    let v = vec4!(v.xyy().with_z(3.0), 4.0);
    let v = v * 1.5;

    println!("{}", v.ceil().recip() + FVec4::RIGHT);
}
