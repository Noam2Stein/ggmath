use syn::{braced, ext::IdentExt, ItemType, TraitBound, TraitItem};

use super::*;

#[derive(Clone)]
pub struct VecInterface {
    pub ident: Ident,
    pub generics: Generics,
    pub supertraits: Vec<TraitBound>,
    pub scalar_items: Vec<TraitItem>,
    pub impls: Vec<VecInterfaceImpl>,
}

#[derive(Clone)]
pub struct VecInterfaceImpl {
    pub vis: Option<Token![pub]>,
    pub generics: Generics,
    pub r#trait: Option<Type>,
    pub fns: Vec<VecInterfaceFn>,
    pub assoc_types: Vec<ItemType>,
    pub errors: Vec<Error>,
}

#[derive(Clone, Parse)]
pub struct VecInterfaceFn {
    pub sig: Signature,
    pub default: Block,
}

impl Parse for VecInterface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = Parse::parse(input)
            .map_err(|err| Error::new(err.span(), &format!("expected the scalar trait's ident")))?;

        let mut generics = Generics::parse(input)?;

        let supertraits = if input.parse::<Option<Token![:]>>()?.is_some() {
            Punctuated::<TraitBound, Token![+]>::parse_separated_nonempty(input)?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };

        generics.where_clause = Parse::parse(input)?;

        let scalar_items = if <Option<Token![;]>>::parse(input)?.is_some() {
            Vec::new()
        } else {
            let stream;
            braced!(stream in input);

            let mut scalar_items = Vec::new();
            while !stream.is_empty() {
                scalar_items.push(stream.parse()?);
            }

            scalar_items
        };

        let mut impls = Vec::new();
        while !input.is_empty() {
            impls.push(input.parse()?);
        }

        Ok(Self {
            ident,
            generics,
            supertraits,
            scalar_items,
            impls,
        })
    }
}

impl Parse for VecInterfaceImpl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = match Visibility::parse(input)? {
            Visibility::Inherited => None,
            Visibility::Public(vis) => Some(vis),
            Visibility::Restricted(vis) => {
                return Err(Error::new(
                    vis.span(),
                    "restricted visibility is not supported supported",
                ))
            }
        };

        <Token![impl]>::parse(input)?;

        let mut generics = Generics::parse(input)?;

        let r#trait = if input.peek(Ident::peek_any) {
            Some(input.parse()?)
        } else {
            None
        };

        generics.where_clause = input.parse()?;

        <Token![:]>::parse(input)?;

        let mut fns = Vec::new();
        let mut assoc_types = Vec::<ItemType>::new();
        let mut errors = Vec::new();

        while !input.is_empty() {
            if input.peek(Token![fn]) || input.peek(Token![unsafe]) || input.peek(Token![async]) {
                match input.parse() {
                    Ok(ok) => fns.push(ok),
                    Err(err) => errors.push(err),
                }
            } else if input.peek(Token![type]) {
                match input.parse() {
                    Ok(ok) => assoc_types.push(ok),
                    Err(err) => errors.push(err),
                }
            } else {
                errors.push(Error::new(
                    input.parse::<TokenTree>()?.span(),
                    "expected a trait item",
                ));
                while !input.is_empty()
                    && !input.peek(Token![fn])
                    && !input.peek(Token![unsafe])
                    && !input.peek(Token![async])
                    && !input.peek(Token![type])
                {
                    TokenTree::parse(input).unwrap();
                }
            }
        }

        if r#trait.is_none() {
            for assoc_type in &assoc_types {
                errors.push(Error::new(
                    assoc_type.ident.span(),
                    "associated types are only allowed when using impl trait",
                ));
            }
        }

        Ok(Self {
            vis,
            generics,
            r#trait,
            fns,
            assoc_types,
            errors,
        })
    }
}
