use std::f32::consts::PI;
use std::fs::File;
use std::io::Read;
use hound;
use super::constants::*;

// Constants for audio generation
const SAMPLE_RATE: u32 = 44100;     // Standard CD-quality audio sample rate

/// Modulates the contents of an input file into a WAV file using FSK
/// and returns a result indicating success or an error
pub fn modulate_file(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire contents of the input file
    let mut file = File::open(input_file)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    println!("[MODULATOR]: Read {} bytes from input file", contents.len());
    println!("[MODULATOR]: First few bytes: {:?}", &contents[..std::cmp::min(10, contents.len())]);

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
    for (_i, &symbol) in contents.iter().enumerate() {
        write_symbol(&mut writer, symbol)?;
    }

    writer.finalize()?;

    Ok(())
}

/// Writes a single symbol to the WAV file
fn write_symbol(writer: &mut hound::WavWriter<std::io::BufWriter<File>>, symbol: u8) -> Result<(), Box<dyn std::error::Error>> {
    let freq = match symbol {
        0 => FREQUENCY_00,
        1 => FREQUENCY_01,
        2 => FREQUENCY_10,
        3 => FREQUENCY_11,
        _ => {
            println!("[MODULATOR]: Invalid symbol encountered: {}", symbol);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid symbol")));
        }
    };

    let samples = generate_sine_wave(freq, SAMPLES_PER_SYMBOL);

    // Write the samples to the WAV file
    for sample in samples {
        // Convert float sample to 16-bit integer and write to file
        // Scale to 50% of maximum amplitude to avoid clipping
        let scaled_sample = (sample * i16::MAX as f32 * 0.5) as i16;
        writer.write_sample(scaled_sample)?;
    }

    Ok(())
}

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
