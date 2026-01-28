pub(crate) mod f32 {
    type T = f32;

    include!("float.rs");
}

pub(crate) mod f64 {
    type T = f64;

    include!("float.rs");
}

pub(crate) mod i8 {
    type T = i8;

    include!("int.rs");
}

pub(crate) mod i16 {
    type T = i16;

    include!("int.rs");
}

pub(crate) mod i32 {
    type T = i32;

    include!("int.rs");
}

pub(crate) mod i64 {
    type T = i64;

    include!("int.rs");
}

pub(crate) mod i128 {
    type T = i128;

    include!("int.rs");
}

pub(crate) mod isize {
    type T = isize;

    include!("int.rs");
}

pub(crate) mod u8 {
    type T = u8;

    include!("uint.rs");
}

pub(crate) mod u16 {
    type T = u16;

    include!("uint.rs");
}

pub(crate) mod u32 {
    type T = u32;

    include!("uint.rs");
}

pub(crate) mod u64 {
    type T = u64;

    include!("uint.rs");
}

pub(crate) mod u128 {
    type T = u128;

    include!("uint.rs");
}

pub(crate) mod usize {
    type T = usize;

    include!("uint.rs");
}

mod bool;
