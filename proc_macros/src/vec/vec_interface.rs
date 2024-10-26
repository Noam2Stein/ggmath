use super::*;

use array_ext::ArrayExt;
use syn::{
    punctuated::Punctuated,
    token::{Brace, Bracket},
    Block, ConstParam, FnArg, GenericParam, Generics, Lit, LitInt, Pat, Receiver, Signature,
    TypeParam, Visibility,
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
        use crate::vec::{inner::*, *};

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
    scalar_trait: TypeParam,
    vis: Visibility,
    ident: Ident,
    fns: Vec<VecInterfaceFn>,
    errors: Vec<Error>,
}
impl Parse for VecInterface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let scalar_trait = TypeParam::parse(input)?;
        <Token![,]>::parse(input)?;

        let vis = Visibility::parse(input)?;
        let ident = Ident::parse(input)?;
        <Token![:]>::parse(input)?;

        let mut fns = Vec::new();
        let mut errors = Vec::new();
        while !input.is_empty() {
            while !input.peek(Token![fn])
                && !input.peek(Token![unsafe])
                && !input.peek(Token![async])
            {
                errors.push(Error::new(
                    input.parse::<TokenTree>()?.span(),
                    "expected Signature",
                ));
            }

            match input.parse() {
                Ok(ok) => fns.push(ok),
                Err(err) => errors.push(err),
            }
        }

        Ok(Self {
            scalar_trait,
            vis,
            ident,
            fns,
            errors,
        })
    }
}

struct VecInterfaceFn {
    sig: Signature,
    defaults: [[Block; 3]; 2],
}
impl Parse for VecInterfaceFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            sig: input.parse()?,
            defaults: evaluate_fn_defaults(input.parse()?)?,
        })
    }
}
fn evaluate_fn_match<const N: usize>(
    input: VecInterfaceFnMatch,
    expected_keys: [&str; N],
) -> Result<[VecInterfaceFnTree; N], Error> {
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

    #[allow(unstable_name_collisions)]
    values.zip(expected_keys).try_map(|(value, key)| {
        value.map_or_else(
            || {
                Err(Error::new(
                    input._brackets1.span.close(),
                    format!("'{key}' not covered"),
                ))
            },
            |value| Ok(value),
        )
    })
}
fn evaluate_fn_defaults(input: VecInterfaceFnTree) -> Result<[[Block; 3]; 2], Error> {
    handle_fn_defaults(
        input,
        |input| {
            Ok([
                [input.clone(), input.clone(), input.clone()],
                [input.clone(), input.clone(), input],
            ])
        },
        |input| {
            #[allow(unstable_name_collisions)]
            let [[aligned2, packed2], [aligned3, packed3], [aligned4, packed4]] =
                evaluate_fn_match(input, LEN_STRS)?.try_map(evaluate_fn_defaults_for_n)?;

            Ok([[aligned2, aligned3, aligned4], [packed2, packed3, packed4]])
        },
        |input| {
            #[allow(unstable_name_collisions)]
            evaluate_fn_match(input, ALIGN_STRS)?.try_map(evaluate_fn_defaults_for_a)
        },
    )
}
fn evaluate_fn_defaults_for_n(input: VecInterfaceFnTree) -> Result<[Block; 2], Error> {
    handle_fn_defaults(
        input,
        |input| Ok([input.clone(), input]),
        |input| Err(Error::new(input.item.span(), "'N' already matched")),
        |input| {
            #[allow(unstable_name_collisions)]
            evaluate_fn_match(input, ALIGN_STRS)?.try_map(evaluate_fn_defaults_for_n_a)
        },
    )
}
fn evaluate_fn_defaults_for_a(input: VecInterfaceFnTree) -> Result<[Block; 3], Error> {
    handle_fn_defaults(
        input,
        |input| Ok([input.clone(), input.clone(), input]),
        |input| {
            #[allow(unstable_name_collisions)]
            evaluate_fn_match(input, LEN_STRS)?.try_map(evaluate_fn_defaults_for_n_a)
        },
        |input| Err(Error::new(input.item.span(), "'A' already matched")),
    )
}
fn evaluate_fn_defaults_for_n_a(input: VecInterfaceFnTree) -> Result<Block, Error> {
    handle_fn_defaults(
        input,
        |input| Ok(input),
        |input| Err(Error::new(input.item.span(), "'N' already matched")),
        |input| Err(Error::new(input.item.span(), "'A' already matched")),
    )
}
fn handle_fn_defaults<Output>(
    input: VecInterfaceFnTree,
    handle_single: impl FnOnce(Block) -> Result<Output, Error>,
    handle_match_n: impl FnOnce(VecInterfaceFnMatch) -> Result<Output, Error>,
    handle_match_a: impl FnOnce(VecInterfaceFnMatch) -> Result<Output, Error>,
) -> Result<Output, Error> {
    match input {
        VecInterfaceFnTree::Single(input) => handle_single(input),
        VecInterfaceFnTree::Match(input) => match input.item.to_string().as_str() {
            "N" => handle_match_n(input),
            "A" => handle_match_a(input),
            _ => Err(Error::new(input.item.span(), "expected either 'N' or 'A'")),
        },
    }
}

#[derive(Parse, Clone)]
enum VecInterfaceFnTree {
    #[peek(Brace, name = "Single")]
    Single(Block),
    #[peek(Bracket, name = "Match")]
    Match(VecInterfaceFnMatch),
}
#[derive(Parse, Clone)]
struct VecInterfaceFnMatch {
    #[bracket]
    _brackets0: Bracket,
    #[inside(_brackets0)]
    _match_token: Token![match],
    #[inside(_brackets0)]
    item: Ident,
    #[inside(_brackets0)]
    #[bracket]
    _brackets1: Bracket,
    #[inside(_brackets1)]
    #[call(Self::parse_pats)]
    pats: Vec<VecInterfaceFnMatchPat>,
}
impl VecInterfaceFnMatch {
    fn parse_pats(input: ParseStream) -> syn::Result<Vec<VecInterfaceFnMatchPat>> {
        let mut output = Vec::new();
        while !input.is_empty() {
            output.push(input.parse()?);
            input.parse::<Token![,]>()?;
        }

        Ok(output)
    }
}
#[derive(Parse, Clone)]
struct VecInterfaceFnMatchPat {
    #[call(Punctuated::parse_separated_nonempty)]
    keys: Punctuated<TokenTree, Token![|]>,
    #[prefix(Token![=>])]
    value: VecInterfaceFnTree,
}

fn impl_block(input: &VecInterface) -> TokenStream {
    let scalar_trait = &input.scalar_trait.ident;
    let len_trait = len_trait_ident(input);
    let vis = &input.vis;

    let fn_impls = input.fns.iter().map(|r#fn| {
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
        impl<const N: usize, T: #scalar_trait, A: VecAlignment> Vector<N, T, A> where ScalarCount<N>: VecLen<N> {
            #(
                #fn_impls
            )*
        }
    }
}
fn scalar(input: &VecInterface) -> TokenStream {
    let trait_attrs = &input.scalar_trait.attrs;
    let trait_ident = &input.scalar_trait.ident;
    let trait_supertraits = &input.scalar_trait.bounds;

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
        pub trait #trait_ident: #trait_supertraits {
            #(
                #fn_declarations
            )*
        }
    }
}
fn len(input: &VecInterface) -> TokenStream {
    let scalar_trait = &input.scalar_trait.ident;
    let len_trait = len_trait_ident(input);
    let alignment_trait = alignment_trait_ident(input);

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
                #(
                    #[allow(unused_mut)]
                    #fn_impls
                )*
            }
        }
    });

    quote_spanned! {
        len_trait.span() =>
        pub(super) trait #len_trait<const N: usize>: VecLenInnerVec where ScalarCount<N>: VecLen<N> {
            #(
                #fn_declarations
            )*
        }
        #(
            #impls
        )*
    }
}
fn alignment(input: &VecInterface) -> TokenStream {
    let scalar_trait = &input.scalar_trait.ident;
    let alignment_trait = alignment_trait_ident(input);

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

                let fn_impls = fn_sigs.iter().map(|sig| {
                    let scalar_fn = scalar_fn_ident(&sig.ident, n_str, a_str);

                    let n = LitInt::new(n_str, sig.ident.span());

                    let generics = &generic_args(&sig.generics)[1..];
                    let input_idents = sig.inputs.iter().map(|input| arg_ident(input));

                    search_replace_fn(
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
                    impl #alignment_trait<#n> for #a {
                        #(
                            #[allow(unused_mut)]
                            #fn_impls
                        )*
                    }
                }
            })
        })
        .into_iter()
        .flatten();

    quote_spanned! {
        alignment_trait.span() =>
        pub(super) trait #alignment_trait<const N: usize>: seal::VecAlignment where ScalarCount<N>: VecLen<N> {
            #(
                #fn_declarations
            )*
        }
        #(
            #impls
        )*
    }
}

fn search_replace_fn(
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

    let mut output = TokenStream::new();
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
