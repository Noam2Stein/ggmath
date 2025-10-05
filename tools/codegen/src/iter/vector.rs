use strum::IntoEnumIterator;

use crate::iter::{Length, Primitive, Simdness};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub n: Length,
    pub t: Primitive,
    pub s: Simdness,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct VectorInfo {
    pub n: Length,
    #[expect(dead_code)]
    pub t: Primitive,
    pub s: Simdness,
    pub alias: String,
    pub r#macro: String,
    pub name_snakecase: String,
    #[expect(dead_code)]
    pub is_test_unique: bool,
}

impl Vector {
    pub fn test_iter(t: impl Into<Primitive>) -> impl Iterator<Item = Self> {
        let t = t.into();

        Length::iter()
            .flat_map(move |n| Simdness::iter().map(move |s| Self { n, t, s }))
            .filter(|v| v.is_test_unique())
    }

    pub fn alias(&self) -> String {
        format!(
            "Vec{n}{s_postfix}",
            n = self.n,
            s_postfix = self.s.postfix_uppercase(),
        )
    }

    pub fn r#macro(&self) -> String {
        format!(
            "vec{n}{s_postfix}",
            n = self.n,
            s_postfix = self.s.postfix_lowercase(),
        )
    }

    pub fn name_snakecase(&self) -> String {
        format!(
            "{t_prefix}vec{n}{s_postfix}",
            t_prefix = self.t.prefix_lowercase(),
            n = self.n,
            s_postfix = self.s.postfix_lowercase(),
        )
    }

    pub fn is_test_unique(&self) -> bool {
        if self.t.is_primary() {
            true
        } else if self.s == Simdness::Simd {
            true
        } else if self.n == Length::Four {
            true
        } else {
            false
        }
    }

    pub fn info(&self) -> VectorInfo {
        VectorInfo {
            n: self.n,
            t: self.t,
            s: self.s,
            alias: self.alias(),
            r#macro: self.r#macro(),
            name_snakecase: self.name_snakecase(),
            is_test_unique: self.is_test_unique(),
        }
    }
}
