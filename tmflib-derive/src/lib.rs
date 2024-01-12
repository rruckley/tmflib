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
    let _id = fields.iter().find(|s| *s == "id").expect("No id field present");
    let _href = fields.iter().find(|s| *s == "href").expect("No href field present");
    // Generate HasId impl block based on this name.

    let out = quote! {

        impl tmflib::HasId for #name {
            fn generate_id(&mut self) {
                let id = #name::get_uuid();
                self.id = id.into();
                self.generate_href();
            }
            fn generate_href(&mut self) {
                let href = format!("{}/{}",#name::get_class_href(),self.get_id());
                self.href = href.into();
            }
            fn get_id(&self) -> String {
                self.id.as_ref().unwrap().clone()
            }
            fn get_href(&self) -> String {
                self.href.as_ref().unwrap().clone()
            }
            fn get_class() -> String {
                CLASS_PATH.to_string()
            }
            fn get_class_href() -> String {
                format!("{}/{}/{}",LIB_PATH,MOD_PATH,#name::get_class())
            }
        }
    };
    out.into()
}

