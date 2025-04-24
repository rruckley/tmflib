//! Intent Expression Module

use serde::{Deserialize, Serialize};

/// Represents an expression in the TMF921 Intent Management API.
/// This struct is used to define the different types of expressions that can be associated with an intent.
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TurtleExpression {
    /// An unique identifier for the expression.
    pub iri : String,
    /// An value of the expression.
    pub exression_value : String,
}

/// Represents a JSON-LD expression in the TMF921 Intent Management API.
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct JsonLdExpression {
    /// An unique identifier for the expression.
    /// This is a JSON-LD expression.
    pub iri : String,
}

/// Represents the type of expression in the TMF921 Intent Management API.
/// This enum is used to define the different types of expressions that can be associated with an intent.
#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum ExpressionType {
    /// Another intent expression.
    IntentExpression(Box<IntentExpression>),
    /// A turtle expression.
    TurtleExpression(Box<TurtleExpression>),
    /// A JSON-LD expression.
    JsonLdExpression(Box<JsonLdExpression>),
}

/// Represents an intent expression in the TMF921 Intent Management API.
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct IntentExpression {
    /// An unique identifier for the expression.
    pub iri : String,
    /// A value for the expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<ExpressionType>,
}