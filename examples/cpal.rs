use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use sonic_nodes::{math::Math, wave::SineWave, Node, Time};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
    }
    .unwrap();
}

pub fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    let mut node = Math::Add(
        Box::new(SineWave {
            time: Box::new(Time::new()),
            amplitude: 1.0.into(),
            frequency: 440.0.into(),
            sample_rate: sample_rate.into(),
        }),
        Box::new(SineWave {
            time: Box::new(Time::new()),
            amplitude: 1.0.into(),
            frequency: 200.0.into(),
            sample_rate: sample_rate.into(),
        }),
    );

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let v = node.process() as f32;
                let value: T = cpal::Sample::from::<f32>(&v);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
