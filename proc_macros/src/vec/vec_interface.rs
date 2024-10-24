use std::mem::transmute_copy;

use super::*;

use array_ext::ArrayExt;
use syn::{
    punctuated::Punctuated,
    token::{Brace, Bracket},
    Block, FnArg, Lit, LitInt, Pat, Signature, TypeParam, Visibility,
};

pub fn vec_interface(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let errors = input.fns.errors.iter().map(|err| err.to_compile_error());
    let impl_block = vec(&input);
    let scalar = scalar(&input);
    let len = len(&input);
    let storage = alignment(&input);

    quote_spanned! {
        input.interface_ident.span() =>
        use crate::vec::api::prelude::*;

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

#[derive(Parse)]
struct Input {
    scalar_trait: TypeParam,
    #[prefix(Token![,])]
    fns_vis: Visibility,
    interface_ident: Ident,
    #[prefix(Token![:])]
    fns: InputFns,
}
struct InputFns {
    fns: Vec<InputFn>,
    errors: Vec<Error>,
}
impl Parse for InputFns {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fns = Vec::new();
        let mut errors = Vec::new();
        while !input.is_empty() {
            match input.parse() {
                Ok(ok) => fns.push(ok),
                Err(err) => errors.push(err),
            }
        }

        Ok(Self { fns, errors })
    }
}
#[derive(Parse)]
struct InputFn {
    sig: Signature,
    #[call(Self::parse_defaults)]
    defaults: [[Block; 3]; 2],
}
impl InputFn {
    fn parse_defaults(input: ParseStream) -> syn::Result<[[Block; 3]; 2]> {
        fn resolve_match<const N: usize>(
            input: InputFnDefaultMatch,
            expected_keys: [&str; N],
        ) -> Result<[InputFnDefault; N], Error> {
            let mut matches = [(); N].map(|_| None);

            for input in input.pats {
                for key in input.keys {
                    let key_str = key.to_string();
                    if let Some(key_index) = expected_keys
                        .into_iter()
                        .position(|expected_key| expected_key == key_str)
                    {
                        let r#match = &mut matches[key_index];
                        if r#match.is_some() {
                            *r#match = Some(input.value.clone());
                        } else {
                            return Err(Error::new(
                                key.span(),
                                format!("'{key_str}' already covered"),
                            ));
                        }
                    } else {
                        return Err(Error::new(
                            key.span(),
                            format!("unexpected key '{key_str}'"),
                        ));
                    }
                }
            }

            #[allow(unstable_name_collisions)]
            matches.zip(expected_keys).try_map(|(value, key)| {
                if let Some(value) = value {
                    Ok(value)
                } else {
                    Err(Error::new(
                        input._brackets1.span.close(),
                        format!("'{key}' not covered"),
                    ))
                }
            })
        }
        fn resolve_top_match(input: InputFnDefaultMatch) -> Result<[[Block; 3]; 2], Error> {
            match input.item.to_string().as_str() {
                "N" => {
                    #[allow(unstable_name_collisions)]
                    let blocks =
                        resolve_match(input, ["2", "3", "4"])?.try_map(
                            |default| match default {
                                InputFnDefault::Single(default) => Ok([default.clone(), default]),
                                InputFnDefault::Match(default) => resolve_inside_n_match(default),
                            },
                        )?;

                    unsafe {
                        Ok([
                            [
                                transmute_copy(&blocks[0][0]),
                                transmute_copy(&blocks[1][0]),
                                transmute_copy(&blocks[2][0]),
                            ],
                            [
                                transmute_copy(&blocks[0][1]),
                                transmute_copy(&blocks[1][1]),
                                transmute_copy(&blocks[2][1]),
                            ],
                        ])
                    }
                }
                "A" =>
                {
                    #[allow(unstable_name_collisions)]
                    resolve_match(input, ["VecAligned", "VecPacked"])?.try_map(|default| {
                        match default {
                            InputFnDefault::Single(default) => {
                                Ok([default.clone(), default.clone(), default])
                            }
                            InputFnDefault::Match(default) => resolve_inside_a_match(default),
                        }
                    })
                }
                _ => Err(Error::new(input.item.span(), "expected either 'N' or 'A'")),
            }
        }
        fn resolve_inside_n_match(input: InputFnDefaultMatch) -> Result<[Block; 2], Error> {
            match input.item.to_string().as_str() {
                "N" => Err(Error::new(input.item.span(), "'N' already matched")),
                "A" =>
                {
                    #[allow(unstable_name_collisions)]
                    resolve_match(input, ["VecAligned", "VecPacked"])?.try_map(|default| {
                        match default {
                            InputFnDefault::Single(default) => Ok(default),
                            InputFnDefault::Match(default) => {
                                match default.item.to_string().as_str() {
                                    "N" => {
                                        Err(Error::new(default.item.span(), "'N' already matched"))
                                    }
                                    "A" => {
                                        Err(Error::new(default.item.span(), "'A' already matched"))
                                    }
                                    _ => Err(Error::new(
                                        default.item.span(),
                                        "expected either 'N' or 'A'",
                                    )),
                                }
                            }
                        }
                    })
                }
                _ => Err(Error::new(input.item.span(), "expected either 'N' or 'A'")),
            }
        }
        fn resolve_inside_a_match(input: InputFnDefaultMatch) -> Result<[Block; 3], Error> {
            match input.item.to_string().as_str() {
                "N" =>
                {
                    #[allow(unstable_name_collisions)]
                    resolve_match(input, ["2", "3", "4"])?.try_map(|default| match default {
                        InputFnDefault::Single(default) => Ok(default),
                        InputFnDefault::Match(default) => match default.item.to_string().as_str() {
                            "N" => Err(Error::new(default.item.span(), "'N' already matched")),
                            "A" => Err(Error::new(default.item.span(), "'A' already matched")),
                            _ => Err(Error::new(
                                default.item.span(),
                                "expected either 'N' or 'A'",
                            )),
                        },
                    })
                }
                "A" => Err(Error::new(input.item.span(), "'A' already matched")),
                _ => Err(Error::new(input.item.span(), "expected either 'N' or 'A'")),
            }
        }

        let input = input.parse::<InputFnDefault>()?;
        match input {
            InputFnDefault::Single(input) => Ok([
                [input.clone(), input.clone(), input.clone()],
                [input.clone(), input.clone(), input.clone()],
            ]),
            InputFnDefault::Match(input) => resolve_top_match(input),
        }
    }
}

#[derive(Parse, Clone)]
enum InputFnDefault {
    #[peek(Brace, name = "Single")]
    Single(Block),
    #[peek(Bracket, name = "Match")]
    Match(InputFnDefaultMatch),
}
#[derive(Parse, Clone)]
struct InputFnDefaultMatch {
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
    pats: Vec<InputFnDefaultMatchPat>,
}
impl InputFnDefaultMatch {
    fn parse_pats(input: ParseStream) -> syn::Result<Vec<InputFnDefaultMatchPat>> {
        let mut output = Vec::new();
        while !input.is_empty() {
            output.push(input.parse()?);
        }

        Ok(output)
    }
}
#[derive(Parse, Clone)]
struct InputFnDefaultMatchPat {
    #[call(Punctuated::parse_separated_nonempty)]
    keys: Punctuated<TokenTree, Token![|]>,
    #[prefix(Token![=>])]
    value: InputFnDefault,
}

fn vec(input: &Input) -> TokenStream {
    let scalar_trait_ident = &input.scalar_trait.ident;
    let len_trait_ident = len_trait_ident(input);

    let impl_fns = input.fns.fns.iter().map(|r#fn| {
        let mut output = TokenStream::new();
        output.append_all(quote_spanned! { r#fn.sig.fn_token.span() => #[inline(always)] });
        input.fns_vis.to_tokens(&mut output);
        r#fn.sig.to_tokens(&mut output);

        let fn_ident = &r#fn.sig.ident;
        let generics = &r#fn.sig.generics.params;
        let input_idents = r#fn.sig.inputs.iter().map(|input| arg_ident(input));

        quote_spanned! { fn_ident.span() => {
            <ScalarCount<N> as #len_trait_ident<N>>::#fn_ident::<T, A, #generics>(#(#input_idents), *)
        }}
    });

    quote_spanned! {
        input.interface_ident.span() =>
        impl<const N: usize, T: #scalar_trait_ident, A: VecAlignment> Vector<N, T, A> where ScalarCount<N>: VecLen<N> {
            #(
                #impl_fns
            )*
        }
    }
}
fn scalar(input: &Input) -> TokenStream {
    let trait_attrs = &input.scalar_trait.attrs;
    let trait_ident = &input.scalar_trait.ident;
    let trait_supertraits = &input.scalar_trait.bounds;

    let fns = input
        .fns
        .fns
        .iter()
        .map(|r#fn| {
            ["VecAligned", "VecPacked"]
                .zip(r#fn.defaults.clone())
                .map(|(a, defaults)| {
                    ["2", "3", "4"].zip(defaults).map(|(n, default)| {
                        let mut sig = r#fn.sig.clone();
                        sig.ident = scalar_fn_ident(&sig.ident, n, a);

                        let n = LitInt::new(n, r#fn.sig.span());
                        let a = Ident::new(a, r#fn.sig.span());

                        let mut output = TokenStream::new();
                        push_modified_tokens(
                            sig.to_token_stream(),
                            &mut output,
                            |span| quote_spanned! { span => <Vector<#n, Self, #a>> },
                            |span| quote_spanned! { span => vec: Vector<#n, Self, #a> },
                            |span| quote_spanned! { span => #n },
                            |span| quote_spanned! { span => Self },
                            |span| quote_spanned! { span => #a },
                        );
                        push_modified_tokens(
                            default.to_token_stream(),
                            &mut output,
                            |span| quote_spanned! { span => <Vector<#n, Self, #a>> },
                            |span| quote_spanned! { span => vec },
                            |span| quote_spanned! { span => #n },
                            |span| quote_spanned! { span => Self },
                            |span| quote_spanned! { span => #a },
                        );

                        output
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
                #fns
            )*
        }
    }
}
fn len(input: &Input) -> TokenStream {
    let trait_ident = len_trait_ident(input);
    let scalar_trait_ident = &input.scalar_trait.ident;
    let alignment_trait_ident = alignment_trait_ident(input);

    let fn_sigs = input
        .fns
        .fns
        .iter()
        .map(|r#fn| {
            let mut sig = r#fn.sig.clone();
            sig.generics.params.insert(
                0,
                parse_quote_spanned! { sig.ident.span() => T: #scalar_trait_ident },
            );
            sig.generics.params.insert(
                1,
                parse_quote_spanned! { sig.ident.span() => A: VecAlignment },
            );

            sig
        })
        .collect::<Box<[Signature]>>();

    let fns = fn_sigs.iter().map(|sig| {
        let mut output = TokenStream::new();
        push_modified_tokens(
            sig.to_token_stream(),
            &mut output,
            |span| quote_spanned! { span => <Vector<N, T, A>> },
            |span| quote_spanned! { span => vec: Vector<N, T, A> },
            |span| quote_spanned! { span => N },
            |span| quote_spanned! { span => T },
            |span| quote_spanned! { span => A },
        );
        output.extend(quote_spanned! { sig.fn_token.span() => ; });

        output
    });

    let impls = ["2", "3", "4"].map(|n| {
        let fns = fn_sigs.iter().map(|sig| {
            let n = Lit::Int(LitInt::new(n, sig.ident.span()));

            let fn_ident = &sig.ident;
            let generics = &sig.generics.params;
            let input_idents = sig.inputs.iter().map(|input| arg_ident(input));

            let mut output = TokenStream::new();
            push_modified_tokens(
                sig.to_token_stream(),
                &mut output,
                |span| quote_spanned! { span => <Vector<#n, T, A>> },
                |span| quote_spanned! { span => vec: Vector<#n, T, A> },
                |span| quote_spanned! { span => #n },
                |span| quote_spanned! { span => T },
                |span| quote_spanned! { span => A },
            );
            push_modified_tokens(
                quote_spanned! { sig.ident.span() => {
                    <A as #alignment_trait_ident>::#fn_ident::<T, #generics>(#(#input_idents), *)
                } },
                &mut output,
                |span| quote_spanned! { span => <Vector<#n, T, A>> },
                |span| quote_spanned! { span => vec: Vector<#n, T, A> },
                |span| quote_spanned! { span => #n },
                |span| quote_spanned! { span => T },
                |span| quote_spanned! { span => A },
            );

            output
        });

        quote_spanned! {
            input.interface_ident.span() =>
            impl #trait_ident<#n> for ScalarCount<#n> {
                #(
                    #fns
                )*
            }
        }
    });

    quote_spanned! {
        trait_ident.span() =>
        pub(super) trait #trait_ident<const N: usize>: inner::VecLenInnerVec where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
        #(
            #impls
        )*
    }
}
fn alignment(input: &Input) -> TokenStream {
    let trait_ident = alignment_trait_ident(input);
    let scalar_trait_ident = &input.scalar_trait.ident;

    let fn_sigs = input
        .fns
        .fns
        .iter()
        .map(|r#fn| {
            let mut sig = r#fn.sig.clone();
            sig.generics.params.insert(
                0,
                parse_quote_spanned! { sig.ident.span() => T: #scalar_trait_ident },
            );

            sig
        })
        .collect::<Box<[Signature]>>();

    let fns = fn_sigs.iter().map(|sig| {
        let mut output = TokenStream::new();
        push_modified_tokens(
            sig.to_token_stream(),
            &mut output,
            |span| quote_spanned! { span => <Vector<N, T, Self>> },
            |span| quote_spanned! { span => vec: Vector<N, T, Self> },
            |span| quote_spanned! { span => N },
            |span| quote_spanned! { span => T },
            |span| quote_spanned! { span => Self },
        );
        output.extend(quote_spanned! { sig.fn_token.span() => ; });

        output
    });

    let impls = ["2", "3", "4"]
        .map(|n| {
            ["VecAligned", "VecPacked"].map(|a| {
                let fns = fn_sigs.iter().map(|sig| {
                    let scalar_fn_ident = scalar_fn_ident(&sig.ident, n, a);

                    let n = Lit::Int(LitInt::new(n, sig.ident.span()));

                    let generics = &sig.generics.params;
                    let input_idents = sig.inputs.iter().map(|input| arg_ident(input));

                    let mut output = TokenStream::new();
                    push_modified_tokens(
                        sig.to_token_stream(),
                        &mut output,
                        |span| quote_spanned! { span => <Vector<#n, T, Self>> },
                        |span| quote_spanned! { span => vec: Vector<#n, T, Self> },
                        |span| quote_spanned! { span => #n },
                        |span| quote_spanned! { span => T },
                        |span| quote_spanned! { span => Self },
                    );
                    push_modified_tokens(
                        quote_spanned! { sig.ident.span() => {
                            T::#scalar_fn_ident::<#generics>(#(#input_idents), *)
                        } },
                        &mut output,
                        |span| quote_spanned! { span => <Vector<#n, T, Self>> },
                        |span| quote_spanned! { span => vec: Vector<#n, T, Self> },
                        |span| quote_spanned! { span => #n },
                        |span| quote_spanned! { span => T },
                        |span| quote_spanned! { span => Self },
                    );

                    output
                });

                quote_spanned! {
                    input.interface_ident.span() =>
                    impl #trait_ident<#n> for #a {
                        #(
                            #fns
                        )*
                    }
                }
            })
        })
        .into_iter()
        .flatten();

    quote_spanned! {
        trait_ident.span() =>
        pub(super) trait #trait_ident<const N: usize>: inner::VecAlignmentInnerVecs where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
        #(
            #impls
        )*
    }
}

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
fn len_trait_ident(input: &Input) -> Ident {
    Ident::new(
        &format!("VecLen{}", input.interface_ident),
        input.interface_ident.span(),
    )
}
fn alignment_trait_ident(input: &Input) -> Ident {
    Ident::new(
        &format!("VecAlignment{}", input.interface_ident),
        input.interface_ident.span(),
    )
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

fn push_modified_tokens(
    tokens: TokenStream,
    output: &mut TokenStream,
    self_ty: impl Fn(Span) -> TokenStream + Copy,
    self_arg: impl Fn(Span) -> TokenStream + Copy,
    n: impl Fn(Span) -> TokenStream + Copy,
    t: impl Fn(Span) -> TokenStream + Copy,
    a: impl Fn(Span) -> TokenStream + Copy,
) {
    for token in tokens {
        match token {
            TokenTree::Group(token) => {
                push_modified_tokens(token.stream(), output, self_ty, self_arg, n, t, a);
            }
            TokenTree::Ident(token) => match token.to_string().as_str() {
                "Self" => output.extend((self_ty)(token.span())),
                "self" => output.extend((self_arg)(token.span())),
                "n" => output.extend((n)(token.span())),
                "t" => output.extend((t)(token.span())),
                "a" => output.extend((a)(token.span())),
                _ => output.extend(TokenTree::Ident(token)),
            },
            TokenTree::Literal(_) => output.extend(token),
            TokenTree::Punct(_) => output.append(token),
        }
    }
}
