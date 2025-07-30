use ggmath::*;

fn main() {
    let a = vec3!(1.0, 2.0, 3.0);
    let b = vec3!(4.0, 5.0, 6.0);
    let c = vec4!(7.0, 8.0, 9.0, 10.0);
    let d = vec4!(11.0, 12.0, 13.0, 14.0);
    println!("{:?}", add3(a, b));
    println!("{:?}", add4(c, d));
}

#[inline(never)]
fn add3(a: FVec3, b: FVec3) -> FVec3 {
    a + b
}

#[inline(never)]
fn add4(a: FVec4, b: FVec4) -> FVec4 {
    a + b
}
