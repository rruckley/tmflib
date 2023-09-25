//! Category Module

use crate::tmf620::product_offering::ProductOfferingRef;

use serde::{Deserialize,Serialize};
use uuid::Uuid;
use chrono::naive::NaiveDateTime;
use chrono::Utc;

use super::LIB_PATH;
use super::MOD_PATH;

const CAT_PATH : &str = "category";
const CAT_VERS : &str = "1.0";

/// Category Resource
#[derive(Debug,Deserialize, Serialize)]
pub struct Category {
    // Scalar fields
    /// Id
    pub id : Option<String>,
    /// HREF where object is located
    pub href : Option<String>,
    /// Description
    description : Option<String>,
    /// Is this the root of a heirarchy of categories?
    pub is_root : bool,
    /// When was this object last updated?
    pub last_update : Option<String>,
    /// What is the status of this object?
    pub lifecycle_status : Option<String>,
    /// Name
    pub name : Option<String>,
    /// Id of parent in the heirarchy
    pub parent_id : Option<String>,
    /// Version of this record
    pub version : Option<String>,
    /// How long his this object valid for?
    pub valid_for : Option<String>,
    /// Subcategory
    pub sub_category : Option<Vec<CategoryRef>>,
    /// Product Offering
    pub product_offering : Option<Vec<ProductOfferingRef>>,
}

impl Category {
    /// Create a new instance of the Category struct
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new(String::from("MyCategory"));
    /// ```
    pub fn new(name : String) -> Category {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,CAT_PATH,id);
        let now = Utc::now();
        let time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        Category { 
            id          : Some(id),
            href        : Some(href),
            description : None,
            is_root     : true,
            last_update : Some(time.to_string()),
            lifecycle_status : None,
            name        : Some(name.clone()),
            parent_id   : None,
            version     : Some(CAT_VERS.to_string()),
            valid_for   : None,
            sub_category: None, 
            product_offering: None
        }
    }

    /// Set the description of this category
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new(String::from("MyCategory"))
    ///     .description(String::from("Library of product components"));
    /// ```
    pub fn description(mut self, description :  String) -> Category {
        self.description = Some(description);
        self
    }

    /// Set parent category
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new(String::from("MyCategory"))
    ///     .parent(String::from("23948-234908"));
    /// ```
    pub fn parent(mut self, parent_id : String) -> Category {
        // Since we are setting a parent, we cannot be root anymore
        self.is_root = false;
        self.parent_id = Some(parent_id);
        self
    }

    /// Set is_root, will remove parent linkage if true.
    /// # Examples
    /// Setting is_root=true will also set parent_id to None.
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new(String::from("MyCategory"))
    ///     .is_root(true);
    /// ```
    pub fn is_root(mut self, root : bool) -> Category {
        self.is_root = root;
        if self.is_root {
            // Remove parent
            self.parent_id = None;
        }
        self
    }

}

#[cfg(test)]
mod tests {
    use super::Category;
    use super::CAT_VERS;
    #[test]
    fn cat_test_name() {
        let cat = Category::new(String::from("MyCategory"));
        assert_eq!(cat.name, Some(String::from("MyCategory")))
    }

    #[test]
    fn cat_test_version() {
        let cat = Category::new(String::from("MyCategory"));
        assert_eq!(cat.version, Some(CAT_VERS.to_string()));
    }

    #[test]
    fn cat_test_root() {
        let cat = Category::new(String::from("MyCategory"));
        assert_eq!(cat.is_root, true);
    }

    #[test]
    fn cat_test_parent() {
        let cat = Category::new(String::from("MyCategory"))
            .parent(String::from("parent_id"));
        assert_eq!(cat.parent_id,Some(String::from("parent_id")));
        assert_eq!(cat.is_root, false);
    }
}

/// Category Reference
#[derive(Clone,Debug,Deserialize, Serialize)]
pub struct CategoryRef {
    id      : Option<String>,
    href    : Option<String>,
    name    : Option<String>,
    version : Option<String>,   
}

impl From<&Category> for CategoryRef {
    fn from(cat : &Category) -> CategoryRef {
        CategoryRef { 
            id  : cat.id.clone(), 
            href: cat.href.clone(), 
            name: cat.name.clone(), 
            version: cat.version.clone() 
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tmf620::category::CAT_VERS;

    use super::{Category,CategoryRef};
    #[test]

    fn catref_test_from() {
        let cat = Category::new(String::from("MyCategory"));
        let cat_ref = CategoryRef::from(&cat);

        assert_eq!(cat_ref.name , Some(String::from("MyCategory")));
        assert_eq!(cat_ref.version, Some(CAT_VERS.to_string()));
    }
}