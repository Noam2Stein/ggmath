pub(in crate::vector) mod f32 {
    type T = f32;

    include!("float.rs");
}

pub(in crate::vector) mod f64 {
    type T = f64;

    include!("float.rs");
}

pub(in crate::vector) mod i8 {
    type T = i8;

    include!("int.rs");
}

pub(in crate::vector) mod i16 {
    type T = i16;

    include!("int.rs");
}

pub(in crate::vector) mod i32 {
    type T = i32;

    include!("int.rs");
}

pub(in crate::vector) mod i64 {
    type T = i64;

    include!("int.rs");
}

pub(in crate::vector) mod i128 {
    type T = i128;

    include!("int.rs");
}

pub(in crate::vector) mod isize {
    type T = isize;

    include!("int.rs");
}

pub(in crate::vector) mod u8 {
    type T = u8;

    include!("uint.rs");
}

pub(in crate::vector) mod u16 {
    type T = u16;

    include!("uint.rs");
}

pub(in crate::vector) mod u32 {
    type T = u32;

    include!("uint.rs");
}

pub(in crate::vector) mod u64 {
    type T = u64;

    include!("uint.rs");
}

pub(in crate::vector) mod u128 {
    type T = u128;

    include!("uint.rs");
}

pub(in crate::vector) mod usize {
    type T = usize;

    include!("uint.rs");
}

pub(in crate::vector) mod bool;
