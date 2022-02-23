extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnableDocument)]
pub fn enable_document_macro_fn(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_enable_document(&ast);
    gen.parse().unwrap()
}

fn impl_enable_document(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl EnableDocument for #name {
            fn get_document(&self) -> Option<String> {
                return self.docu.clone();
            }
            fn set_document(&mut self, document : Option<String>) {
                self.docu = document;
            }
        }
    }
}