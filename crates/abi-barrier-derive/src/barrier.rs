use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr, ItemFn, Path, Result, Token,
    parse::{Parse, ParseStream},
};

/// Arguments passed to `#[barrier(...)]`.
pub struct BarrierArgs {
    pub convention: Option<Path>,
    pub on_panic: Option<Expr>,
}

impl Parse for BarrierArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut convention = None;
        let mut on_panic = None;

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if key == "convention" {
                if convention.is_some() {
                    return Err(syn::Error::new(
                        key.span(),
                        "duplicate `convention` attribute",
                    ));
                }
                convention = Some(input.parse::<Path>()?);
            } else if key == "on_panic" {
                if on_panic.is_some() {
                    return Err(syn::Error::new(
                        key.span(),
                        "duplicate `on_panic` attribute",
                    ));
                }
                on_panic = Some(input.parse::<Expr>()?);
            } else {
                return Err(syn::Error::new(
                    key.span(),
                    format!("unknown attribute `{key}`, expected `convention` or `on_panic`"),
                ));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            convention,
            on_panic,
        })
    }
}

pub fn expand_barrier(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let args: BarrierArgs = syn::parse2(attr)?;
    let item_fn: ItemFn = syn::parse2(item)?;

    let fn_vis = &item_fn.vis;
    let fn_sig = &item_fn.sig;
    let fn_ident = &fn_sig.ident;
    let fn_args = &fn_sig.inputs;
    let fn_output = &fn_sig.output;
    let fn_body = &item_fn.block;
    let fn_attrs = &item_fn.attrs;
    let fn_unsafety = &fn_sig.unsafety;

    let fn_name_str = fn_ident.to_string();

    let core_path = quote! { ::abi_barrier::__core };

    let on_panic_expr = if let Some(expr) = &args.on_panic {
        quote! { #expr }
    } else {
        quote! { #core_path::PanicStrategy::ReturnDefault }
    };

    // Determine ABI string for `extern "..."`
    let abi_str = if let Some(path) = &args.convention {
        let last_segment = path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();
        match last_segment.as_str() {
            "C" => "C",
            "Cdecl" => "cdecl",
            "Stdcall" => "stdcall",
            "Fastcall" => "fastcall",
            "System" => "system",
            _ => "C",
        }
    } else {
        "C"
    };

    let inner_fn_ident = syn::Ident::new(
        &format!("__inner_impl_{}", fn_ident),
        proc_macro2::Span::call_site(),
    );

    let mut arg_names = Vec::new();
    for input in fn_args {
        match input {
            syn::FnArg::Receiver(_) => {
                return Err(syn::Error::new_spanned(
                    input,
                    "#[barrier] cannot be used on methods with `self` receiver",
                ));
            }
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let ident = &pat_ident.ident;
                    arg_names.push(quote! { #ident });
                } else {
                    return Err(syn::Error::new_spanned(
                        pat_type,
                        "#[barrier] only supports simple parameter identifiers",
                    ));
                }
            }
        }
    }

    let inner_call = if fn_unsafety.is_some() {
        quote! {
            // SAFETY: Outer FFI barrier entry point delegates to inner unsafe function
            unsafe { #inner_fn_ident(#(#arg_names),*) }
        }
    } else {
        quote! {
            #inner_fn_ident(#(#arg_names),*)
        }
    };

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_unsafety extern #abi_str fn #fn_ident(#fn_args) #fn_output {
            #[inline(always)]
            #fn_unsafety fn #inner_fn_ident(#fn_args) #fn_output {
                #fn_body
            }

            let __strategy = #on_panic_expr;
            match #core_path::catch_panic_barrier(#fn_name_str, __strategy, move || {
                #inner_call
            }) {
                Ok(val) => val,
                Err(err) => {
                    #core_path::AbiReturn::from_panic_error(__strategy, err)
                }
            }
        }
    };

    Ok(expanded)
}
