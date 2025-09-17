use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

#[proc_macro_derive(HasSpan)]
pub fn derive_has_span(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    match input.data {
        Data::Union(_) => quote_spanned! {
            input.span() =>
            compile_error!("expected a struct or enum, unions are not allowed!");
        },

        Data::Enum(ref data) => {
            let mut variants = Vec::new();

            for item in &data.variants {
                variants.push(item.ident.clone());
            }

            let name = input.ident.clone();
            let mut matchers = Vec::new();
            let mut matchers_consume = Vec::new();

            for item in &variants {
                matchers.push(quote! {
                    Self::#item { span, .. } => span.clone(),
                });

                matchers_consume.push(quote! {
                    Self::#item { span, .. } => span,
                });
            }

            quote! {
                impl crate::common::traits::HasSpan for #name {
                    fn span(&self) -> miette::SourceSpan {
                        match self {
                            #(#matchers)*
                        }
                    }

                    fn into_span(self) -> miette::SourceSpan {
                        match self {
                            #(#matchers_consume)*
                        }
                    }
                }
            }
        },

        Data::Struct(ref data) => {
            let mut found = false;

            for field in &data.fields {
                if let Some(id) = &field.ident {
                    if id == "span" {
                        found = true;
                    }
                }
            }

            if !found {
                quote_spanned! {
                    input.span() =>
                    compile_error("missing field 'span' in struct!");
                }
            } else {
                let name = input.ident.clone();

                quote! {
                    impl crate::common::traits::HasSpan for #name {
                        fn span(&self) -> miette::SourceSpan {
                            self.span.clone()
                        }

                        fn into_span(self) -> miette::SourceSpan {
                            self.span
                        }
                    }
                }
            }
        }
    }.into()
}

#[proc_macro_derive(HasSpanGroup)]
pub fn derive_has_span_group(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    match input.data {
        Data::Struct(_) | Data::Union(_) => quote_spanned! {
            input.span() =>
            compile_error!("expected an enum, structs and unions are not allowed!");
        },

        Data::Enum(ref data) => {
            let mut variants = Vec::new();

            for item in &data.variants {
                variants.push(item.ident.clone());
            }

            let name = input.ident.clone();
            let mut matchers = Vec::new();
            let mut matchers_consume = Vec::new();

            for item in &variants {
                matchers.push(quote! {
                    Self::#item(it) => it.span(),
                });

                matchers_consume.push(quote! {
                    Self::#item(it) => it.into_span(),
                });
            }

            quote! {
                impl crate::common::traits::HasSpan for #name {
                    fn span(&self) -> miette::SourceSpan {
                        match self {
                            #(#matchers)*
                        }
                    }

                    fn into_span(self) -> miette::SourceSpan {
                        match self {
                            #(#matchers_consume)*
                        }
                    }
                }
            }
        }
    }.into()
}
