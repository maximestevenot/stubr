use std::convert::TryFrom;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemFn, Lit, LitInt, LitStr, NestedMeta};

pub(crate) fn mock_transform(args: AttributeArgs, item: TokenStream) -> syn::Result<TokenStream> {
    let func = syn::parse2::<ItemFn>(item)?;
    let ret = &func.sig.output;
    let name = &func.sig.ident;
    let body = &func.block;
    let attrs = &func.attrs;
    let vis = &func.vis;
    let asyncness = &func.sig.asyncness;
    let args = Args::try_from(args)?;
    let starter = starter(&func, &args);
    Ok(quote! {
        #(#attrs)*
        #vis #asyncness fn #name() #ret {
            #starter
            #body
        }
    })
}

struct Args {
    path: Option<LitStr>,
    full_path: Option<LitStr>,
    port: Option<LitInt>,
}

impl Args {
    const DEFAULT_PATH: &'static str = "tests/stubs";
    const ATTR_FULL_PATH: &'static str = "full_path";
    const ATTR_PORT: &'static str = "port";

    fn path(&self) -> TokenStream {
        self.full_path().unwrap_or_else(|| self.default_path())
    }

    fn full_path(&self) -> Option<TokenStream> {
        self.full_path.as_ref()
            .map(|fp| fp.into_token_stream())
    }

    fn default_path(&self) -> TokenStream {
        self.path.to_owned()
            .map(|p| format!("{}/{}", Self::DEFAULT_PATH, p.value()))
            .map(|p| LitStr::new(p.as_str(), Span::call_site()))
            .unwrap_or_else(|| LitStr::new(Self::DEFAULT_PATH, Span::call_site()))
            .into_token_stream()
    }

    fn port(&self) -> TokenStream {
        self.port.as_ref()
            .map(|p| p.into_token_stream())
            .map(|p| quote! { Some(#p) })
            .unwrap_or_else(|| quote! { None })
    }
}

impl TryFrom<AttributeArgs> for Args {
    type Error = syn::Error;

    fn try_from(input: AttributeArgs) -> Result<Self, Self::Error> {
        let mut path = None;
        let mut full_path = None;
        let mut port = None;
        for arg in input {
            match arg {
                NestedMeta::Lit(Lit::Str(lit)) => path = Some(lit),
                NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    if nv.path.is_ident(Self::ATTR_FULL_PATH) {
                        if let syn::Lit::Str(lit) = nv.lit {
                            full_path = Some(lit)
                        }
                    } else if nv.path.is_ident(Self::ATTR_PORT) {
                        if let syn::Lit::Int(lit) = nv.lit {
                            port = Some(lit)
                        } else {
                            return Err(syn::Error::new_spanned(nv.lit, "Attribute 'port' expects integer"));
                        }
                    }
                }
                _ => {}
            }
        };
        Ok(Self { path, full_path, port })
    }
}

fn starter(func: &ItemFn, args: &Args) -> TokenStream {
    let path = args.path();
    let port = args.port();
    let cfg = quote! { let cfg = stubr::Config { port: #port, ..Default::default() }; };
    if func.sig.asyncness.is_some() {
        quote! {
            #cfg
            let stubr = stubr::Stubr::start_with(#path, cfg).await;
        }
    } else {
        quote! {
            #cfg
            let stubr = stubr::Stubr::start_blocking_with(#path, cfg);
        }
    }
}

#[cfg(test)]
mod mock_tests {
    use super::*;

    mod visibility {
        use syn::Visibility;

        use super::*;

        #[test]
        fn should_conserve_private_visibility() {
            let item = quote! { fn a() {} };
            let transformed = mock_transform(vec![], item).unwrap().into();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(matches!(transformed.vis, Visibility::Inherited))
        }

        #[test]
        fn should_conserve_pub_visibility() {
            let item = quote! { pub fn a() {} };
            let transformed = mock_transform(vec![], item).unwrap().into();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(matches!(transformed.vis, Visibility::Public(_)))
        }
    }

    mod asyncness {
        use super::*;

        #[test]
        fn should_conserve_asyncness() {
            let item = quote! { async fn a() {} };
            let transformed = mock_transform(vec![], item).unwrap().into();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(transformed.sig.asyncness.is_some())
        }

        #[test]
        fn should_not_add_asyncness_when_none() {
            let item = quote! { fn a() {} };
            let transformed = mock_transform(vec![], item).unwrap().into();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(transformed.sig.asyncness.is_none())
        }
    }

    mod name {
        use super::*;

        #[test]
        fn should_conserve_function_name() {
            let item = quote! { fn azerty() {} };
            let transformed = mock_transform(vec![], item).unwrap().into();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert_eq!(transformed.sig.ident.to_string(), String::from("azerty"))
        }
    }

    mod attributes {
        use super::*;

        #[test]
        fn should_conserve_attributes() {
            let item = quote! {
                #[test]
                #[should_panic]
                fn azerty() {}
            };
            let transformed = mock_transform(vec![], item).unwrap().into();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert_eq!(transformed.attrs.len(), 2);
        }
    }

    mod port {
        use syn::{Meta, MetaNameValue, Path, PathSegment};

        use super::*;

        #[test]
        fn should_accept_int_port() {
            let port = Meta::NameValue(MetaNameValue {
                path: Path::from(PathSegment::from(syn::Ident::new("port", Span::call_site()))),
                eq_token: syn::token::Eq([Span::call_site()]),
                lit: Lit::Int(LitInt::new("1234", Span::call_site())),
            });
            let args = vec![NestedMeta::from(port)];
            let transformed = mock_transform(args, quote! { fn a() {} });
            assert!(transformed.is_ok())
        }

        #[test]
        fn should_fail_when_port_not_int() {
            let port = Meta::NameValue(MetaNameValue {
                path: Path::from(PathSegment::from(syn::Ident::new("port", Span::call_site()))),
                eq_token: syn::token::Eq([Span::call_site()]),
                lit: Lit::Str(LitStr::new("abcd", Span::call_site())),
            });
            let args = vec![NestedMeta::from(port)];
            let transformed = mock_transform(args, quote! { fn a() {} });
            assert!(transformed.is_err());
            assert_eq!(transformed.err().unwrap().to_string(), String::from("Attribute 'port' expects integer"))
        }
    }
}