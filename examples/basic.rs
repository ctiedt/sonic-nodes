use sonic_nodes::{wave::SquareWave, Node, Time};

fn main() {
    let mut node = SquareWave {
        time: Box::new(Time::new()),
        amplitude: 1.0.into(),
        frequency: 440.0.into(),
        sample_rate: 1.0.into(),
    };

    node.play();
}
