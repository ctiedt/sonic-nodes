//! This module contains mathematical operations that can be used to manipulate float values.

use crate::{Node, NodeInput, Value};

/// Mathematical operations to combine different inputs
pub enum Math {
    Add(NodeInput<f64>, NodeInput<f64>),
    Subtract(NodeInput<f64>, NodeInput<f64>),
    Multiply(NodeInput<f64>, NodeInput<f64>),
    Divide(NodeInput<f64>, NodeInput<f64>),
    Pow(NodeInput<f64>, NodeInput<f64>),
    Sqrt(NodeInput<f64>),
}

impl Node<f64> for Math {
    fn process(&mut self) -> f64 {
        match self {
            Math::Add(in1, in2) => in1.process() + in2.process(),
            Math::Subtract(in1, in2) => in1.process() - in2.process(),
            Math::Multiply(in1, in2) => in1.process() * in2.process(),
            Math::Divide(in1, in2) => in1.process() / in2.process(),
            Math::Pow(in1, in2) => in1.process().powf(in2.process()),
            Math::Sqrt(in1) => in1.process().sqrt(),
        }
    }
}

/// Cut off the value if it is below the minimum or above the maximum.
pub struct Limit {
    pub input: Value<f64>,
    pub min: Option<Value<f64>>,
    pub max: Option<Value<f64>>,
}

impl Node<f64> for Limit {
    fn process(&mut self) -> f64 {
        self.input
            .process()
            .min(
                self.max
                    .as_mut()
                    .unwrap_or(&mut Value::Const(f64::MAX))
                    .process(),
            )
            .max(
                self.min
                    .as_mut()
                    .unwrap_or(&mut Value::Const(f64::MIN))
                    .process(),
            )
    }
}
