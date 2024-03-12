//! Individual Module

use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{HasId, HasName, CreateTMF, DateTime};
use tmflib_derive::HasId;
use crate::LIB_PATH;
use super::MOD_PATH;
use crate::common::related_party::RelatedParty;
use crate::common::contact::ContactMedium;

const CLASS_PATH : &str = "individual";

/// An individual
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
    /// Unique id for this individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML reference for this individual object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Aristocratic Title, e.g. Lord, Lady, Count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aristocratic_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date of Birth
    pub birth_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    death_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family_name_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted_name: Option<String>,
    /// Full name of the individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    /// Given Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// Legal Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// Location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Marital Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    /// Middle Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// Nationality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// Birth Place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<String>,
    /// Preferred Given Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_given_name: Option<String>,
    /// Title, e.g.Mr, Mrs etc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Methods for contacting this individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
  
    /// Parties related to this individual, e.g. company / organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl Individual {
    /// Create a new instance of indiviudal object
    pub fn new(name : impl Into<String>) -> Individual {
        let mut ind = Individual::create();
        // Try to split name into two parts
        // If it splits, take 1st as given name, second as family name
        let name : String = name.into();
        let name_str = name.as_str();
        let name_parts = name_str.split(' ');
        // Determine the number of parts the name is given in
        match name_parts.clone().count() {
            1 => {
                // Only a single name, nothing to do here.

            },
            2 => {
                // two parts
                let parts_vec : Vec<&str> = name_parts.collect();
                let first_name = parts_vec[0];
                let last_name = parts_vec[1];
                ind.given_name = Some(first_name.to_string());
                ind.preferred_given_name = ind.given_name.clone();
                ind.family_name = Some(last_name.to_string());
            },
            3 => {
                // three parts
                let parts_vec : Vec<&str> = name_parts.collect();
                let first_name = parts_vec[0];
                let middle_name = parts_vec[1];
                let last_name = parts_vec[2];
                ind.given_name = Some(first_name.to_string());
                ind.preferred_given_name = ind.given_name.clone();
                ind.family_name = Some(last_name.to_string());
                ind.middle_name = Some(middle_name.to_string());
            }
            _ => {

            }
        }
        //let (given,family) = name.as_ref().split
        ind.full_name = Some(name);
        ind.legal_name = ind.full_name.clone();
        // Need this as default would be None
        ind.related_party = Some(vec![]);
        ind.contact_medium = Some(vec![]);
        ind
    }

    /// Convenience function to add an email contact medium
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual::Individual;
    /// 
    /// let individual = Individual::new("John Smith")
    ///     .email("john.smith@example.com");
    /// ```
    pub fn email(mut self, email : &str) -> Individual {
        let medium = ContactMedium::email(email);
        self.add_contact(medium);
        self
    }

    /// Convenience function to set the title for an individual
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual::Individual;
    /// 
    /// let individual = Individual::new("John Smith")
    ///     .title("Mr");
    /// ```
    pub fn title(mut self, title : impl Into<String>) -> Individual {
        self.title = Some(title.into());
        self
    }

    /// Convenience function to set the gender for an individual
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual::Individual;
    /// 
    /// let individual = Individual::new("John Smith")
    ///     .gender("Unspecified");
    /// ```
    pub fn gender(mut self, gender: impl Into<String>) -> Individual {
        self.gender = Some(gender.into());
        self
    }

    /// Convenience function to set the preferred given name for an individual
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual::Individual;
    /// 
    /// let individual = Individual::new("John Smith")
    ///     .gender("Unspecified");
    /// ```
    pub fn preferred(mut self, preferred: impl Into<String>) -> Individual {
        self.preferred_given_name = Some(preferred.into());
        self
    }

    /// Convenience funciton to add a mobile number contact medium
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual::Individual;
    /// 
    /// let individual = Individual::new("John Smith")
    ///     .mobile("0411 111 111");
    /// ```
    pub fn mobile(mut self, mobile: &str) -> Individual {
        let medium = ContactMedium::mobile(mobile);
        self.add_contact(medium);
        self
    }

    /// Add a related party to the individual
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.as_mut().unwrap().push(party);
    }

    /// Add a contact medium to the individual
    pub fn add_contact(&mut self, medium : ContactMedium) {
        self.contact_medium.as_mut().unwrap().push(medium);
    }

    /// Find a particular contact medium matching ``medium``
    fn find_medium(&self, medium : impl Into<String>) -> Option<Vec<&ContactMedium>> {
        match &self.contact_medium {
            None => None,
            Some(cm) => {
                let name : String = medium.into();
                Some(cm.into_iter().filter(|i: &&ContactMedium|  {
                    i.medium_type.is_some() && i.medium_type.as_ref().unwrap().deref() == name
                }).collect())
            }
        }
        
    }

    /// Get Email address from contact medium if present
    pub fn get_mobile(&self) -> Option<String> {
        // Optionally get the email address
        let medium = self.find_medium("mobile")?;
        let medium = medium.first()?;
        let characteristic = medium.characteristic.as_ref()?;
        characteristic.email_address.clone()
    }

    /// Get Email address from contact medium if present
    pub fn get_email(&self) -> Option<String> {
        // Optionally get the email address
        let medium = self.find_medium("email")?;
        let medium = medium.first()?;
        let characteristic = medium.characteristic.as_ref()?;
        characteristic.email_address.clone()
    }
}

impl HasName for Individual {
    fn get_name(&self) -> String {
        self.full_name.as_ref().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::Individual;
    #[test]
    fn test_individual_create_id() {
        let ind = Individual::new("APerson");

        assert_eq!(ind.id.is_some(),true);
    }

    #[test]
    fn test_individual_create_href() {
        let ind = Individual::new("APerson");

        assert_eq!(ind.href.is_some(),true);
    }

    #[test]
    fn test_individual_single_name() {
        let ind = Individual::new("Madonna");

        assert_eq!(ind.full_name.is_some(),true);
        assert_eq!(ind.full_name,Some("Madonna".to_string()));
    }

    #[test]
    fn test_individual_two_names() {
        let ind = Individual::new("John Smith");

        assert_eq!(ind.full_name.is_some(),true);
        assert_eq!(ind.full_name,Some("John Smith".to_string()));
        assert_eq!(ind.given_name,Some("John".to_string()));
        assert_eq!(ind.family_name,Some("Smith".to_string()));
    }

    #[test]
    fn test_individual_three_names() {
        let ind = Individual::new("John Bagford Smith");

        assert_eq!(ind.full_name.is_some(),true);
        assert_eq!(ind.full_name,Some("John Bagford Smith".to_string()));
        assert_eq!(ind.given_name,Some("John".to_string()));
        assert_eq!(ind.middle_name,Some("Bagford".to_string()));
        assert_eq!(ind.family_name,Some("Smith".to_string()));
    }

    #[test]
    fn test_individual_get_email() {
        const EMAIL : &str = "john.bagford.smith@example.com";
        let ind = Individual::new("John Bagford Smith")
            .email(EMAIL);

        assert_eq!(ind.get_email(),Some(EMAIL.to_string())) 
    }
}