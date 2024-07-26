use std::{env, fs::{create_dir_all, remove_file, write}, ops::Range, path::PathBuf};

const VECS: Range<usize> = 2..5;
const COMPONENTS: [char; 4] = ['x', 'y', 'z', 'w'];

fn main() {
    let src_dir = PathBuf::from_iter([
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()),
        PathBuf::from("src/gen"),
    ].iter());

    for path in std::fs::read_dir(&src_dir).unwrap() {
        remove_file(path.unwrap().path()).unwrap();
    };

    let write = |path: &str, str: &str| {
        let full_path = PathBuf::from_iter([&src_dir, &PathBuf::from(path)].iter());

        let full_dir = full_path.parent().unwrap();
        if !full_dir.exists() {
            create_dir_all(full_dir).unwrap();
        }

        write(full_path, str.as_bytes()).unwrap();
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
fn vec_rs(n: usize, aligned: bool) -> String {
    let name = if aligned { format!("Vec{n}A") } else { format!("Vec{n}") };
    let name_lower = if aligned { format!("vec{n}a") } else { format!("vec{n}") };

    let components = (0..n).map(|i| COMPONENTS[i].to_string()).collect::<Box<[String]>>();
    let align_components = if aligned {
        (n..n.next_power_of_two()).map(|i| format!("_{i}")).collect::<Box<[String]>>()
    }
    else {
        Box::new([])
    };

    let fields = components.iter().map(|c| format!("pub(crate) {c}: C,")).collect::<Box<[String]>>().join("\n");
    let align_fields = align_components.iter().map(|c| format!("pub(crate) {c}: C,")).collect::<Box<[String]>>().join("\n");

    let constructor_fields = components.iter().map(|c| format!("{c},")).collect::<Box<[String]>>().join("\n");
    let constructor_align_fields = align_components.iter().map(|c| format!("{c}: unsafe {{ std::mem::zeroed() }},")).collect::<Box<[String]>>().join("\n");
    
    let args = components.iter().map(|c| format!("{c}: C")).collect::<Box<[String]>>().join(", ");

    let use_args = components.iter().map(|c| format!("{c}")).collect::<Box<[String]>>().join(", ");

    let format_self = format!(
        "\"({})\", {}",
        components.iter().map(|_| "{}").collect::<Box<[&str]>>().join(", "),
        components.iter().map(|c| format!("self.{c}")).collect::<Box<[String]>>().join(", ")
    );

    let swizzle = {
        let mut swizzle = String::new();
        let mut combo = Vec::with_capacity(VECS.end - 1);
        for dst_n in VECS {
            let dst_ty = format!("Vec{dst_n}");
            for combo_index in 0..n.pow(dst_n as u32) {
                combo.clear();
                for slot in (0..dst_n).rev() {
                    combo.push(
                        combo_index / n.pow(slot as u32) % n
                    );
                }

                let fn_name = combo.iter().map(|i| COMPONENTS[*i]).collect::<String>();
                
                let mut instructions = String::new();
                let mut instruction_len = 1;
                instructions.push_str(&format!("{} -> {} * ", COMPONENTS[combo[0]], COMPONENTS[0]));
                for slot in 1..dst_n {
                    let src_component = combo[slot];
                    let src_prev_component = combo[slot - 1];

                    if src_component == src_prev_component + 1 {
                        instruction_len += 1;
                    }
                    else {
                        instructions.push_str(&format!("{}, {} -> {} * ", &instruction_len.to_string(), COMPONENTS[src_component], COMPONENTS[slot]));
                    }
                };
                instructions.push_str(&instruction_len.to_string());

                swizzle.push_str(&format!("\
                    #[inline(always)] pub const fn {fn_name}(self) -> {dst_ty}<C> {{ unsafe {{ swizzle!(self, {dst_ty}, C, [{instructions}]) }} }}
                "));

                if !dst_n.is_power_of_two() {
                    swizzle.push_str(&format!("\
                        #[inline(always)] pub const fn {fn_name}_a(self) -> {dst_ty}A<C> {{ unsafe {{ swizzle!(self, {dst_ty}A, C, [{instructions}]) }} }}
                    "));
                }
            }
        }

        swizzle.trim().to_string()
    };

    cleanup_rs(
        &format!(
            "
            use crate::*;
            
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct {name}<C: Component> {{
                {fields}
                {align_fields}
            }}

            #[inline(always)]
            pub const fn {name_lower}<C: Component>({args}) -> {name}<C> {{
                {name}::new({use_args})
            }}
            impl<C: Component> {name}<C> {{
                #[inline(always)]
                pub const fn new({args}) -> Self {{
                    Self {{
                        {constructor_fields}
                        {constructor_align_fields}
                    }}
                }}
            }}

            impl<C: Component> std::fmt::Display for {name}<C> {{
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                    write!(f, {format_self})
                }}
            }}

            impl<C: Component> {name}<C> {{
                {swizzle}
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

    dst
    .replace("\n\n\n\n\n", "\n\n")
    .replace("\n\n\n\n", "\n\n")
    .replace("\n\n\n", "\n\n")
    .replace("\n\n)", "\n)")
    .replace("\n\n]", "\n]")
    .replace("\n\n}", "\n}")
}