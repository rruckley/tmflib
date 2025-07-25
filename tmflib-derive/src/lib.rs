// Copyright [2025] [Ryan Ruckley]

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Derive Crate for TMFLib traits.

#![warn(missing_docs)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,Data,DeriveInput};

/// Generate code for struct when HasId trait is required. 
/// NB: This trait requires both id and href fields to be present.
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
                match self.id.as_ref() {
                    Some(i) => i.clone(),
                    None => String::default(),
                }
            }
            fn get_href(&self) -> String {
                match self.href.as_ref() {
                    Some(h) => h.clone(),
                    None => String::default(),
                }
            }
            fn get_class() -> String {
                CLASS_PATH.to_string()
            }
            fn get_class_href() -> String {
                format!("/{}/{}/{}",LIB_PATH,MOD_PATH,#name::get_class())
            }
            fn get_mod_path() -> String {
                format!("/{}/{}",LIB_PATH,MOD_PATH)
            }
            fn set_id(&mut self, id : impl Into<String>) {
                self.id = Some(id.into());
                // Since we have changed the Id, the href will be invalid.
                self.generate_href();
            }

            fn id(mut self, id : impl Into<String>) -> Self {
                self.set_id(id);
                self
            }
        }
    };
    out.into()
}

/// Generate functions for HasAttachment Trait
#[proc_macro_derive(HasAttachment)]
pub fn hasattachment_derive(input : TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        Data::Struct(s) => {
            s.fields
                .into_iter()
                .map(|f| f.ident.unwrap().to_string()).collect::<Vec<_>>()
            },
        _ => panic!("HasAttachments only supports Struct"),
    };
    let _last_update = fields.iter().find(|s| *s == "attachment").expect("No attachment field present");
    let name = input.ident;
    let out = quote! {
        impl HasAttachment for #name {
            fn add(&mut self, attachment : &AttachmentRefOrValue) {
                match self.attachment.as_mut() {
                    Some(v) => {
                        v.push(attachment.clone());
                    }
                    None => {
                        self.attachment = Some(vec![attachment.clone()]);
                    }
                }    
            }
            fn position(&self, name : impl Into<String>) -> Option<usize> {
                match self.attachment.as_ref() {
                    Some(v) => {
                        let pattern : String = name.into();
                        v.iter().position(|a| a.name == Some(pattern.clone()))
                    }
                    None => None,
                }
            }
            fn find(&self, name : impl Into<String>) -> Option<&AttachmentRefOrValue> {
                match self.attachment.as_ref() {
                    Some(v) => {
                        let pattern : String = name.into();
                        v.iter().find(|a| a.name == Some(pattern.clone()))               
                    },
                    None => None,
                }
            }
            fn get(&self, position: usize) -> Option<AttachmentRefOrValue> {
                match self.attachment.as_ref() {
                    Some(v) => {
                        v.get(position).cloned()
                    },
                    None => None,
                }    
            }
            fn remove(&mut self, position : usize) -> Option<AttachmentRefOrValue> {
                match self.attachment.as_mut() {
                    Some(v) => {
                        Some(v.remove(position))
                    },
                    None => None,
                }
            }

            fn attachment(mut self, attachment : AttachmentRefOrValue) -> Self {
                self.add(&attachment);
                self
            }
        }
    };
    out.into()
}

/// Generate code for [tmflib::HasLastUpdate] trait.
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

            fn last_update(mut self, time : Option<String>) -> Self {
                match time {
                    Some(t) => self.set_last_update(t),
                    None => self.set_last_update(Self::get_timestamp()),
                };
                self
            }
        }
    };
    out.into()
}

/// Generate code for [tmflib::HasName] trait.
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
            fn set_name(&mut self, name : impl Into<String>) {
                self.name = Some(name.into().trim().to_string());
            }
            fn name(mut self, name :impl Into<String>) -> Self {
                self.set_name(name);
                self
            }
        }
    };
    out.into()
}

/// Generate code for [tmflib::HasNote] trait
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
                match self.note.as_mut() {
                    Some(v) => v.push(note),
                    None => self.note = Some(vec![note]),
                }  
            }
            fn get_note(&self, idx : usize) -> Option<&Note> {
                match self.note.as_ref() {
                    Some(n) => n.get(idx),
                    None => None,
                }
            }
            fn remove_note(&mut self, idx: usize) -> Result<Note,TMFError> {
                match self.note.as_mut() {
                    Some(n) => Ok(n.remove(idx)),
                    None => Err(TMFError::NoDataError("No notes present".to_string())),
                }  
            }
            fn note(mut self, note : Note) -> Self {
                self.add_note(note);
                self
            }
        }
    };
    out.into()
}

/// Generate code for [tmflib::HasRelatedParty] trait.
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
                match self.related_party.as_mut() {
                    Some(v) => v.push(party),
                    None => self.related_party = Some(vec![party]),
                }
            }
            fn get_party(&self, idx : usize ) -> Option<&RelatedParty> {
                match self.related_party.as_ref() {
                    Some(rp) => {
                        // Simple return results of get()
                        rp.get(idx)
                    },
                    None => None,
                }
                  
            }
            fn remove_party(&mut self, idx : usize) -> Result<RelatedParty,TMFError> {
                match self.related_party.as_mut() {
                    None => Err(TMFError::NoDataError("No related parties present".to_string())),
                    Some(rp) => {
                        Ok(rp.remove(idx))
                    }
                }
            }
            fn get_by_role(&self, role : String) -> Option<Vec<&RelatedParty>> {
                match &self.related_party {
                    Some(rp) => {
                        let out = rp.iter()
                            .filter(|p| p.role.is_some())
                            .filter(|p| p.role.clone().unwrap() == role)
                            .collect();
                        Some(out)
                    },
                    None => None,
                }    
            }

            fn party(mut self, party : RelatedParty) -> Self {
                self.add_party(party);
                self
            }
        }
    };
    out.into()
}

/// Implement the HasDescription Trait
#[proc_macro_derive(HasDescription)]
pub fn hasdescription_derive(input: TokenStream) -> TokenStream {
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
    let _name = fields.iter().find(|s| *s == "description").expect("No description field found");    

    let out = quote! {
        impl HasDescription for #name {
            fn description(mut self, description : impl Into<String>) -> Self {
                self.description = Some(description.into());
                self
            }
            fn get_description(&self) -> String {
                match self.description.as_ref() {
                    Some(d) => d.clone(),
                    None => String::default(),
                }    
            }
            fn set_description(&mut self, description : impl Into<String>) -> Option<String> {
                self.description.replace(description.into())
            }
        }
    };
    out.into()
}


/// Generate code for HasValidity trait.
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
            fn is_valid(&self) -> bool {
                let validity = self.get_validity();
                match validity {
                    Some(v) => {
                        if v.started() && !v.finished()  {
                            return true
                        }
                        false
                    },
                    None => false
                }
            }

            fn validity(mut self, validity : TimePeriod) -> Self {
                self.set_validity(validity);
                self
            }
        }
    };
    out.into()   
}