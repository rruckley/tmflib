use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,Data,DeriveInput};

#[proc_macro_derive(HasId)]
pub fn hasid_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        Data::Struct(s) => {
            s.fields
                .into_iter()
                .map(|f| f.ident.unwrap().to_string()).collect::<Vec<_>>()
            },
        _ => panic!("HasId only supports Struct"),
    };
    let name = input.ident;
    // Ensure id field is present
    let id = fields.iter().find(|s| *s == "id").expect("No id field present");
    let href = fields.iter().find(|s| *s == "href").expect("No href field present");
    // Generate HasId impl block based on this name.

    let out = quote! {

        impl tmflib::HasId for #name {
            fn generate_id(&mut self) {
                let id = #name::get_uuid();
                self.id = id;
            }
            /// Generate a new HTML reference.
            /// # Details
            /// This is usually triggered directly from generate_id() but can be manually triggered.
            fn generate_href(&mut self) {
                let href = format!("/{}",CLASS_PATH);
                self.href = href;
            }
            /// Extract the id of this object into a new String
            fn get_id(&self) -> String {
                
                "id".to_string()
            }
            /// Extract the HREF of this object into a new String
            fn get_href(&self) -> String {
                self.href.clone()
            }
            /// Get the class of this object
            fn get_class() -> String {
                "class".to_string()
            }
            /// Get Class HREF
            fn get_class_href() -> String {
                "class_href".to_string()
            }
        }
    };
    out.into()
}

