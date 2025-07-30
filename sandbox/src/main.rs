use ggmath::*;

fn main() {
    println!("{:?}", add4 as fn(FVec4, FVec4) -> FVec4);
    //println!("{:?}", add3 as fn(FVec3, FVec3) -> FVec3);
}

#[inline(never)]
fn add4(a: FVec4, b: FVec4) -> FVec4 {
    a + b
}

#[inline(never)]
fn add3(a: FVec3, b: FVec3) -> FVec3 {
    a + b
}
