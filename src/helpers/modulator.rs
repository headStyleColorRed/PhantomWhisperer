use std::f32::consts::PI;
use std::fs::File;
use std::io::Read;
use hound;

// Constants for audio generation
const SAMPLE_RATE: u32 = 44100;     // Standard CD-quality audio sample rate
const SAMPLES_PER_BIT: u32 = 100;   // Number of samples to represent each bit
const FREQUENCY_0: f32 = 1000.0;    // Frequency used to represent bit 0
const FREQUENCY_1: f32 = 2000.0;    // Frequency used to represent bit 1

/// Generates a sine wave of specified frequency and duration
/// and returns a vector of f32 values representing the sine wave
fn generate_sine_wave(freq: f32, num_samples: u32) -> Vec<f32> {
    (0..num_samples)
        .map(|i| {
            // Calculate the sine value for each sample
            (2.0 * PI * freq * i as f32 / SAMPLE_RATE as f32).sin()
        })
        .collect()
}

/// Modulates the contents of an input file into a WAV file using FSK
/// and returns a result indicating success or an error
pub fn modulate_file(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire contents of the input file
    let mut file = File::open(input_file)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Set up the WAV file specifications
    let spec = hound::WavSpec {
        channels: 1,  // Mono audio
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,  // 16-bit audio
        sample_format: hound::SampleFormat::Int,
    };

    // Create a WAV writer for the output file
    let mut writer = hound::WavWriter::create(output_file, spec)?;

    // Process each byte in the input file
    for byte in contents {
        // Process each bit in the byte, from most significant to least (per convention)
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;  // Extract the current bit
            let freq = if bit == 0 { FREQUENCY_0 } else { FREQUENCY_1 };  // Choose frequency based on bit value
            let samples = generate_sine_wave(freq, SAMPLES_PER_BIT);  // Generate the sine wave for this bit

            // Write the samples to the WAV file
            for sample in samples {
                // Convert float sample to 16-bit integer and write to file
                writer.write_sample((sample * i16::MAX as f32) as i16)?;
            }
        }
    }

    // Finalize the WAV file
    writer.finalize()?;

    Ok(())
}
