fn main() {
    println!("cargo::rustc-check-cfg=cfg(backend)");
    if cfg!(feature = "std") {
        println!("cargo::rustc-cfg=backend");
    }

    println!("cargo::rustc-check-cfg=cfg(assertions)");
    if cfg!(feature = "assertions") || cfg!(debug_assertions) && !cfg!(feature = "no-assertions") {
        println!("cargo::rustc-cfg=assertions");
    }
}
