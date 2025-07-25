//! Individual Module

use std::ops::Deref;
use chrono::Utc;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::{
    HasId, 
    HasName, 
    HasRelatedParty,
    HasReference,
    DateTime,
    TMFEvent,
    gen_code,
};
use tmflib_derive::{
    HasId,
    HasRelatedParty
};
use crate::LIB_PATH;
use super::{
    MOD_PATH,
    Characteristic
};
use crate::common::related_party::RelatedParty;
use crate::common::contact::ContactMedium;
use crate::common::event::{Event, EventPayload};
use crate::common::tmf_error::TMFError;

const CLASS_PATH : &str = "individual";
const CODE_PREFIX : &str = "I-";
const NAMENOTSET : &str = "NAMENOTSET";

/// An individual
#[derive(Clone, Debug, Default, Deserialize, HasId, HasRelatedParty, Serialize)]
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

    /// Party Characteristics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_characteristic: Option<Vec<Characteristic>>,
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
        ind.generate_code(None);
        ind
    }

    /// Convenience function to add an email contact medium
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual_v4::Individual;
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
    /// use tmflib::tmf632::individual_v4::Individual;
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
    /// use tmflib::tmf632::individual_v4::Individual;
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
    /// use tmflib::tmf632::individual_v4::Individual;
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
    /// use tmflib::tmf632::individual_v4::Individual;
    /// 
    /// let individual = Individual::new("John Smith")
    ///     .mobile("0411 111 111");
    /// ```
    pub fn mobile(mut self, mobile: &str) -> Individual {
        let medium = ContactMedium::mobile(mobile);
        self.add_contact(medium);
        self
    }

    /// Add a contact medium to the individual
    pub fn add_contact(&mut self, medium : ContactMedium) {
        match self.contact_medium.as_mut() {
            Some(v) => v.push(medium),
            None => self.contact_medium = Some(vec![medium]),
        }
    }

    /// Find a particular contact medium matching [`medium`]
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

    /// Get Mobile number from contact medium if present
    pub fn get_mobile(&self) -> Option<String> {
        // Optionally get the email address
        let medium = self.find_medium("mobile")?;
        let medium = medium.first()?;
        let characteristic = medium.characteristic.as_ref()?;
        characteristic.phone_number.clone()
    }

    /// Get Email address from contact medium if present
    pub fn get_email(&self) -> Option<String> {
        // Optionally get the email address
        let medium = self.find_medium("email")?;
        let medium = medium.first()?;
        let characteristic = medium.characteristic.as_ref()?;
        characteristic.email_address.clone()
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

    /// Generate a new site code based on available fields
    pub fn generate_code(&mut self, offset : Option<u32>) {
        let (code,hash) = gen_code(self.get_name(), self.get_id(), offset, Some(CODE_PREFIX.to_string()), None);
        let code_char = Characteristic {
            name : String::from("code"),
            name_type : String::from("String"),
            value : code,
            ..Default::default()
        };
        let hash_char = Characteristic {
            name : String::from("hash"),
            name_type : String::from("String"),
            value : hash,
            ..Default::default()
        };
        self.replace_characteristic(code_char);
        self.replace_characteristic(hash_char);
    }
}

impl HasName for Individual {
    fn get_name(&self) -> String {
        match self.full_name.as_ref() {
            Some(f) => f.clone(),
            None => String::from(NAMENOTSET),
        }
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
                self.preferred_given_name.clone_from(&self.given_name);
                self.family_name = Some(last_name.to_string());
            },
            3 => {
                // three parts
                let parts_vec : Vec<&str> = name_parts.collect();
                let first_name = parts_vec[0];
                let middle_name = parts_vec[1];
                let last_name = parts_vec[2];
                self.given_name = Some(first_name.to_string());
                self.preferred_given_name.clone_from(&self.given_name);
                self.family_name = Some(last_name.to_string());
                self.middle_name = Some(middle_name.to_string());
            }
            _ => {

            }
        }
        //let (given,family) = name.as_ref().split
        self.full_name = Some(name);
        self.legal_name.clone_from(&self.full_name);
    }
    
    fn name(mut self, name : impl Into<String>) -> Self {
        self.set_name(name);
        self
    }
}


impl HasReference for Individual {
    type RefType = RelatedParty;
    fn as_ref(&self) -> Option<Self::RefType> {
        Some(RelatedParty::from(self))
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
    use crate::HasReference;

    const IND_TITLE: &str = "A Title";
    const IND_GENDER: &str = "A Gender";
    const IND_PREF: &str = "A Preferred Name";
    const IND_MOBILE: &str = "0411 111 111";

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

    #[test]
    fn test_individual_title() {
        let ind = Individual::new("John Bagford Smith")
            .title(IND_TITLE);
    
        assert_eq!(ind.title.unwrap(),IND_TITLE.to_string());
    }

    #[test]
    fn test_individual_gender() {
        let ind = Individual::new("John Bagford Smith")
            .gender(IND_GENDER);

        assert_eq!(ind.gender.unwrap(),IND_GENDER.to_string());
    }

    #[test]
    fn test_individual_preferred() {
        let ind = Individual::new("John Bagford Smith")
            .preferred(IND_PREF);

        assert_eq!(ind.preferred_given_name.unwrap(),IND_PREF.to_string());
    }

    #[test]
    fn test_individual_find_medium() {
        let ind = Individual::new("John Bagford Smith");

        let medium = ind.find_medium("medium");

        assert_eq!(medium.is_none(),true);
    }

    #[test]
    fn test_individual_mobile() {
        let ind = Individual::new("John Bagford Smith")
            .mobile(IND_MOBILE);

        // assert_eq!(ind.get_mobile(),Some(IND_MOBILE.to_string()));
        assert_eq!(ind.contact_medium.is_some(),true);
        assert_eq!(ind.get_mobile(),Some(IND_MOBILE.to_string()));
    }

    #[test]
    fn test_individual_get_mobile() {
        let ind = Individual::new("John Bagford Smith")
        .mobile(IND_MOBILE);
        let mobile = ind.get_mobile();

        assert_eq!(mobile.is_some(),true);    
        assert_eq!(mobile.unwrap(),IND_MOBILE.to_string());
    }

    #[test]
    fn test_individual_asref() {
        use crate::HasName;
        let ind = Individual::new("John Bagford Smith")
            .mobile(IND_MOBILE);
        let ref_ind = ind.as_ref();

        assert_eq!(ref_ind.is_some(),true);
        assert_eq!(ref_ind.unwrap().name.unwrap(),ind.get_name());
        // assert_eq!(ref_ind.unwrap().id,ind.get_id());
    }
}