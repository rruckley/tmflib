//! TMF622 Wrapper to implement TypeState pattern

use std::marker::PhantomData;
use crate::HasLastUpdate;

use super::product_order_v4::ProductOrder;

type OrderState = super::product_order_v4::ProductOrderStateType;

pub struct Draft;
pub struct Acknowledged;
pub struct InProgress;
pub struct Completed;
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

pub struct Order<S> {
    data: ProductOrder,
    _state: PhantomData<S>,
}

impl Order<Draft> {
    /// Create a new order in the Draft state.
    pub fn new() -> Self {
        let mut data = ProductOrder::create_with_time();
        data.state = Some(OrderState::Draft);
        Order {        
            data,
            _state: PhantomData,
        }
    }

    /// Acknowledge the order and transition to the Acknowledged state.
    pub fn acknowledge(self) -> Order<Acknowledged> {
        let mut data = self.data;
        data.state = Some(OrderState::Acknowledged);
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
        Order {
            data,
            _state: PhantomData,
        }
    }

    pub fn cancel(self) -> Order<Cancelled> {
        let mut data = self.data;
        data.state = Some(OrderState::Cancelled);
        Order {
            data,
            _state: PhantomData,
        }
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn create_draft() {
        let order = super::Order::<super::Draft>::new();

        assert_eq!(order.data.state, Some(super::OrderState::Draft));
    }

    #[test]
    fn failed_transition() {
        let order = super::Order::<super::Draft>::new();

        let order = order.acknowledge();
// This should fail to compile because complete() is not defined for Order<Draft>
        let order = order.start();

        let order = order.complete();

        // This should fail to compile because cancel() is not defined for Order<Completed>
        // let order = order.cancel();
    }
}