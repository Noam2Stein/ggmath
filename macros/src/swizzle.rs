use std::sync::LazyLock;

use proc_macro2::Span;
use strum::IntoEnumIterator;
use syn::Ident;

use crate::vec::VecType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SwizzleOp {
    Get,
    Mut,
    Set,
    With,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Swizzle {
    pub self_ty: VecType,
    pub value_ty: VecType,
    pub reflections: Vec<SwizzleReflection>,
    pub str: String,
    pub op: SwizzleOp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwizzleReflection {
    pub self_component: usize,
    pub value_component: usize,
    pub len: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VecSwizzles {
    pub get: Vec<Swizzle>,
    pub mut_: Vec<Swizzle>,
    pub set: Vec<Swizzle>,
    pub with: Vec<Swizzle>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Swizzles {
    pub vec2: VecSwizzles,
    pub vec3: VecSwizzles,
    pub vec3a: VecSwizzles,
    pub vec4: VecSwizzles,
}
impl Swizzle {
    pub fn ident_str(&self) -> String {
        let mut output = match self.op {
            SwizzleOp::Get => format!("{}", self.str),
            SwizzleOp::Mut => format!("{}_mut", self.str),
            SwizzleOp::Set => format!("set_{}", self.str),
            SwizzleOp::With => format!("with_{}", self.str),
        };

        if self.value_ty == VecType::Vec3A {
            output.push_str("_a");
        }

        output
    }
    pub fn inner_ident_str(&self) -> String {
        format!("{}_{}", self.ident_str(), self.self_ty.ident_lower())
    }
    pub fn ident(&self) -> Ident {
        Ident::new(&self.ident_str(), Span::call_site())
    }
    pub fn inner_ident(&self) -> Ident {
        Ident::new(&self.inner_ident_str(), Span::call_site())
    }
}
impl VecSwizzles {
    pub fn iter(&self) -> impl Iterator<Item = &Swizzle> + Clone {
        self.get.iter().chain(
            self.mut_.iter().chain(
                self.set.iter().chain(
                    self.with.iter()
                )
            )
        )
    }
}
impl Swizzles {
    pub fn iter(&self) -> impl Iterator<Item = &Swizzle> + Clone {
        self.vec2.iter().chain(
            self.vec3.iter().chain(
                self.vec3a.iter().chain(
                    self.vec4.iter()
                )
            )
        )
    }
    pub fn get_iter(&self) -> impl Iterator<Item = &Swizzle> + Clone {
        self.vec2.get.iter().chain(
            self.vec3.get.iter().chain(
                self.vec3a.get.iter().chain(
                    self.vec4.get.iter()
                )
            )
        )
    }
    pub fn mut_iter(&self) -> impl Iterator<Item = &Swizzle> + Clone {
        self.vec2.mut_.iter().chain(
            self.vec3.mut_.iter().chain(
                self.vec3a.mut_.iter().chain(
                    self.vec4.mut_.iter()
                )
            )
        )
    }
    pub fn set_iter(&self) -> impl Iterator<Item = &Swizzle> + Clone {
        self.vec2.set.iter().chain(
            self.vec3.set.iter().chain(
                self.vec3a.set.iter().chain(
                    self.vec4.set.iter()
                )
            )
        )
    }
    pub fn with_iter(&self) -> impl Iterator<Item = &Swizzle> + Clone {
        self.vec2.with.iter().chain(
            self.vec3.with.iter().chain(
                self.vec3a.with.iter().chain(
                    self.vec4.with.iter()
                )
            )
        )
    }
}

pub static SWIZZLES: LazyLock<Swizzles> = LazyLock::new(|| {
    let mut output = Swizzles {
        vec2: VecSwizzles {
            get: Vec::with_capacity(2usize.pow(2) + 2usize.pow(3) * 2 + 2usize.pow(4)),
            mut_: Vec::with_capacity(2 + 1 - 1 + 2 + 1 - 2),
            set: Vec::with_capacity([1, 2].map(|i| (0..i).fold(1, |acc, i| acc * (2 - i))).iter().sum()),
            with: Vec::with_capacity([1, 2].map(|i| (0..i).fold(1, |acc, i| acc * (2 - i))).iter().sum()),
        },
        vec3: VecSwizzles {
            get: Vec::with_capacity(3usize.pow(2) + 3usize.pow(3) * 2 + 3usize.pow(4)),
            mut_: Vec::with_capacity(3 + 1 - 1 + 3 + 1 - 2 + 3 + 1 - 3),
            set: Vec::with_capacity([1, 2, 3, 3].map(|i| (0..i).fold(1, |acc, i| acc * (3 - i))).iter().sum()),
            with: Vec::with_capacity([1, 2, 3, 3].map(|i| (0..i).fold(1, |acc, i| acc * (3 - i))).iter().sum()),
        },
        vec3a: VecSwizzles {
            get: Vec::with_capacity(3usize.pow(2) + 3usize.pow(3) * 2 + 3usize.pow(4)),
            mut_: Vec::with_capacity(3 + 1 - 1 + 3 + 1 - 2 + 3 + 1 - 3),
            set: Vec::with_capacity([1, 2, 3, 3].map(|i| (0..i).fold(1, |acc, i| acc * (3 - i))).iter().sum()),
            with: Vec::with_capacity([1, 2, 3, 3].map(|i| (0..i).fold(1, |acc, i| acc * (3 - i))).iter().sum()),
        },
        vec4: VecSwizzles {
            get: Vec::with_capacity(4usize.pow(2) + 4usize.pow(3) * 2 + 4usize.pow(4)),
            mut_: Vec::with_capacity(4 + 1 - 1 + 4 + 1 - 2 + 4 + 1 - 3 + 4 + 1 - 4),
            set: Vec::with_capacity([1, 2, 3, 3, 4].map(|i| (0..i).fold(1, |acc, i| acc * (4 - i))).iter().sum()),
            with: Vec::with_capacity([1, 2, 3, 3, 4].map(|i| (0..i).fold(1, |acc, i| acc * (4 - i))).iter().sum()),
        }
    };

    let mut str = String::with_capacity(4);
    let mut reflections = Vec::<SwizzleReflection>::with_capacity(4);

    for self_ty in VecType::iter() {
        let self_len = self_ty.len();
        if self_len < 2 {
            continue;
        };

        let swizzles = match self_ty {
            VecType::SingleElement => unreachable!(),
            VecType::Vec2 => &mut output.vec2,
            VecType::Vec3 => &mut output.vec3,
            VecType::Vec3A => &mut output.vec3a,
            VecType::Vec4 => &mut output.vec4,
        };

        for value_ty in VecType::iter() {
            let value_len = value_ty.len();

            for combination in 0..self_len.pow(value_len as u32) {
                str.clear();
                reflections.clear();

                let (is_continuous, all_unique) = {
                    let mut is_continuous = true;
                    let mut all_unique = true;
                    
                    let mut previous_self_component = 0;
                    for value_component in 0..value_len {
                        let self_component = combination / self_len.pow(value_component as u32) % self_len;
                        
                        let c = ['x', 'y', 'z', 'w'][self_component];
                        if str.contains(c) {
                            all_unique = false;
                        }
                        str.push(c);

                        if value_component > 0 && self_component == previous_self_component + 1 {
                            reflections.last_mut().unwrap().len += 1;
                        }
                        else {
                            reflections.push(
                                SwizzleReflection {
                                    self_component,
                                    value_component,
                                    len: 1,
                                }
                            );

                            if value_component > 0 {
                                is_continuous = false;
                            }
                        }
    
                        previous_self_component = self_component;
                    }

                    (is_continuous, all_unique)
                };

                swizzles.get.push(
                    Swizzle {
                        self_ty,
                        value_ty,
                        reflections: reflections.clone(),
                        str: str.clone(),
                        op: SwizzleOp::Get,
                    }
                );
                if is_continuous {
                    swizzles.mut_.push(
                        Swizzle {
                            self_ty,
                            value_ty,
                            reflections: reflections.clone(),
                            str: str.clone(),
                            op: SwizzleOp::Mut,
                        }
                    );
                }
                if all_unique {
                    swizzles.set.push(
                        Swizzle {
                            self_ty,
                            value_ty,
                            reflections: reflections.clone(),
                            str: str.clone(),
                            op: SwizzleOp::Set,
                        }
                    );
                    swizzles.with.push(Swizzle {
                        self_ty,
                        value_ty,
                        reflections: reflections.clone(),
                        str: str.clone(),
                        op: SwizzleOp::With,
                    });
                }
            }
        }
    }

    output
});