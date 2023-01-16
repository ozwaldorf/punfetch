use proc_macro::TokenStream;

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Error, Fields, LitStr, Result, Type, TypePath,
};

/// Derive a Render implementation for a struct.
#[proc_macro_derive(Render)]
pub fn rule_system_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as _);
    TokenStream::from(match impl_render(ast) {
        Ok(it) => it,
        Err(err) => err.to_compile_error(),
    })
}

fn impl_render(ast: DeriveInput) -> Result<TokenStream2> {
    Ok({
        let name = ast.ident;
        let fields = match ast.data {
            Data::Struct(DataStruct {
                fields: Fields::Named(it),
                ..
            }) => it,
            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    "Expected a `struct` with named fields",
                ));
            }
        };

        let data_expanded_members = fields.named.into_iter().map(|field| {
            let field_name = field.ident.expect("Unreachable");
            let span = field_name.span();

            // build the field name
            // first char uppercase, rest lowercase with _ replaced with space
            let name = field_name.to_string();
            let display_name = LitStr::new(&name.chars().enumerate().map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().to_string()
                } else if c == '_' {
                    " ".to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            }).collect::<String>(), span);

            match field.ty {
                Type::Path(TypePath { path, .. }) => {
                    match path {
                        path if path.is_ident("String") => {
                            quote_spanned! {span=>
                                buf.push(format!("{}: {}", #display_name.bold().color(color), self.#field_name));
                            }
                        }
                        path if path.segments.last().unwrap().ident == "Option" => {
                            quote_spanned! {span=>
                                if let Some(inner) = self.#field_name.as_ref() {
                                    buf.push(format!("{}: {}", #display_name.bold().color(color), inner));
                                }
                            }
                        }
                        _ => quote!(),
                    }
                }
                _ => unimplemented!(),
            }
        });
        quote! {
            impl Render for #name {
                fn render(self: &'_ Self, color: owo_colors::DynColors) -> Vec<String> {
                    let mut buf = Vec::new();
                    #(#data_expanded_members)*
                    buf
                }
            }
        }
    })
}
