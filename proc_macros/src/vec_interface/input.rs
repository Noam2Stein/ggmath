use syn::{ext::IdentExt, ItemType, TraitBound};

use super::*;

#[derive(Clone)]
pub struct VecInterface {
    pub ident: Ident,
    pub generics: Generics,
    pub supertraits: Vec<TraitBound>,
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

#[derive(Clone)]
pub struct VecInterfaceFn {
    pub sig: Signature,
    pub defaults: [[Block; 3]; 2],
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

        <Token![;]>::parse(input)?;

        let mut impls = Vec::new();
        while !input.is_empty() {
            impls.push(input.parse()?);
        }

        Ok(Self {
            ident,
            generics,
            supertraits,
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

impl Parse for VecInterfaceFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            sig: input.parse()?,
            defaults: evaluate_item(input.parse()?)?,
        })
    }
}

fn evaluate_match<const N: usize, S: Parse + Clone>(
    input: VecInterfaceItemMatch<S>,
    expected_keys: [&str; N],
) -> Result<[VecInterfaceItemTree<S>; N], Error> {
    let mut values = [(); N].map(|_| None);

    for pat in input.pats {
        for pat_key in pat.keys {
            let pat_key_str = pat_key.to_string();
            expected_keys
                .into_iter()
                .position(|expected_key| expected_key == pat_key_str)
                .map_or_else(
                    || {
                        Err(Error::new(
                            pat_key.span(),
                            format!("unexpected key '{pat_key_str}'"),
                        ))
                    },
                    |position| {
                        let value = &mut values[position];
                        if value.is_some() {
                            Err(Error::new(
                                pat_key.span(),
                                format!("'{pat_key_str}' already covered"),
                            ))
                        } else {
                            *value = Some(pat.value.clone());
                            Ok(())
                        }
                    },
                )?;
        }
    }

    ArrayExt::try_map(ArrayExt::zip(values, expected_keys), |(value, key)| {
        value.map_or_else(
            || {
                Err(Error::new(
                    input._brace.span.close(),
                    format!("'{key}' not covered"),
                ))
            },
            |value| Ok(value),
        )
    })
}
fn evaluate_item<S: Parse + Clone>(input: VecInterfaceItemTree<S>) -> Result<[[S; 3]; 2], Error> {
    redirect_match(
        input,
        |input| {
            Ok([
                [input.clone(), input.clone(), input.clone()],
                [input.clone(), input.clone(), input],
            ])
        },
        |input| {
            let [[aligned2, packed2], [aligned3, packed3], [aligned4, packed4]] =
                ArrayExt::try_map(evaluate_match(input, LEN_STRS)?, evaluate_fn_defaults_for_n)?;

            Ok([[aligned2, aligned3, aligned4], [packed2, packed3, packed4]])
        },
        |input| ArrayExt::try_map(evaluate_match(input, ALIGN_STRS)?, redirect_a_match),
    )
}
fn evaluate_fn_defaults_for_n<S: Parse + Clone>(
    input: VecInterfaceItemTree<S>,
) -> Result<[S; 2], Error> {
    redirect_match(
        input,
        |input| Ok([input.clone(), input]),
        |input| Err(Error::new(input.item.span(), "'N' already matched")),
        |input| ArrayExt::try_map(evaluate_match(input, ALIGN_STRS)?, redirect_n_match),
    )
}
fn redirect_a_match<S: Parse + Clone>(input: VecInterfaceItemTree<S>) -> Result<[S; 3], Error> {
    redirect_match(
        input,
        |input| Ok([input.clone(), input.clone(), input]),
        |input| ArrayExt::try_map(evaluate_match(input, LEN_STRS)?, redirect_n_match),
        |input| Err(Error::new(input.item.span(), "'A' already matched")),
    )
}
fn redirect_n_match<S: Parse + Clone>(input: VecInterfaceItemTree<S>) -> Result<S, Error> {
    redirect_match(
        input,
        |input| Ok(input),
        |input| Err(Error::new(input.item.span(), "'N' already matched")),
        |input| Err(Error::new(input.item.span(), "'A' already matched")),
    )
}
fn redirect_match<S: Parse + Clone, Output>(
    input: VecInterfaceItemTree<S>,
    handle_single: impl FnOnce(S) -> Result<Output, Error>,
    handle_match_n: impl FnOnce(VecInterfaceItemMatch<S>) -> Result<Output, Error>,
    handle_match_a: impl FnOnce(VecInterfaceItemMatch<S>) -> Result<Output, Error>,
) -> Result<Output, Error> {
    match input {
        VecInterfaceItemTree::Single(input) => handle_single(input),
        VecInterfaceItemTree::Match(input) => match input.item.to_string().as_str() {
            "N" => handle_match_n(input),
            "A" => handle_match_a(input),
            _ => Err(Error::new(input.item.span(), "expected either 'N' or 'A'")),
        },
    }
}

#[derive(Parse, Clone)]
enum VecInterfaceItemTree<S: Parse + Clone> {
    #[peek(Token![@], name = "Match")]
    Match(VecInterfaceItemMatch<S>),
    #[peek_with(|_| true, name = "Single")]
    Single(S),
}
#[derive(Parse, Clone)]
struct VecInterfaceItemMatch<S: Parse + Clone> {
    #[prefix(Token![@])]
    #[prefix(Token![match])]
    item: Ident,
    #[brace]
    _brace: Brace,
    #[inside(_brace)]
    #[call(Self::parse_pats)]
    pats: Vec<VecInterfaceItemMatchPat<S>>,
}
impl<S: Parse + Clone> VecInterfaceItemMatch<S> {
    fn parse_pats(input: ParseStream) -> syn::Result<Vec<VecInterfaceItemMatchPat<S>>> {
        let mut output = Vec::new();
        while !input.is_empty() {
            output.push(input.parse()?);
            input.parse::<Token![,]>()?;
        }

        Ok(output)
    }
}
#[derive(Parse, Clone)]
struct VecInterfaceItemMatchPat<S: Parse + Clone> {
    #[call(Punctuated::parse_separated_nonempty)]
    keys: Punctuated<TokenTree, syn::token::Or>,
    #[prefix(Token![=>])]
    value: VecInterfaceItemTree<S>,
}
