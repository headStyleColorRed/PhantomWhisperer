use hound;
use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use base64::{engine::general_purpose, Engine as _};

const SAMPLE_RATE: f32 = 44100.0;
const SAMPLES_PER_BIT: u32 = 100;
const FREQUENCY_0: f32 = 1000.0;
const FREQUENCY_1: f32 = 2000.0;

fn goertzel(samples: &[i16], target_frequency: f32) -> f32 {
    let omega = 2.0 * PI * target_frequency / SAMPLE_RATE;
    let coeff = 2.0 * omega.cos();
    let (mut s1, mut s2) = (0.0, 0.0);

    for &sample in samples {
        let s0 = sample as f32 + coeff * s1 - s2;
        s2 = s1;
        s1 = s0;
    }

    (s1 * s1 + s2 * s2 - coeff * s1 * s2).sqrt()
}

pub fn decode_file(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(input_file)?;
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    let mut bits = Vec::new();
    for chunk in samples.chunks(SAMPLES_PER_BIT as usize) {
        let power_0 = goertzel(chunk, FREQUENCY_0);
        let power_1 = goertzel(chunk, FREQUENCY_1);
        bits.push(if power_1 > power_0 { 1 } else { 0 });
    }

    let bytes: Vec<u8> = bits.chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b))
        .collect();

    let decoded = String::from_utf8(bytes)?;
    let json_message = general_purpose::STANDARD.decode(decoded)?;
    let json_string = String::from_utf8(json_message)?;

    let mut file = File::create(output_file)?;
    file.write_all(json_string.as_bytes())?;

    println!("Decoded message saved to: {}", output_file);
    Ok(())
}
