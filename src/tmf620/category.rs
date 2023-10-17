//! Category Module

use crate::CreateTMF;
use crate::HasLastUpdate;
use crate::tmf620::product_offering::ProductOfferingRef;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::LIB_PATH;
use super::MOD_PATH;
use super::HasId;

use crate::CreateTMFWithTime;

const CAT_PATH: &str = "category";
const CAT_VERS: &str = "1.0";

/// Category Resource
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    // Scalar fields
    /// Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HREF where object is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Is this the root of a heirarchy of categories? Default is false.
    pub is_root: bool,
    /// When was this object last updated?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    /// What is the status of this object?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Id of parent in the heirarchy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Version of this record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// How long his this object valid for?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<String>,
    /// Subcategory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<Vec<CategoryRef>>,
    /// Product Offering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<Vec<ProductOfferingRef>>,
}

impl HasLastUpdate for Category {
    fn set_last_update(&mut self, time : String) {
        self.last_update = Some(time);
    }
}

impl CreateTMFWithTime<Category> for Category {}

impl Category {
    /// Create a new instance of the Category struct
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new(String::from("MyCategory"));
    /// ```
    pub fn new(name: String) -> Category {
        let mut cat = Category::create_with_time();
        cat.version = Some(CAT_VERS.to_string());
        cat.name = Some(name);
        cat
    }

    /// Generate a unique id for this object
    pub fn generate_id(&mut self) {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, CAT_PATH, id);
        self.id = Some(id);
        self.href = Some(href);
    } 

    /// Set the description of this category
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new(String::from("MyCategory"))
    ///     .description(String::from("Library of product components"));
    /// ```
    pub fn description(mut self, description: String) -> Category {
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
    pub fn parent(mut self, parent_id: String) -> Category {
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
    pub fn is_root(mut self, root: bool) -> Category {
        self.is_root = root;
        if self.is_root {
            // Remove parent
            self.parent_id = None;
        }
        self
    }
}

impl CreateTMF<Category> for Category {}

impl HasId for Category {
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()
    }

    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()
    }

    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,CAT_PATH,self.get_id());
        self.href = Some(href);    
    }

    fn generate_id(&mut self) {
        // No return type for now
        // Using simple format as SurrealDB doesn't like dashes in standard format.
        let id = Uuid::new_v4().as_simple().to_string(); 
        self.id = Some(id);
        self.generate_href();
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
        assert_eq!(cat.is_root, false);
    }

    #[test]
    fn cat_test_parent() {
        let cat = Category::new(String::from("MyCategory")).parent(String::from("parent_id"));
        assert_eq!(cat.parent_id, Some(String::from("parent_id")));
        assert_eq!(cat.is_root, false);
    }
}

/// Category Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CategoryRef {
    id: Option<String>,
    href: Option<String>,
    name: Option<String>,
    version: Option<String>,
}

impl From<&Category> for CategoryRef {
    fn from(cat: &Category) -> CategoryRef {
        CategoryRef {
            id: cat.id.clone(),
            href: cat.href.clone(),
            name: cat.name.clone(),
            version: cat.version.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tmf620::category::CAT_VERS;

    use super::{Category, CategoryRef};
    #[test]

    fn catref_test_from() {
        let cat = Category::new(String::from("MyCategory"));
        let cat_ref = CategoryRef::from(&cat);

        assert_eq!(cat_ref.name, Some(String::from("MyCategory")));
        assert_eq!(cat_ref.version, Some(CAT_VERS.to_string()));
    }
}
