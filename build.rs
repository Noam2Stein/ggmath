use std::{env, fs::{create_dir_all, remove_file, write}, ops::Range, path::PathBuf};

use lazy_static::lazy_static;
use regex::Regex;

struct VecType {
    pub name: String,
    pub n: usize,
    pub is_aligned: bool,
}

const VECS: Range<usize> = 2..5;
const COMPONENTS: [char; VECS.end - 1] = ['x', 'y', 'z', 'w'];

lazy_static!(
    static ref VEC_TYPES: Box<[VecType]> = {
        let mut result = Vec::with_capacity(VECS.len() * 2);
        for n in VECS
        {
            result.push(
                VecType {
                    name: format!("Vec{n}"),
                    n,
                    is_aligned: false,
                }   
            );
            if !n.is_power_of_two() {
                result.push(
                    VecType {
                        name: format!("Vec{n}A"),
                        n,
                        is_aligned: true,
                    }   
                );
            };
        };
        result.into_boxed_slice()
    };
);

fn main() {
    let src_dir = PathBuf::from_iter([
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()),
        PathBuf::from("src/gen"),
    ].iter());

    for entry in std::fs::read_dir(&src_dir).unwrap() {
        remove_file(entry.unwrap().path()).unwrap();
    };

    let write = |path_in_src: &str, str: &str| {
        let path = PathBuf::from_iter([&src_dir, &PathBuf::from(path_in_src)].iter());
        let dir = path.parent().unwrap();
        if !dir.exists() {
            create_dir_all(dir).unwrap();
        }
        write(path, str.as_bytes()).unwrap();
    };

    write("mod.rs", &mod_rs());

    for n in VECS {
        write(&format!("vec{n}.rs"), &vec_rs(n, false));
        if !n.is_power_of_two() {
            write(&format!("vec{n}a.rs"), &vec_rs(n, true));
        }
    };
}

fn mod_rs() -> String {
    cleanup_rs(
        VECS.map(|n|
            format!(
                "\
                mod vec{n};
                pub use vec{n}::*;
                "
            ) +
            &if !n.is_power_of_two() {
                format!(
                    "
                    mod vec{n}a;
                    pub use vec{n}a::*;
                    "
                )
            }
            else {
                "".to_string()
            }
        ).collect::<Box<[String]>>().join("\n").as_str()
    )
}
fn vec_rs(n: usize, align: bool) -> String {
    let _self = if align { format!("Vec{n}A") } else { format!("Vec{n}") };
    let _self_lower = if align { format!("vec{n}a") } else { format!("vec{n}") };

    let components = (0..n).map(|i| COMPONENTS[i].to_string()).collect::<Box<[String]>>();
    let alignment_len = if align { n.next_power_of_two() - n } else { 0 };

    let fields = components.iter().map(|c| format!("pub(crate) {c}: T,")).collect::<Box<[String]>>().join("\n");
    let alignment_field = if align { format!("pub(crate) _alignment: [T; {alignment_len}],") } else { String::new() };
    
    let args = components.iter().map(|c| format!("{c}: T")).collect::<Box<[String]>>().join(", ");
    let reference_args = components.iter().map(|c| format!("{c}")).collect::<Box<[String]>>().join(", ");

    let copy_components = components.iter().map(|c| format!("{c} -> {c} * 1") ).collect::<Box<[String]>>().join(", ");
    let copy_components_splat = components.iter().map(|c| format!("value -> {c} * 1") ).collect::<Box<[String]>>().join(", ");
    
    let format_self = format!(
        "\"({})\", {}",
        components.iter().map(|_| "{}").collect::<Box<[&str]>>().join(", "),
        components.iter().map(|c| format!("self.{c}")).collect::<Box<[String]>>().join(", ")
    );

    let swizzle = VEC_TYPES.iter().map(|dst_type| {
        let dst_type_name = &dst_type.name;

        format!(
            "\
            swizzle_fns!({dst_type_name}<T>, T, [
                {}
            ]);\
            ",
            (0..n.pow(dst_type.n as u32)).map(|combination| {
                let mut combination_str = String::with_capacity(dst_type.n);
                let mut copy_instructions = String::new();
                
                let mut previous_src_component = 0;
                let mut current_copy_len = 1;
                for dst_component in 0..dst_type.n {
                    let src_component = combination / n.pow(dst_component as u32) % n;
    
                    let src_component_c = COMPONENTS[src_component];
                    let dst_component_c = COMPONENTS[dst_component];
    
                    combination_str.push(src_component_c);
                    if dst_component == 0 {
                        copy_instructions.push_str(&format!("{src_component_c} -> {dst_component_c}"));
                    }
                    else if src_component == previous_src_component + 1 {
                        current_copy_len += 1;
                    }
                    else {
                        if current_copy_len > 1 {
                            copy_instructions.push_str(&format!(" * {current_copy_len}"));
                        }
                        copy_instructions.push_str(&format!(", {src_component_c} -> {dst_component_c}"));

                        current_copy_len = 1;
                    };
    
                    previous_src_component = src_component;
                }
                if current_copy_len > 1 {
                    copy_instructions.push_str(&format!(" * {current_copy_len}"));
                }
    
                let fn_name = if dst_type.is_aligned { format!("{combination_str}_a") } else { format!("{combination_str}") };
    
                format!(
                    "({fn_name}, [{copy_instructions}]),"
                )
            }).collect::<Box<[String]>>().join(" ")
        )
    }).collect::<Box<[String]>>().join("\n");

    let set_swizzle = VEC_TYPES.iter().filter_map(|value_type| {
        if value_type.n > n {
            return None;
        };

        let value_type_name = &value_type.name;

        Some(format!(
            "\
            set_swizzle_fns!({value_type_name}<T>, T, [
                {}
            ]);\
            ",
            (0..n.pow(value_type.n as u32)).filter_map(|combination| {
                let mut combination_str = String::with_capacity(value_type.n);
                let mut copy_instructions = String::new();
                
                let mut previous_self_component = 0;
                let mut current_copy_len = 1;
                for value_component in 0..value_type.n {
                    let self_component = combination / n.pow(value_component as u32) % n;
    
                    let self_component_c = COMPONENTS[self_component];
                    let value_component_c = COMPONENTS[value_component];
    
                    if combination_str.contains(self_component_c) {
                        return None;
                    }
    
                    combination_str.push(self_component_c);
                    if value_component == 0 {
                        copy_instructions.push_str(&format!("{value_component_c} -> {self_component_c}"));
                    }
                    else if self_component == previous_self_component + 1 {
                        current_copy_len += 1;
                    }
                    else {
                        if current_copy_len > 1 {
                            copy_instructions.push_str(&format!(" * {current_copy_len}"));
                        }
                        copy_instructions.push_str(&format!(", {value_component_c} -> {self_component_c}"));

                        current_copy_len = 1;
                    };
    
                    previous_self_component = self_component;
                }
                if current_copy_len > 1 {
                    copy_instructions.push_str(&format!(" * {current_copy_len}"));
                }
    
                let fn_name = if value_type.is_aligned { format!("set_{combination_str}_a") } else { format!("set_{combination_str}") };
    
                Some(format!(
                    "({fn_name}, [{copy_instructions}]),"
                ))
            }).collect::<Box<[String]>>().join(" ")
        ))
    }).collect::<Box<[String]>>().join("\n");

    let with = (0..n).map(|i|
        format!("#[inline(always)] pub const fn with_{c}(mut self, value: T) -> Self {{ self.{c} = value; self }}", c = COMPONENTS[i])
    ).collect::<Box<[String]>>().join("");

    let with_swizzle = VEC_TYPES.iter().filter_map(|value_type| {
        if value_type.n > n {
            return None;
        };

        let value_type_name = &value_type.name;

        Some(format!(
            "\
            with_swizzle_fns!({value_type_name}<T>, T, [
                {}
            ]);\
            ",
            (0..n.pow(value_type.n as u32)).filter_map(|combination| {
                let mut combination_str = String::with_capacity(value_type.n);
                let mut copy_instructions = String::new();
                
                let mut previous_self_component = 0;
                let mut current_copy_len = 1;
                for value_component in 0..value_type.n {
                    let self_component = combination / n.pow(value_component as u32) % n;
    
                    let self_component_c = COMPONENTS[self_component];
                    let value_component_c = COMPONENTS[value_component];
    
                    if combination_str.contains(self_component_c) {
                        return None;
                    }
    
                    combination_str.push(self_component_c);
                    if value_component == 0 {
                        copy_instructions.push_str(&format!("{value_component_c} -> {self_component_c}"));
                    }
                    else if self_component == previous_self_component + 1 {
                        current_copy_len += 1;
                    }
                    else {
                        if current_copy_len > 1 {
                            copy_instructions.push_str(&format!(" * {current_copy_len}"));
                        }
                        copy_instructions.push_str(&format!(", {value_component_c} -> {self_component_c}"));

                        current_copy_len = 1;
                    };
    
                    previous_self_component = self_component;
                }
                if current_copy_len > 1 {
                    copy_instructions.push_str(&format!(" * {current_copy_len}"));
                }
    
                let fn_name = if value_type.is_aligned { format!("with_{combination_str}_a") } else { format!("with_{combination_str}") };
    
                Some(format!(
                    "({fn_name}, [{copy_instructions}]),"
                ))
            }).collect::<Box<[String]>>().join(" ")
        ))
    }).collect::<Box<[String]>>().join("\n");

    cleanup_rs(
        &format!(
            "
            use crate::*;
            
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct {_self}<T: Element> {{
                {fields}
                {alignment_field}
            }}

            #[inline(always)]
            pub const fn {_self_lower}<T: Element>({args}) -> {_self}<T> {{
                {_self}::new({reference_args})
            }}
            impl<T: Element> {_self}<T> {{
                #[inline(always)]
                pub const fn new({args}) -> Self {{
                    unsafe {{
                        copy_elements_init!(Self, T, [{copy_components}])
                    }}
                }}
                #[inline(always)]
                pub const fn splat(value: T) -> Self {{
                    unsafe {{
                        copy_elements_init!(Self, T, [{copy_components_splat}])
                    }}
                }}
            }}

            impl<T: Element> std::fmt::Display for {_self}<T> {{
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                    write!(f, {format_self})
                }}
            }}

            impl<T: Element> {_self}<T> {{
                {swizzle}
            }}
            impl<T: Element> {_self}<T> {{
                {set_swizzle}
            }}
            impl<T: Element> {_self}<T> {{
                {with}
                
                {with_swizzle}
            }}
            "
        )
    )
}

fn cleanup_rs(src: &str) -> String {
    let src = src.trim();
    
    let mut dst = String::with_capacity(src.len());
    let mut lvl = 0usize;

    for line in src.split("\n") {
        let line = line.trim();

        let mut starting_lvl = lvl;
        let mut inner_lvl = 0;
        let mut ending_lvl = lvl;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' => inner_lvl += 1,
                ')' | ']' | '}' => {
                    if inner_lvl > 0 {
                        inner_lvl -= 1;
                    }
                    else {
                        starting_lvl -= 1;
                        ending_lvl -= 1;
                    }
                },
                _ => {},
            }
        };

        ending_lvl += inner_lvl;
        lvl = starting_lvl;

        if line.len() > 0 {
            for _ in 0..lvl {
                dst.push('\t');
            }
            dst.push_str(line);
        }
        dst.push('\n');

        lvl = ending_lvl;
    }
    
    dst.remove(dst.len() - 1);

    {
        let regex = Regex::new(r"\n+(\s*[\)\]\}])").unwrap();
        dst = regex.replace_all(&dst, "\n$1").into_owned();
    };

    dst
}