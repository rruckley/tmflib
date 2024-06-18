//! Product Order Module

use serde::{Deserialize, Serialize};

use tmflib_derive::{HasId,HasNote,HasRelatedParty};
use crate::tmf641::service_order::ServiceOrder;
use crate::common::related_party::RelatedParty;
use crate::common::note::Note;
use crate::tmf651::agreement::AgreementRef;
use crate::{DateTime, HasId, HasLastUpdate, HasNote, HasRelatedParty};
use crate::tmf663::shopping_cart::ShoppingCart;
use super::product_order_item::ProductOrderItem;

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;



const CLASS_PATH: &str = "productOrder";

/// ProductOrder
#[derive(Clone, Debug, Default, Deserialize, HasId, HasNote, HasRelatedParty, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    /// Cancellation Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cancellation Reason
    pub cancellation_reason: Option<String>,
    /// Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category : Option<String>,
    /// Completion Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<DateTime>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Expected Completion Date
    pub expected_completion_date : Option<DateTime>,
    /// External Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id : Option<String>,
    /// Order Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_date: Option<DateTime>,
    /// Product Order Items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_order_item: Option<Vec<ProductOrderItem>>,
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    // Referenced objects
    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note : Option<Vec<Note>>,
    /// Agreements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement : Option<Vec<AgreementRef>>,
}

impl HasLastUpdate for ProductOrder {
    fn set_last_update(&mut self, time : impl Into<String>) {
        self.order_date = Some(time.into());
    }
}

impl ProductOrder {
    /// Create a new product order via trait
    pub fn new() -> ProductOrder {
        let mut po = ProductOrder::create_with_time();
        po.related_party = Some(vec![]);
        po.product_order_item = Some(vec![]);
        po
    }

    /// Add an ProductOrderItem into the ProductOrder
    pub fn add_order_item(&mut self, order_item : ProductOrderItem) {
        self.product_order_item.as_mut().unwrap().push(order_item);
    }

    /// Add a RelatedParty into the ProductOrder
    /// # Example
    /// ```
    /// # use tmflib::tmf622::product_order_v4::ProductOrder;
    /// use tmflib::common::related_party::RelatedParty;
    /// use tmflib::tmf629::customer::Customer;
    /// use tmflib::tmf632::organization_v4::Organization;
    /// 
    /// let organization = Organization::new(String::from("My Customer"));
    /// let customer = Customer::new(organization);
    /// let mut order = ProductOrder::new();
    /// order.add_party(RelatedParty::from(&customer));
    /// dbg!(order);
    /// ```
    pub fn add_party(&mut self, party: RelatedParty) {
        self.related_party.as_mut().unwrap().push(party);
    }
}

impl From<ServiceOrder> for ProductOrder {
    fn from(value: ServiceOrder) -> Self {
        let mut po = ProductOrder::new();
        
        po.cancellation_reason.clone_from(&value.cancellation_reason);
        po.category.clone_from(&value.category);
        po.description.clone_from(&value.description);
        po.external_id.clone_from(&value.external_id);
        po.note.clone_from(&value.note);
        po.related_party.clone_from(&value.related_party);
        
        // Dates
        po.completion_date.clone_from(&value.completion_date);
        po.order_date.clone_from(&value.order_date);
        po.cancellation_date.clone_from(&value.cancellation_date);
        po.expected_completion_date.clone_from(&value.expected_completion_date);
        
        // Iterate through service order items
        let items = match value.servce_order_item {
            Some(i) => {
                let mut out = vec![];
                i.into_iter().for_each(|i| {
                    // Conert i into ProductOrderItem
                    let poi = ProductOrderItem::from(i);
                    out.push(poi);
                });
                Some(out)
            },
            None => None,
        };
        po.product_order_item = items;

        po  
    }
}

impl From<ShoppingCart> for ProductOrder {
    fn from(value: ShoppingCart) -> Self {
        // Convert a Shopping cart into a product order.
        // Each CartItem converts into an order item using a conversion function.
        let mut order = ProductOrder::new();
        order.description = Some("Order from Cart".into());
        // Bring across the cart items
        if value.cart_item.is_some() {
            value.cart_item.unwrap().into_iter().for_each(|i| {
                order.product_order_item.as_mut().unwrap().push(ProductOrderItem::from(i));
            });
        }
        // Bring across the related parties
        if value.related_party.is_some() {
            value.related_party.unwrap().into_iter().for_each(|rp| {
                order.related_party.as_mut().unwrap().push(rp.clone());
            });
        }
        order
    }
}