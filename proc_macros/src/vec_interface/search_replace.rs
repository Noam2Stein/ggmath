use super::*;

pub fn search_replace_fn(
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

pub fn search_replace_assoc_type(
    attrs: TokenStream,
    ident: Ident,
    generics: &Generics,
    type_value: Option<TokenStream>,
    n: impl Fn(Span) -> TokenStream + Copy,
    t: impl Fn(Span) -> TokenStream + Copy,
    a: impl Fn(Span) -> TokenStream + Copy,
) -> TokenStream {
    let mut output = attrs;

    output.append_all(quote_spanned! { ident.span() => type #ident });

    search_replace(
        generics.to_token_stream(),
        &mut output,
        |span| {
            let n = n(span);
            let t = t(span);
            let a = a(span);
            quote_spanned! { span => <Vector<#n, #t, #a>> }
        },
        |span| quote_spanned! { span => vec },
        n,
        t,
        a,
    );

    if let Some(type_value) = type_value {
        output.append_all(quote_spanned! { type_value.span() => = });

        search_replace(
            type_value.to_token_stream(),
            &mut output,
            |span| {
                let n = n(span);
                let t = t(span);
                let a = a(span);
                quote_spanned! { span => <Vector<#n, #t, #a>> }
            },
            |span| quote_spanned! { span => vec },
            n,
            t,
            a,
        );
    }

    output.append_all(quote_spanned! { ident.span() => ; });

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
