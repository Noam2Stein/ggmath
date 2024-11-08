use super::*;

use syn::{
    punctuated::Punctuated, token::Brace, Block, ConstParam, FnArg, GenericParam, Generics, Lit,
    LitInt, Pat, Receiver, Signature, Type, TypeParam, Visibility,
};

const LEN_STRS: [&str; 3] = ["2", "3", "4"];
const ALIGN_STRS: [&str; 2] = ["VecAligned", "VecPacked"];

fn scalar_fn_ident(ident: &Ident, n: &str, a: &str) -> Ident {
    Ident::new(
        &format!(
            "{}_vec{n}_{ident}",
            match a {
                "VecAligned" => "aligned",
                "VecPacked" => "packed",
                _ => unreachable!(),
            }
        ),
        ident.span(),
    )
}
fn len_trait_ident(input: &VecInterface) -> Ident {
    Ident::new(&format!("VecLen{}", input.ident), input.ident.span())
}
fn alignment_trait_ident(input: &VecInterface) -> Ident {
    Ident::new(&format!("VecAlignment{}", input.ident), input.ident.span())
}

pub fn vec_interface(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as VecInterface);

    let errors = input.errors.iter().map(|err| err.to_compile_error());
    let impl_block = impl_block(&input);
    let scalar = scalar(&input);
    let len = len(&input);
    let storage = alignment(&input);

    quote_spanned! {
        input.ident.span() =>
        #[allow(unused_imports)]
        use crate::vector::{alignment::*, inner::*, length::*, *};

        const _: () = {
            #(#errors)*
        };
        #impl_block
        #scalar
        #len
        #storage
    }
    .into()
}

struct VecInterface {
    vis: Option<Token![pub]>,
    impl_trait: Option<Token![impl]>,
    ident: Ident,
    generics: Generics,
    scalar_trait: TypeParam,
    fns: Vec<VecInterfaceFn>,
    assoc_types: Vec<VecInterfaceAssocType>,
    errors: Vec<Error>,
}
impl Parse for VecInterface {
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

        let impl_trait = Option::parse(input)?;

        let ident = Ident::parse(input)
            .map_err(|err| Error::new(err.span(), "expected the interface's ident"))?;

        let mut generics = Generics::parse(input)?;
        if input.peek(Token![where]) {
            generics.where_clause = input.parse()?;
        }

        <Token![:]>::parse(input)?;

        let scalar_trait = TypeParam::parse(input)
            .map_err(|err| Error::new(err.span(), "expected the scalar trait's ident"))?;
        <Token![,]>::parse(input)?;

        let mut fns = Vec::new();
        let mut assoc_types = Vec::<VecInterfaceAssocType>::new();
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
                    "expected Signature",
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

        if impl_trait.is_none() {
            for assoc_type in &assoc_types {
                errors.push(Error::new(
                    assoc_type.ident.span(),
                    "associated types are only allowed when using impl trait",
                ));
            }
        }

        Ok(Self {
            vis,
            impl_trait,
            ident,
            generics,
            scalar_trait,
            fns,
            assoc_types,
            errors,
        })
    }
}

#[derive(Clone)]
struct VecInterfaceFn {
    sig: Signature,
    defaults: [[Block; 3]; 2],
}
impl Parse for VecInterfaceFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            sig: input.parse()?,
            defaults: evaluate_item(input.parse()?)?,
        })
    }
}

#[derive(Clone, Parse)]
struct VecInterfaceAssocType {
    #[prefix(Token![type])]
    ident: Ident,
    generics: Generics,
    #[prefix(Token![=])]
    value: VecInterfaceItemTree<Type>,
    _semi: Token![;],
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

fn impl_block(input: &VecInterface) -> TokenStream {
    let scalar_trait = &input.scalar_trait.ident;
    let len_trait = len_trait_ident(input);

    let generics = input.generics.params.iter();
    let generics_args = generic_args(&input.generics);

    let impl_trait = input.impl_trait.map(|impl_| {
        let trait_ = &input.ident;
        quote_spanned! { impl_.span() => #trait_<#(#generics_args), *> for }
    });

    let fn_impls = input.fns.iter().map(|r#fn| {
        let vis = input.vis;

        let sig = &r#fn.sig;
        let ident = &r#fn.sig.ident;
        let generics = generic_args(&r#fn.sig.generics);
        let input_idents = r#fn.sig.inputs.iter().map(arg_ident);

        quote_spanned! {
            ident.span() =>
            #[inline(always)]
            #[allow(unused_mut)]
            #vis #sig {
                <ScalarCount<N> as #len_trait<N>>::#ident::<T, A, #(#generics), *>(#(#input_idents), *)
            }
        }
    });

    quote_spanned! {
        input.ident.span() =>
        impl<const N: usize, T: #scalar_trait<#(#generics_args), *>, A: VecAlignment, #(#generics), *> #impl_trait Vector<N, T, A> where ScalarCount<N>: VecLen<N> {
            #(#fn_impls)*
        }
    }
}
fn scalar(input: &VecInterface) -> TokenStream {
    let trait_attrs = &input.scalar_trait.attrs;
    let trait_ident = &input.scalar_trait.ident;
    let trait_supertraits = &input.scalar_trait.bounds;

    let generics = &input.generics;
    let where_clause = &input.generics.where_clause;

    let fn_declarations = input
        .fns
        .iter()
        .map(|r#fn| {
            ALIGN_STRS
                .zip(r#fn.defaults.clone())
                .map(|(a_str, defaults)| {
                    let a = Ident::new(a_str, r#fn.sig.span());

                    LEN_STRS.zip(defaults).map(|(n_str, default)| {
                        let n = LitInt::new(n_str, r#fn.sig.span());

                        let mut sig = r#fn.sig.clone();
                        sig.ident = scalar_fn_ident(&sig.ident, n_str, a_str);

                        search_replace_fn(
                            quote_spanned! { sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                            sig.clone(),
                            Some(default.to_token_stream()),
                            |span| quote_spanned! { span => #n },
                            |span| quote_spanned! { span => Self },
                            |span| quote_spanned! { span => #a },
                        )
                    })
                })
        })
        .flatten()
        .flatten();

    quote_spanned! {
        trait_ident.span() =>
        #(#trait_attrs)*
        pub trait #trait_ident #generics: #trait_supertraits #where_clause {
            #(#fn_declarations)*
        }
    }
}
fn len(input: &VecInterface) -> TokenStream {
    let scalar_trait = &input.scalar_trait.ident;
    let len_trait = len_trait_ident(input);
    let alignment_trait = alignment_trait_ident(input);

    let generic_params = input.generics.params.iter();

    let fn_sigs = input
        .fns
        .iter()
        .map(|r#fn| {
            let mut sig = r#fn.sig.clone();
            sig.generics.params.insert(
                0,
                parse_quote_spanned! { sig.ident.span() => T: #scalar_trait },
            );
            sig.generics.params.insert(
                1,
                parse_quote_spanned! { sig.ident.span() => A: VecAlignment },
            );

            sig
        })
        .collect::<Box<[Signature]>>();

    let fn_declarations = fn_sigs.iter().map(|sig| {
        search_replace_fn(
            quote_spanned! { sig.fn_token.span => #[allow(missing_docs)] },
            sig.clone(),
            None,
            |span| quote_spanned! { span => N },
            |span| quote_spanned! { span => T },
            |span| quote_spanned! { span => A },
        )
    });

    let impls = LEN_STRS.map(|n_str| {
        let n = Lit::Int(LitInt::new(n_str, len_trait.span()));

        let fn_impls = fn_sigs.iter().map(|sig| {
            let n = Lit::Int(LitInt::new(n_str, sig.ident.span()));

            let fn_ident = &sig.ident;
            let generics = &generic_args(&sig.generics)[2..];
            let input_idents = sig.inputs.iter().map(|input| arg_ident(input));

            search_replace_fn(
                quote_spanned! { sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                sig.clone(),
                Some(quote_spanned! { sig.ident.span() => {
                    <A as #alignment_trait<N>>::#fn_ident::<T, #(#generics), *>(#(#input_idents), *)
                } }),
                |span| quote_spanned! { span => #n },
                |span| quote_spanned! { span => T },
                |span| quote_spanned! { span => A },
            )
        });

        quote_spanned! {
            input.ident.span() =>
            impl #len_trait<#n> for ScalarCount<#n> {
                #(#fn_impls)*
            }
        }
    });

    quote_spanned! {
        len_trait.span() =>
        pub(super) trait #len_trait<const N: usize #(, #generic_params)*>: VecLenInnerVec where ScalarCount<N>: VecLen<N> {
            #(#fn_declarations)*
        }

        #(#impls)*
    }
}
fn alignment(input: &VecInterface) -> TokenStream {
    let scalar_trait = &input.scalar_trait.ident;
    let alignment_trait = alignment_trait_ident(input);

    let generic_params = input.generics.params.iter();

    let fn_sigs = input
        .fns
        .iter()
        .map(|r#fn| {
            let mut sig = r#fn.sig.clone();
            sig.generics.params.insert(
                0,
                parse_quote_spanned! { sig.ident.span() => T: #scalar_trait },
            );

            sig
        })
        .collect::<Box<[Signature]>>();

    let fn_declarations = fn_sigs.iter().map(|sig| {
        search_replace_fn(
            quote_spanned! { sig.fn_token.span => #[allow(missing_docs)] },
            sig.clone(),
            None,
            |span| quote_spanned! { span => N },
            |span| quote_spanned! { span => T },
            |span| quote_spanned! { span => Self },
        )
    });

    let impls = LEN_STRS
        .map(|n_str| {
            let n = LitInt::new(n_str, alignment_trait.span());
            
            ALIGN_STRS.map(|a_str| {
                let a = Ident::new(a_str, alignment_trait.span());
                
                let generic_params = input.generics.params.iter();

                let fn_impls = fn_sigs.iter().map(|sig| {
                    let scalar_fn = scalar_fn_ident(&sig.ident, n_str, a_str);

                    let n = LitInt::new(n_str, sig.ident.span());

                    let generics = &generic_args(&sig.generics)[1..];
                    let input_idents = sig.inputs.iter().map(|input| arg_ident(input));

                    search_replace_fn(
                        quote_spanned! { sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                        sig.clone(),
                        Some(quote_spanned! { sig.ident.span() => {
                            T::#scalar_fn::<#(#generics), *>(#(#input_idents), *)
                        } }),
                        |span| quote_spanned! { span => #n },
                        |span| quote_spanned! { span => T },
                        |span| quote_spanned! { span => Self },
                    )
                });

                quote_spanned! {
                    input.ident.span() =>
                    impl #alignment_trait<#n #(, #generic_params)*> for #a {
                        #(#fn_impls)*
                    }
                }
            })
        })
        .into_iter()
        .flatten();

    quote_spanned! {
        alignment_trait.span() =>

        pub(super) trait #alignment_trait<const N: usize #(, #generic_params)*>: alignment_seal::VecAlignment where ScalarCount<N>: VecLen<N> {
            #(#fn_declarations)*
        }

        #(#impls)*
    }
}

fn search_replace_fn(
    attrs: TokenStream,
    mut sig: Signature,
    block: Option<TokenStream>,
    n: impl Fn(Span) -> TokenStream + Copy,
    t: impl Fn(Span) -> TokenStream + Copy,
    a: impl Fn(Span) -> TokenStream + Copy,
) -> TokenStream {
    for input in &mut sig.inputs {
        match input {
            FnArg::Receiver(Receiver {
                attrs,
                reference,
                mutability,
                self_token,
                colon_token: _,
                ty: _,
            }) => {
                if let Some((and_token, lifetime)) = reference {
                    *input = parse_quote_spanned! {
                        self_token.span() =>
                        #(#[#attrs])* vec: #and_token #lifetime #mutability Vector<N, T, A>
                    }
                } else {
                    let mutability = mutability.map_or(None, |mutability| {
                        if block.is_some() {
                            Some(mutability)
                        } else {
                            None
                        }
                    });
                    *input = parse_quote_spanned! {
                        self_token.span() =>
                        #(#[#attrs])* #mutability vec: Vector<N, T, A>
                    }
                }
            }
            FnArg::Typed(typed) => {
                if block.is_none() {
                    match &mut *typed.pat {
                        Pat::Type(pat) => match &mut *pat.pat {
                            Pat::Ident(pat) => {
                                pat.mutability = None;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    let mut input = sig.to_token_stream();
    input.extend(block.unwrap_or_else(|| quote_spanned! { sig.fn_token.span() => ; }));

    let mut output = attrs;
    search_replace(
        input,
        &mut output,
        |span| {
            let n = n(span);
            let t = t(span);
            let a = a(span);
            quote_spanned! { span => Vector<#n, #t, #a> }
        },
        |span| quote_spanned! { span => vec },
        n,
        t,
        a,
    );

    output
}
fn search_replace(
    input: TokenStream,
    output: &mut TokenStream,
    self_ty: impl Fn(Span) -> TokenStream + Copy,
    self_arg: impl Fn(Span) -> TokenStream + Copy,
    n_f: impl Fn(Span) -> TokenStream + Copy,
    t: impl Fn(Span) -> TokenStream + Copy,
    a: impl Fn(Span) -> TokenStream + Copy,
) {
    for token in input {
        match token {
            TokenTree::Group(token) => {
                output.append({
                    let mut output = TokenStream::new();
                    search_replace(token.stream(), &mut output, self_ty, self_arg, n_f, t, a);
                    Group::new(token.delimiter(), output)
                });
            }
            TokenTree::Ident(token) => match token.to_string().as_str() {
                "Self" => output.append_all(self_ty(token.span())),
                "self" => output.append_all(self_arg(token.span())),
                "N" => output.append_all(n_f(token.span())),
                "T" => output.append_all(t(token.span())),
                "A" => output.append_all(a(token.span())),
                _ => output.append(token),
            },
            token => output.append(token),
        }
    }
}

fn generic_args(generics: &Generics) -> Vec<TokenStream> {
    generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Const(ConstParam {
                attrs: _,
                const_token,
                ident,
                colon_token,
                ty,
                eq_token: _,
                default: _,
            }) => Some(quote_spanned! { ident.span() => #const_token #ident #colon_token #ty }),
            GenericParam::Lifetime(_) => None,
            GenericParam::Type(param) => Some(param.ident.to_token_stream()),
        })
        .collect()
}
fn arg_ident(arg: &FnArg) -> Ident {
    match arg {
        FnArg::Receiver(_) => Ident::new("self", arg.span()),
        FnArg::Typed(arg) => match &*arg.pat {
            Pat::Ident(pat) => pat.ident.clone(),
            _ => panic!("non-ident arguments are not supported"),
        },
    }
}
