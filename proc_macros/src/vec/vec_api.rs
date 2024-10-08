use derive_syn_parse::Parse;
use syn::{token, Ident, Meta};

pub fn vec_api(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        api_ident: Ident,
        #[paren]
        #[peek(token::Paren)]
        tag_paren: Option<token::Paren>,
        #[parse_if(tag_paren.is_some())]
        tag: Option<Meta>,
    }

    todo!(
        "funny funny err! i guess with this there will probably be less errors then before (750 💀)"
    )
}
