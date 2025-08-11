use ggmath::*;

fn main() {
    let a: fn(&FVec2) -> FVec2 = test;

    println!("{a:?}");
    println!("{}", align_of::<FVec2>());
}

#[inline(never)]
fn test(a: &FVec2) -> FVec2 {
    a.yx()
}
