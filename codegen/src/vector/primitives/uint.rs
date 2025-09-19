use std::collections::HashMap;

use genco::lang::rust::Tokens;

pub fn push_src(
    _primitive: &str,
    _use_crate_items: &mut Vec<Tokens>,
    _functions: &mut Vec<Tokens>,
    _len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _std_functions: &mut Vec<Tokens>,
    _std_len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _trait_impls: &mut Vec<Tokens>,
) {
}

pub fn push_test(_primitive: &str, _use_stmts: &mut Vec<Tokens>, _functions: &mut Vec<Tokens>) {}
