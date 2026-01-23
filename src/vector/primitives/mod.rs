pub(crate) mod f32 {
    type T = f32;

    include!("float.rs");
}

pub(crate) mod f64 {
    type T = f64;

    include!("float.rs");
}

mod bool;
