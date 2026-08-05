//! TMF622 Wrapper to implement TypeState pattern

use std::marker::PhantomData;
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

pub struct OrderData {
    data : ProductOrder,
}

pub struct Order<S> {
    data: OrderData,
    _state: PhantomData<S>,
}

impl Order<Draft> {
    pub fn new() -> Self {
        Order {
            data: OrderData {
                data: ProductOrder::new(),
            },
            _state: PhantomData,
        }
    }

    pub fn acknowledge(self) -> Order<Acknowledged> {
        let mut data = self.data;
        data.data.state = Some(OrderState::Acknowledged);
        Order {
            data,
            _state: PhantomData,
        }
    }
}

impl Order<Acknowledged> {
    pub fn start(self) -> Order<InProgress> {
        let mut data = self.data;
        data.data.state = Some(OrderState::InProgress);
        Order {
            data,
            _state: PhantomData,
        }
    }
}   

impl Order<InProgress> {
    pub fn complete(self) -> Order<Completed> {
        let mut data = self.data;
        data.data.state = Some(OrderState::Completed);
        Order {
            data,
            _state: PhantomData,
        }
    }

    pub fn cancel(self) -> Order<Cancelled> {
        let mut data = self.data;
        data.data.state = Some(OrderState::Cancelled);
        Order {
            data,
            _state: PhantomData,
        }
    }
}