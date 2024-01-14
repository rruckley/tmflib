//! Cost Module

use crate::{HasId,CreateTMF,LIB_PATH,TimePeriod};
use serde::{Deserialize,Serialize};
use std::convert::From;

use super::MOD_PATH;
use crate::common::money::Money;

const COST_PATH : &str = "cost";
const COST_DEFAULT : f32 = 1.0;

/// Cost Reference
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CostRef {
    id: String,
    href: String,
    name: String,
    value: Money,
}

impl From<Cost> for CostRef {
    fn from(value: Cost) -> Self {
        CostRef {
            id: value.get_id(),
            href: value.get_href(),
            name: value.name.unwrap_or("NoName".to_string()),
            value: value.cost.clone(),
        }
    }
}

/// Cost Management
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CostModel {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HREF to this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name of this cost
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// Cost Value
    pub cost : Money,
    /// Parent Cost
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent : Option<String>,
    /// Child Costs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_costs : Option<Vec<CostRef>>,
}

impl CostModel {
    /// Create new cost entry
    pub fn new(name : &str) -> Cost {
        let mut cost = Cost::create();
        cost.name = Some(name.to_owned());
        cost.valid_for = Some(TimePeriod::default());
        cost.child_costs = Some(vec![]);
        cost.cost.value = COST_DEFAULT;
        cost
    }
    /// Set value for this cost
    pub fn cost(mut self, cost : Money) -> Cost {
        self.cost = cost;
        self
    }
    /// Add a child into this cost model
    pub fn add_child(&mut self,cost : Cost) {
        self.child_costs.as_mut().unwrap().push(CostRef::from(cost));
    }

    /// Sum up all costs from this entry down
    pub fn total_cost(&self) -> f32 {
        match self.child_costs.as_ref() {
            Some(cc) => {
                let vec = cc.clone();
                let sum = vec.into_iter().fold(0.0,|acc,cf| {
                    acc + cf.value.value
                });
                sum + self.cost.value
            }
            None => self.cost.value,
        }
    }
}

impl HasId for Cost {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,COST_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = Cost::get_uuid();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_class() -> String {
        COST_PATH.to_string()    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()       
    }
}

impl CreateTMF<Cost> for Cost {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_name() {
        let cost = Cost::new("MyCost");

        assert_eq!(cost.name.unwrap(),"MyCost".to_string());
    }

    #[test]
    fn test_default_cost() {
        let cost = Cost::new("MyCost");
        assert_eq!(cost.cost.unit,"".to_string());
        assert_eq!(cost.cost.value,COST_DEFAULT);
    }
}