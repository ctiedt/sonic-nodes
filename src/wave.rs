//! This module contains wave primitives that can be used as building blocks.

use crate::{Node, NodeInput, Value};

/// A sine wave
pub struct SineWave {
    /// Time input, though any node that outputs an `f64` can be used here
    pub time: NodeInput<f64>,
    /// The wave's amplitude. If in doubt, leave at `1.0`.
    pub amplitude: Value<f64>,
    /// The wave's frequency in Hz.
    pub frequency: Value<f64>,
    /// The sample rate is relevant for your output device.
    pub sample_rate: Value<f64>,
}

impl Node<f64> for SineWave {
    fn process(&mut self) -> f64 {
        (self.time.process() * self.frequency.process() * 2.0 * std::f64::consts::PI
            / self.sample_rate.process())
        .sin()
            * self.amplitude.process()
    }
}

/// A square wave
pub struct SquareWave {
    /// Time input, though any node that outputs an `f64` can be used here
    pub time: NodeInput<f64>,
    /// The wave's amplitude. If in doubt, leave at `1.0`.
    pub amplitude: Value<f64>,
    /// The wave's frequency in Hz.
    pub frequency: Value<f64>,
    /// The sample rate is relevant for your output device.
    pub sample_rate: Value<f64>,
}

impl Node<f64> for SquareWave {
    fn process(&mut self) -> f64 {
        (self.time.process() * self.frequency.process() * 2.0 * std::f64::consts::PI
            / self.sample_rate.process())
        .sin()
        .signum()
            * self.amplitude.process()
    }
}
