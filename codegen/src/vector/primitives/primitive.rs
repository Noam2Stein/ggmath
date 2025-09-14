pub fn push_fns(
    _primitive: &str,
    _functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _trait_impls: &mut Vec<String>,
    use_crate_items: &mut Vec<String>,
) {
    use_crate_items.push("Vector, VecAlignment, Usize, VecLen".to_string());
}
