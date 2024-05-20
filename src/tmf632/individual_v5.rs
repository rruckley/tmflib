//! Individual Module

use std::ops::Deref;
use chrono::Utc;
use uuid::Uuid;
use sha256::digest;
use hex::decode;
use base32::encode;

use serde::{Deserialize, Serialize};

use crate::{HasId, HasName, CreateTMF,DateTime,TMFEvent,TimePeriod,LIB_PATH};
use tmflib_derive::HasId;
use super::{MOD_PATH,Characteristic};
use crate::common::related_party::RelatedParty;
use crate::common::contact::ContactMedium;
use crate::common::event::{Event, EventPayload};

const CLASS_PATH : &str = "individual";
const CODE_LENGTH : usize = 6;
const CODE_PREFIX : &str = "I-";

/// Language ability of an individual
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LanguageAbility {
    is_favourite_language : bool,
    language_code: String,
    language_name: String,
    listening_proficiency: String,
    reading_proficiency: String,
    speaking_proficiency: String,
    valid_for: TimePeriod,
    writing_proficiency: String,
}

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

    /// Language ability of individual
    #[serde(skip_serializing_if = "Option::is_none")]
    language_ability : Option<LanguageAbility>,
    
    /// Party Characteristics
    #[serde(skip_serializing_if = "Option::is_none")]
    party_characteristic: Option<Vec<Characteristic>>,
}

impl Individual {
    /// Create a new instance of indiviudal object
    pub fn new(name : impl Into<String>) -> Individual {
        let mut ind = Individual::create();
        // Try to split name into two parts
        // If it splits, take 1st as given name, second as family name
        ind.set_name(name);
        // Need this as default would be None
        ind.related_party = Some(vec![]);
        ind.contact_medium = Some(vec![]);
        ind.party_characteristic = Some(vec![]);
        ind.generate_code(None);
        ind
    }

    /// Convenience function to add an email contact medium
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual_v5::Individual;
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
    /// use tmflib::tmf632::individual_v5::Individual;
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
    /// use tmflib::tmf632::individual_v5::Individual;
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
    /// use tmflib::tmf632::individual_v5::Individual;
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
    /// use tmflib::tmf632::individual_v5::Individual;
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
                Some(cm.iter().filter(|i: &&ContactMedium|  {
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

        /// Generate a new site code based on available fields
        pub fn generate_code(&mut self, offset : Option<u32>) {
            let (code,hash) = gen_code(self.get_name(), self.get_id(), offset, Some(CODE_PREFIX.to_string()), None);
            let code_char = Characteristic {
                name : String::from("code"),
                name_type : String::from("String"),
                value : format!("{}{}",CODE_PREFIX,sha_slice),
                ..Default::default()
            };
            let hash_char = Characteristic {
                name : String::from("code"),
                name_type : String::from("String"),
                value : format!("{}{}",CODE_PREFIX,sha_slice),
                ..Default::default()
            };
            self.replace_characteristic(code_char);
            self.replace_characteristic(hash_char);
        }
    
        /// Replace a characteristic returning the old value if found
        pub fn replace_characteristic(&mut self, characteristic : Characteristic) -> Option<Characteristic> {
            match self.party_characteristic.as_mut() {
                Some(c) => {
                    // Characteristic array exist
                    let pos = c.iter().position(|c| c.name == characteristic.name);
                    match pos {
                        Some(u) => {
                            // Clone old value for return
                            let old = c[u].clone();
                            // Replace
                            c[u] = characteristic;
                            Some(old)
                        },
                        None => {
                            // This means the characteristic could not be found, instead we insert it
                            // Additional we return None to indicate that no old value was found
                            c.push(characteristic);
                            None
                        },
                    }
                }
                None => {
                    // Characteristic Vec was not created yet, create it now.
                    self.party_characteristic = Some(vec![characteristic]);
                    // Return None to show no previous value existed.
                    None
                },
            }
        }
}

impl HasName for Individual {
    fn get_name(&self) -> String {
        self.full_name.as_ref().unwrap().clone()
    }
    fn set_name(&mut self, name : impl Into<String>) {
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
                self.given_name = Some(first_name.to_string());
                self.preferred_given_name = self.given_name.clone();
                self.family_name = Some(last_name.to_string());
            },
            3 => {
                // three parts
                let parts_vec : Vec<&str> = name_parts.collect();
                let first_name = parts_vec[0];
                let middle_name = parts_vec[1];
                let last_name = parts_vec[2];
                self.given_name = Some(first_name.to_string());
                self.preferred_given_name = self.given_name.clone();
                self.family_name = Some(last_name.to_string());
                self.middle_name = Some(middle_name.to_string());
            }
            _ => {

            }
        }
        //let (given,family) = name.as_ref().split
        self.full_name = Some(name);
        self.legal_name = self.full_name.clone();
    }
}

/// Individual Event Types
#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum IndividualEventType {
    /// Individual Created
    IndividualCreateEvent,
    /// Individual Attribute Changed
    IndividualAttributeValueChangeEvent,
    /// Individual Status Change
    IndividualStateChangeEvent,
    /// Individual Deleted
    IndividualDeleteEvent,
}

/// Container for Individual events
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct IndividualEvent {
    /// The impacted individual data
    pub individual : Individual,    
}

impl TMFEvent<IndividualEvent> for Individual {
    fn event(&self) -> IndividualEvent {
        IndividualEvent {
            individual : self.clone(),
        }
    }
}

impl EventPayload<IndividualEvent> for Individual {
    type Subject = Individual;
    type EventType = IndividualEventType;

    fn to_event(&self,event_type : Self::EventType) -> Event<IndividualEvent,Self::EventType> {
        let now = Utc::now();
        let desc = format!("{:?} for {} [{}]",event_type,self.get_name(),self.get_id());
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();

        Event {
            correlation_id : None,
            description: Some(desc),
            domain: Some(Individual::get_class()),
            event_id: Uuid::new_v4().to_string(),
            field_path: None,
            href: Some(self.get_href()),
            id: Some(self.get_id()),
            title: Some(self.get_name()),
            event_time: event_time.to_string(),
            priority: None,
            time_occurred: Some(event_time.to_string()),
            event_type,
            event: self.event(),
        }
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