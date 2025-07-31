use ggmath::IVec4;

fn main() {
    println!("{:?}", div as fn(IVec4, IVec4) -> IVec4);
}

#[inline(never)]
fn div(a: IVec4, b: IVec4) -> IVec4 {
    a / b
}
