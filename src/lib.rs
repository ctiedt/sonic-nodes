//! A library to procedurally generate sound in Rust.
//! Inspired by [Blender](https://blender.org)'s shader nodes.

pub mod math;
pub mod wave;

use std::fmt::Debug;

/// A simple wrapper for nodes used as an input to other nodes
pub type NodeInput<T> = Box<dyn Node<T> + Send + Sync>;

/// The trait used to represent nodes in our audio graph.
/// To implement it, create a new struct that has your inputs
/// as fields and implement `Node<T>` for it where `T` is your
/// output type (most of the time `f64`).
///
/// Example:
///
/// ```rust
/// struct Double {
///     input: Value,
/// }
///
/// impl Node<f64> for Double {
///     fn process(&mut self) -> f64 {
///         self.input.process() * 2.0
///     }
/// }
/// ```
pub trait Node<T>
where
    T: Debug,
{
    /// Output the next value of your node by evaluating
    /// the inputs and possibly changing internal state
    fn process(&mut self) -> T;

    /// Continuously output values from your node
    fn play(&mut self) -> ! {
        loop {
            println!("{:?}", self.process());
        }
    }
}

/// A value can be a constant value or the output of another node.
pub enum Value<T> {
    Const(T),
    Node(NodeInput<T>),
}

impl Node<f64> for Value<f64> {
    fn process(&mut self) -> f64 {
        match self {
            Value::Const(v) => *v,
            Value::Node(n) => n.process(),
        }
    }
}

impl From<f64> for Value<f64> {
    fn from(v: f64) -> Self {
        Self::Const(v)
    }
}

/// The basic input used by most other nodes.
/// It would seem reasonable to use e.g. `Instant` here, but
/// this way works better with sampling (using cpal for output).
pub struct Time {
    state: f64,
}

impl Time {
    pub fn new() -> Self {
        Self { state: 0. }
    }
}

impl Node<f64> for Time {
    fn process(&mut self) -> f64 {
        self.state += 1.0;
        self.state
    }
}
