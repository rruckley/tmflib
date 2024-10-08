//! Shopping Cart Item

use serde::{Deserialize, Serialize};
use crate::common::note::Note;
use super::shopping_cart::CartPrice;
#[cfg(all(feature = "tmf620", feature = "build-V4"))]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(all(feature = "tmf620", feature = "build-V5"))]
use crate::tmf620::product_offering_v5::ProductOfferingRef;

use std::convert::From;
use uuid::Uuid;

/// Shopping Cart Item
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CartItem {
    id: Option<String>,
    /// Notes for this Cart Item
    pub note : Vec<Note>,
    /// Product Offering in cart
    pub product_offering: Option<ProductOfferingRef>,
    /// Quantity
    pub quantity: u16,
    /// Item Price
    pub item_price: Vec<CartPrice>,
}

impl CartItem {
    /// Add a note to this cart item
    pub fn add_note(&mut self, note : Note) {
        self.note.push(note);
    }
}

impl From<ProductOfferingRef> for CartItem {
    fn from(value: ProductOfferingRef) -> Self {
        let id = Uuid::new_v4().simple().to_string();
        CartItem { 
            id: Some(id), 
            quantity: 1,
            product_offering: Some(value),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::note::Note;
    use crate::tmf620::product_offering::{ProductOffering,ProductOfferingRef};

    use super::CartItem;

    const OFFER_NAME : &str = "OfferName";

    const CART_JSON : &str = "{
        \"id\" : \"CI123\",
        \"note\" : [],
        \"quantity\" : 1,
        \"itemPrice\" : []
    }";
    #[test]
    fn test_cartitem_add_note() {
        let note = Note::from("A Note");

        let mut cart_item = CartItem::default();
        cart_item.add_note(note);

        assert_eq!(cart_item.note.len(),1);
    }

    #[test]
    fn test_cartitem_from_offeringref() {
        let offer = ProductOffering::new(OFFER_NAME);

        let cart_item = CartItem::from(ProductOfferingRef::from(offer));

        assert_eq!(cart_item.quantity,1);
        assert_eq!(cart_item.product_offering.is_some(),true);
        assert_eq!(cart_item.product_offering.unwrap().name,OFFER_NAME.to_string());
    }

    #[test]
    fn test_cartitem_deserialization() {
        let cartitem : CartItem = serde_json::from_str(CART_JSON).unwrap();

        assert_eq!(cartitem.id.is_some(),true);
        assert_eq!(cartitem.quantity,1);
    }
}