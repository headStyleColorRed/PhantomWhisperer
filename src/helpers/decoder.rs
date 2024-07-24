use hound;
use super::constants::*;
use std::collections::VecDeque;
use rustfft::{FftPlanner, num_complex::Complex};

const PREAMBLE_MATCH_THRESHOLD: f32 = 0.9;

#[derive(Debug, PartialEq)]
enum DecoderState {
    SearchingPreamble,
    ReadingSize,
    DecodingData,
    SearchingPostamble
}

fn detect_bits(chunk: &[i16]) -> u8 {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(chunk.len());

    let mut buffer: Vec<Complex<f32>> = chunk.iter()
        .map(|&s| Complex::new(s as f32, 0.0))
        .collect();

    fft.process(&mut buffer);

    let power_1500 = buffer[1500 * chunk.len() / SAMPLE_RATE as usize].norm();
    let power_2000 = buffer[2000 * chunk.len() / SAMPLE_RATE as usize].norm();
    let power_2500 = buffer[2500 * chunk.len() / SAMPLE_RATE as usize].norm();
    let power_3000 = buffer[3000 * chunk.len() / SAMPLE_RATE as usize].norm();

    let max_power = power_1500.max(power_2000).max(power_2500).max(power_3000);

    if max_power == power_1500 { 0 }
    else if max_power == power_2000 { 1 }
    else if max_power == power_2500 { 2 }
    else { 3 }
}

fn compare_pattern(detected: &[u8], pattern: &[u8]) -> f32 {
    let matches = detected.iter()
        .zip(pattern.iter())
        .filter(|&(a, b)| a == b)
        .count();
    matches as f32 / pattern.len() as f32
}

pub fn decode_file(input_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[DECODER] Starting to decode file: {}", input_file);

    let mut reader = hound::WavReader::open(input_file)?;
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    println!("[DECODER] Read {} samples from file", samples.len());

    let mut decoded_symbols = Vec::new();
    let mut current_position = 0;
    let mut state = DecoderState::SearchingPreamble;
    let mut symbol_buffer = VecDeque::new();
    let mut size_buffer = Vec::new();
    let mut data_size = 0;

    while current_position + SAMPLES_PER_SYMBOL as usize <= samples.len() {
        let chunk = &samples[current_position..current_position + SAMPLES_PER_SYMBOL as usize];
        let symbol = detect_bits(chunk);
        symbol_buffer.push_back(symbol);
        if symbol_buffer.len() > PREAMBLE.len() {
            symbol_buffer.pop_front();
        }

        match state {
            DecoderState::SearchingPreamble => {
                if symbol_buffer.len() == PREAMBLE.len() && compare_pattern(&symbol_buffer.make_contiguous(), &PREAMBLE) >= PREAMBLE_MATCH_THRESHOLD {
                    println!("[DECODER] Preamble detected at position {}", current_position);
                    decoded_symbols.extend(&PREAMBLE);
                    symbol_buffer.clear();
                    state = DecoderState::ReadingSize;
                }
            },
            DecoderState::ReadingSize => {
                size_buffer.push(symbol);
                if size_buffer.len() == SIZE_BITS / 2 {  // SIZE_BITS / 2 because we're using 2-bit symbols
                    data_size = size_buffer.iter().enumerate().fold(0, |acc, (i, &sym)| {
                        acc | ((sym as usize) << (2 * ((SIZE_BITS / 2) - 1 - i)))
                    });
                    println!("[DECODER] Data size detected: {} symbols", data_size);
                    decoded_symbols.extend(&size_buffer);
                    size_buffer.clear();
                    state = DecoderState::DecodingData;
                }
            },
            DecoderState::DecodingData => {
                if decoded_symbols.len() - PREAMBLE.len() - (SIZE_BITS / 2) == data_size {
                    println!("[DECODER] All data decoded, searching for postamble");
                    state = DecoderState::SearchingPostamble;
                } else {
                    decoded_symbols.push(symbol);
                }
            },
            DecoderState::SearchingPostamble => {
                if symbol_buffer.len() == POSTAMBLE.len() && compare_pattern(&symbol_buffer.make_contiguous(), &POSTAMBLE) >= PREAMBLE_MATCH_THRESHOLD {
                    println!("[DECODER] Postamble detected, decoding complete");
                    decoded_symbols.extend(&POSTAMBLE);
                    break;
                }
            }
        }

        current_position += SAMPLES_PER_SYMBOL as usize;
    }

    println!("[DECODER] Total decoded symbols: {}", decoded_symbols.len());

    if decoded_symbols.len() < PREAMBLE.len() + (SIZE_BITS / 2) + POSTAMBLE.len() {
        return Err("Insufficient data decoded".into());
    }

    // Convert decoded symbols to bytes
    let data_start = PREAMBLE.len() + (SIZE_BITS / 2);
    let data_end = decoded_symbols.len() - POSTAMBLE.len();
    let decoded_bytes: Vec<u8> = decoded_symbols[data_start..data_end]
        .chunks(4)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &sym| (acc << 2) | sym))
        .collect();

    println!("[DECODER] Converted symbols to {} bytes", decoded_bytes.len());

    // Convert bytes to string
    let decoded_string = String::from_utf8(decoded_bytes)?;

    println!("[DECODER] Decoding completed successfully");
    Ok(decoded_string)
}
