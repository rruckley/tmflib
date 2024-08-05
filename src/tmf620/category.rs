//! Category Module

#[cfg(feature = "v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;

use serde::{Deserialize, Serialize};

use super::MOD_PATH;
use crate::{
    HasId,
    HasName,
    HasLastUpdate,
    DateTime,
    HasValidity,
    TimePeriod,
    LIB_PATH,
    Uri,
};
use tmflib_derive::{HasId,HasLastUpdate,HasName,HasValidity};

const CLASS_PATH: &str = "category";
const CAT_VERS: &str = "1.0";

/// Category Resource
#[derive(Clone, Default, Debug, Deserialize, HasId, HasLastUpdate, HasName, HasValidity, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    /// When was this object last updated?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<DateTime>,
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
    pub valid_for: Option<TimePeriod>,
    /// Subcategory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<Vec<CategoryRef>>,
    /// Product Offering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<Vec<ProductOfferingRef>>,
    
    // META
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    base_type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    schema_location: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    r#type : Option<String>,
}

impl Category {
    /// Create a new instance of the Category struct
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new(String::from("MyCategory"));
    /// ```
    pub fn new(name: impl Into<String>) -> Category {
        let mut cat = Category::create_with_time();
        cat.version = Some(CAT_VERS.to_string());
        cat.name = Some(name.into());
        cat.is_root = Some(false);
        cat
    }

    /// Is this a root category
    pub fn root(&self) -> bool {
        // Extract is_root in a safe manner
        self.is_root.unwrap_or(false)
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
        self.is_root = Some(false);
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
        self.is_root = Some(root);
        if self.is_root.unwrap() {
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
    use super::CLASS_PATH;
    use crate::HasId;

    const DESC: &str = "A Description";
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
        assert_eq!(cat.is_root.unwrap(), false);
    }

    #[test]
    fn cat_test_parent() {
        let cat = Category::new(String::from("MyCategory")).parent(String::from("parent_id"));
        assert_eq!(cat.parent_id, Some(String::from("parent_id")));
        assert_eq!(cat.is_root.unwrap(), false);
    }

    #[test]
    fn cat_test_class() {
        
        assert_eq!(Category::get_class(),CLASS_PATH);
    }

    #[test]
    fn test_cat_root() {
        let cat = Category::new("A Category");

        assert_eq!(cat.root(),false);
    }

    #[test]
    fn test_cat_description() {
        let cat = Category::new("A Category")
            .description(DESC.to_string());

        assert_eq!(cat.description.unwrap(),DESC.to_string());

    }

    #[test]
    fn test_cat_set_parent_root() {
        let parent = Category::new("Parent");
        let child = Category::new("Child")
            // Make root=true
            .is_root(true)
            // Should make root=false
            .parent(parent.get_id());
            

        // Child should no longer be root
        assert_eq!(child.is_root.unwrap(),false);
    }
}

/// Category Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CategoryRef {
    id: Option<String>,
    href: Option<String>,
    name: Option<String>,
    version: Option<String>,
    // META
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    base_type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    schema_location: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    r#type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@referredType")]
    referred_type : Option<String>,
}

impl From<&Category> for CategoryRef {
    fn from(cat: &Category) -> CategoryRef {
        CategoryRef {
            id: cat.id.clone(),
            href: cat.href.clone(),
            name: cat.name.clone(),
            version: cat.version.clone(),
            base_type : Some(Category::get_class()),
            r#type : Some(Category::get_class()),
            schema_location: None,
            referred_type: Some(Category::get_class()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{tmf620::category::CAT_VERS, HasName, HasValidity, TimePeriod};
    use super::{Category, CategoryRef};

    const CAT_NAME : &str = "CategoryName";

    const CAT_JSON : &str = "{
        \"name\" : \"CategoryName\"
    }";

    const CATREF_JSON : &str = "{
        \"name\" : \"CategoryName\",
        \"version\" : \"1.0\"
    }";

    #[test]
    fn catref_test_from() {
        let cat = Category::new(String::from("MyCategory"));
        let cat_ref = CategoryRef::from(&cat);

        assert_eq!(cat_ref.name, Some(String::from("MyCategory")));
        assert_eq!(cat_ref.version, Some(CAT_VERS.to_string()));
    }

    #[test]
    fn cat_hasname() {
        let cat = Category::new(CAT_NAME);

        assert_eq!(cat.get_name().as_str(),CAT_NAME);
    }

    #[test]
    fn cat_deserialize() {
        let cat : Category = serde_json::from_str(CAT_JSON).unwrap();

        assert_eq!(cat.name.is_some(),true);
        assert_eq!(cat.get_name().as_str(),"CategoryName");
    }

    #[test]
    fn cat_hasvalidity() {
        let mut cat = Category::new(CAT_NAME);

        cat.set_validity(TimePeriod::period_30days());

        assert_eq!(cat.valid_for.is_some(),true);
        assert_eq!(cat.get_validity().unwrap().started(),true);
        assert_eq!(cat.get_validity().unwrap().finished(),false);
        assert_eq!(cat.get_validity_start().is_some(),true);
        assert_eq!(cat.get_validity_end().is_some(),true);
    }

    #[test]
    fn catref_deserialize() {
        let catref : CategoryRef = serde_json::from_str(CATREF_JSON).unwrap();

        assert_eq!(catref.name.is_some(),true);
        assert_eq!(catref.name.unwrap().as_str(),"CategoryName");
    }
}
