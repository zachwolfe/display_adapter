use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, Item, Lifetime, FnArg, Type, Ident};
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn display_adapter(attr: TokenStream, item: TokenStream) -> TokenStream {
    assert!(attr.is_empty());
    let item = parse_macro_input!(item as Item);
    let func = match item {
        Item::Fn(func) => func,
        _ => panic!("expected function"),
    };
    let mut attrs = Default::default();
    for attr in func.attrs {
        attr.to_tokens(&mut attrs);
    };
    let vis = func.vis;
    let mut sig = func.sig;
    assert!(sig.generics.lt_token.is_none(), "generics not supported yet");
    let lifetime: Lifetime = parse_quote! { 'a };
    sig.generics = parse_quote! { <#lifetime> };
    let mut w_ident: Option<Ident> = None;
    let mut inputs = Default::default();
    for (input, p) in sig.inputs.pairs().map(|pair| pair.into_tuple()) {
        let mut ident: Option<Ident> = None;
        let mut input = input.clone();
        match input {
            FnArg::Typed(ref mut arg) => {
                if let Type::Reference(ref mut reference) = *arg.ty {
                    if reference.lifetime.is_none() {
                        reference.lifetime = Some(lifetime.clone());
                    }
                    if reference.mutability.is_some() {
                        if let Type::Path(ref path) = *reference.elem {
                            if path.qself.is_none() {
                                let last_segment = path.path.segments.last().unwrap().ident.to_string();
        
                                if last_segment == "Formatter" {
                                    let pat = arg.pat.clone();
                                    ident = Some(parse_quote! { #pat });
                                }
                            }
                        }
                    }
                }
            },
            FnArg::Receiver(ref mut receiver) => {
                let (_, recv_lifetime) = receiver.reference.as_mut().unwrap();
                if recv_lifetime.is_none() {
                    *recv_lifetime = Some(lifetime.clone());
                }
            }
        }

        if let Some(ident) = ident {
            w_ident = Some(ident);
        } else {
            input.to_tokens(&mut inputs);
            p.to_tokens(&mut inputs);
        }
    };
    let w_ident = w_ident.unwrap();
    sig.inputs = parse_quote! { #inputs };
    sig.output = parse_quote! { -> impl std::fmt::Display + #lifetime };
    let block = func.block;
    let expanded = quote! {
        #attrs #vis #sig {
            display_adapter::display_adapter_impl(move |#w_ident| #block)
        }
    };

    TokenStream::from(expanded)
}