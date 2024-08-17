use proc_macro2::Span;
use strum_macros::{Display, EnumIter, EnumString};
use syn::Ident;

use crate::vec::VecType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, EnumString, Display)]
pub enum Op {
    Neg,
    Not,    
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, EnumString, Display)]
pub enum RhsOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}
impl Op {
    pub fn trait_(self) -> Ident {
        Ident::new(&self.to_string(), Span::call_site())
    }
    pub fn fn_(self) -> Ident {
        Ident::new(&self.to_string().to_lowercase(), Span::call_site())
    }
    pub fn inner_fn(self, vec_ty: VecType) -> Ident {
        Ident::new(&format!("{}_{}", self.to_string().to_lowercase(), vec_ty.to_string().to_lowercase(), ), Span::call_site())
    }
}
impl RhsOp {
    pub fn trait_(self) -> Ident {
        Ident::new(&self.to_string(), Span::call_site())
    }
    pub fn fn_(self) -> Ident {
        Ident::new(&self.to_string().to_lowercase(), Span::call_site())
    }
    pub fn inner_fn(self, vec_ty: VecType) -> Ident {
        Ident::new(&format!("{}_{}", self.to_string().to_lowercase(), vec_ty.to_string().to_lowercase(), ), Span::call_site())
    }
    pub fn assign_trait(self) -> Ident {
        Ident::new(&format!("{self}Assign"), Span::call_site())
    }
    pub fn assign_fn(self) -> Ident {
        Ident::new(&format!("{}_assign", self.to_string().to_lowercase()), Span::call_site())
    }
    pub fn inner_assign_fn(self, vec_ty: VecType) -> Ident {
        Ident::new(&format!("{}_assign_{}", self.to_string().to_lowercase(), vec_ty.to_string().to_lowercase(), ), Span::call_site())
    }
}