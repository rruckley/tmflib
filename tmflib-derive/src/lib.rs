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

        impl HasId for #name {
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
                format!("/{}/{}/{}",LIB_PATH,MOD_PATH,#name::get_class())
            }
        }

        impl CreateTMF<#name> for #name {}
    };
    out.into()
}

#[proc_macro_derive(HasLastUpdate)]
pub fn haslastupdate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        Data::Struct(s) => {
            s.fields
                .into_iter()
                .map(|f| f.ident.unwrap().to_string()).collect::<Vec<_>>()
            },
        _ => panic!("HasId only supports Struct"),
    };
    let _last_update = fields.iter().find(|s| *s == "last_update").expect("No last_update field present");
    let name = input.ident;
    let out = quote! {
        impl HasLastUpdate for #name {
            fn set_last_update(&mut self, time : impl Into<String>) {
                self.last_update = Some(time.into());
            }
        }

        impl CreateTMFWithTime<#name> for #name {}
    };
    out.into()
}

#[proc_macro_derive(HasName)]
pub fn hasname_derive(input: TokenStream) -> TokenStream {
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
    let _name = fields.iter().find(|s| *s == "name").expect("No name field present");
    let out = quote! {
        impl HasName for #name {
            fn get_name(&self) -> String {
                self.name.clone().unwrap_or("NoName".to_string())
            }
        }
    };
    out.into()
}

#[proc_macro_derive(HasNote)]
pub fn hasnote_derive(input: TokenStream) -> TokenStream {
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
    let _note = fields.iter().find(|s| *s == "note").expect("No note field present");
    let out = quote! {
        impl HasNote for #name {
            fn add_note(&mut self, note : Note) {
                self.note.as_mut().unwrap().push(note);    
            }
            fn get_note(&self, idx : usize) -> Option<&Note> {
                self.note.as_ref().unwrap().get(idx)
            }
            fn remove_note(&mut self, idx: usize) -> Result<Note,String> {
                Ok(self.note.as_mut().unwrap().remove(idx))  
            }
        }
    };
    out.into()
}

#[proc_macro_derive(HasRelatedParty)]
pub fn hasrelatedparty_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        Data::Struct(s) => {
            s.fields
                .into_iter()
                .map(|f| f.ident.unwrap().to_string()).collect::<Vec<_>>()
            },
        _ => panic!("HasRelatedParty only supports Struct"),
    };
    let name = input.ident;
    // Ensure id field is present
    let _related_party: &String = fields.iter().find(|s| *s == "related_party").expect("No related_party field present");
    let out = quote! {   
        impl HasRelatedParty for #name {
            fn add_party(&mut self, party : RelatedParty) {
                self.related_party.as_mut().unwrap().push(party);
            }
            fn get_party(&self, idx : usize ) -> Option<&RelatedParty> {
                self.related_party.as_ref().unwrap().get(idx)    
            }
            fn remove_party(&mut self, idx : usize) -> Result<RelatedParty,String> {
                Ok(self.related_party.as_mut().unwrap().remove(idx))  
            }
        }
    };
    out.into()
}

#[proc_macro_derive(HasValidity)]
pub fn hasvalidity_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        Data::Struct(s) => {
            s.fields
                .into_iter()
                .map(|f| f.ident.unwrap().to_string()).collect::<Vec<_>>()
            },
        _ => panic!("HasValidity only supports Struct"),
    };
    let name = input.ident;
    // Ensure id field is present
    let _name = fields.iter().find(|s| *s == "valid_for").expect("No valid_for object present");
    let out = quote! {
        impl HasValidity for #name {
            fn get_validity(&self) -> Option<TimePeriod> {
                self.valid_for.clone()
            }
            fn get_validity_end(&self) -> Option<crate::TimeStamp> {
                match self.valid_for.as_ref() {
                    Some(v) => v.end_date_time.clone(),
                    None => None,
                }        
            }
            fn get_validity_start(&self) -> Option<crate::TimeStamp> {
                match self.valid_for.as_ref() {
                    Some(v) => Some(v.start_date_time.clone()),
                    None => None,
                }
            }
            fn set_validity(&mut self, validity : TimePeriod) {
                self.valid_for = Some(validity);
            }
            fn set_validity_end(&mut self, end : crate::TimeStamp) -> TimePeriod {
                let mut validity = match self.get_validity() {
                    Some(v) => v,
                    None => TimePeriod::default(),
                };
                validity.end_date_time = Some(end);
                self.set_validity(validity.clone());
                validity
            }
            fn set_validity_start(&mut self, start : crate::TimeStamp) -> TimePeriod {
                let mut validity = match self.get_validity() {
                    Some(v) => v,
                    None => TimePeriod::default(),
                };
                validity.start_date_time = start;
                self.set_validity(validity.clone());
                validity
            }
    
        }
    };
    out.into()   
}