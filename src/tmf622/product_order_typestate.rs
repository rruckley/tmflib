//! TMF622 Wrapper to implement TypeState pattern

use std::marker::PhantomData;
use crate::{HasLastUpdate, HasNote};
use crate::common::note::Note;

use super::product_order_v4::ProductOrder;

type OrderState = super::product_order_v4::ProductOrderStateType;

pub struct Draft;
pub struct Acknowledged;
pub struct InProgress;
pub struct Completed;

#[derive(Debug)]
pub struct Cancelled;

pub trait StateMarker {
    const VALUE : OrderState;
}

impl StateMarker for Draft {
    const VALUE : OrderState = OrderState::Draft;
}
impl StateMarker for Acknowledged {
    const VALUE : OrderState = OrderState::Acknowledged;
}
impl StateMarker for InProgress {
    const VALUE : OrderState = OrderState::InProgress;
}
impl StateMarker for Completed {
    const VALUE : OrderState = OrderState::Completed;
}
impl StateMarker for Cancelled {
    const VALUE : OrderState = OrderState::Cancelled;
}       

#[derive(Debug)]
pub struct Order<S> {
    data: ProductOrder,
    _state: PhantomData<S>,
}

impl Order<Draft> {
    /// Create a new order in the Draft state.
    pub fn new() -> Self {
        let mut data = ProductOrder::create_with_time();
        data.state = Some(OrderState::Draft);
        data.add_note(Note::from("Draft order created"));
        Order {        
            data,
            _state: PhantomData,
        }
    }

    /// Acknowledge the order and transition to the Acknowledged state.
    pub fn acknowledge(self) -> Order<Acknowledged> {
        let mut data = self.data;
        data.state = Some(OrderState::Acknowledged);
        data.add_note(Note::from("Order Acknowledged"));
        Order {
            data,
            _state: PhantomData,
        }
    }
}

impl Order<Acknowledged> {
    pub fn start(self) -> Order<InProgress> {
        let mut data = self.data;
        data.state = Some(OrderState::InProgress);
        data.add_note(Note::from("Order started"));
        Order {
            data,
            _state: PhantomData,
        }
    }
}   

impl Order<InProgress> {
    /// Complete the order and transition to the Completed state.
    pub fn complete(self) -> Order<Completed> {
        let mut data = self.data;
        data.state = Some(OrderState::Completed);
        data.add_note(Note::from("Order Completed"));
        Order {
            data,
            _state: PhantomData,
        }
    }

    pub fn cancel(self,reason : impl Into<String>) -> Order<Cancelled> {
        let mut data = self.data;
        data.state = Some(OrderState::Cancelled);
        data.add_note(Note::from(format!("Order Cancelled: {}", reason.into()).as_str()));
        Order {
            data,
            _state: PhantomData,
        }
    }
}
#[cfg(test)]
mod test {
    use super::OrderState;

    #[test]
    fn create_draft() {
        let order = super::Order::<super::Draft>::new();

        assert_eq!(order.data.state, Some(OrderState::Draft));
    }

    #[test]
    fn state_transitions() {
        let draft_order = super::Order::<super::Draft>::new();

        assert_eq!(draft_order.data.state, Some(OrderState::Draft));

        let completed_order = draft_order.acknowledge().start().complete();

        assert_eq!(completed_order.data.state, Some(OrderState::Completed));
    }

    #[test]
    fn cancel_order() {
        let draft_order = super::Order::<super::Draft>::new();

        assert_eq!(draft_order.data.state, Some(OrderState::Draft));

        let cancelled_order = draft_order.acknowledge().start().cancel("Customer requested cancellation");

        assert_eq!(cancelled_order.data.state, Some(OrderState::Cancelled));
        // assert_eq!(cancelled_order.data.note.len(), 1);
        // assert_eq!(cancelled_order.data.note[0].text, "Order Cancelled: Customer requested cancellation");
    }   
}