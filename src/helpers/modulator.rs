use std::f32::consts::PI;
use std::fs::File;
use std::io::Read;
use hound;

const SAMPLE_RATE: u32 = 44100;
const SAMPLES_PER_BIT: u32 = 100;
const FREQUENCY_0: f32 = 1000.0; // Frequency for bit 0
const FREQUENCY_1: f32 = 2000.0; // Frequency for bit 1

fn generate_sine_wave(freq: f32, num_samples: u32) -> Vec<f32> {
    (0..num_samples)
        .map(|i| (2.0 * PI * freq * i as f32 / SAMPLE_RATE as f32).sin())
        .collect()
}

pub fn modulate_file(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    let mut file = File::open(input_file)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Prepare WAV writer
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_file, spec)?;

    // Modulate each byte
    for byte in contents {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            let freq = if bit == 0 { FREQUENCY_0 } else { FREQUENCY_1 };
            let samples = generate_sine_wave(freq, SAMPLES_PER_BIT);

            for sample in samples {
                writer.write_sample((sample * i16::MAX as f32) as i16)?;
            }
        }
    }

    writer.finalize()?;

    Ok(())
}
