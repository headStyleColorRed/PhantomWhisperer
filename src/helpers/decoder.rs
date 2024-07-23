use hound;
use std::f32::consts::PI;
use base64::{engine::general_purpose, Engine as _};
use super::constants::*;
use super::debuger::*;
use std::collections::VecDeque;

const SAMPLE_RATE: f32 = 44100.0;
const SAMPLES_PER_BIT: u32 = 100;
const FREQUENCY_0: f32 = 1000.0;
const FREQUENCY_1: f32 = 2000.0;
const SYNC_THRESHOLD: f32 = 0.85;
const PREAMBLE_MATCH_THRESHOLD: f32 = 0.9;

#[derive(Debug, PartialEq)]
enum DecoderState {
    SearchingPreamble,
    ReadingSize,
    DecodingData,
    SearchingPostamble
}

fn correlate(samples: &[i16], frequency: f32) -> f32 {
    let omega = 2.0 * PI * frequency / SAMPLE_RATE;
    samples.iter().enumerate().fold(0.0, |acc, (i, &sample)| {
        acc + (sample as f32) * (omega * i as f32).sin()
    }).abs() / samples.len() as f32
}

fn detect_bit(chunk: &[i16]) -> bool {
    let power_0 = correlate(chunk, FREQUENCY_0);
    let power_1 = correlate(chunk, FREQUENCY_1);
    power_1 > power_0
}

fn find_sync(samples: &[i16]) -> usize {
    (0..SAMPLES_PER_BIT as usize)
        .map(|offset| {
            let power_diff = (0..16).map(|i| {
                let start = offset + i * SAMPLES_PER_BIT as usize;
                let end = start + SAMPLES_PER_BIT as usize;
                let chunk = &samples[start..end];
                let power_1 = correlate(chunk, FREQUENCY_1);
                let power_0 = correlate(chunk, FREQUENCY_0);
                (power_1 - power_0).abs()
            }).sum::<f32>();
            (offset, power_diff)
        })
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap())
        .map(|(offset, _)| offset)
        .unwrap_or(0)
}

fn compare_pattern(detected: &[bool], pattern: &[bool]) -> f32 {
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
    println!("[DECODER] Read {} samples from file.", samples.len());

    let sync_offset = find_sync(&samples);
    println!("[DECODER] Sync offset: {} samples", sync_offset);

    let mut decoded_bits = Vec::new();
    let mut current_position = sync_offset;
    let mut state = DecoderState::SearchingPreamble;
    let mut bit_buffer = VecDeque::new();
    let mut size_buffer = Vec::new();
    let mut data_size = 0;

    while current_position + SAMPLES_PER_BIT as usize <= samples.len() {
        let chunk = &samples[current_position..current_position + SAMPLES_PER_BIT as usize];
        let bit = detect_bit(chunk);
        bit_buffer.push_back(bit);
        if bit_buffer.len() > 16 {
            bit_buffer.pop_front();
        }

        match state {
            DecoderState::SearchingPreamble => {
                if bit_buffer.len() == PREAMBLE.len() && compare_pattern(&bit_buffer.make_contiguous(), &PREAMBLE) >= PREAMBLE_MATCH_THRESHOLD {
                    println!("[DECODER] Preamble detected");
                    decoded_bits.extend(&PREAMBLE);
                    bit_buffer.clear();
                    state = DecoderState::ReadingSize;
                }
            },
            DecoderState::ReadingSize => {
                size_buffer.push(bit);
                if size_buffer.len() == SIZE_BITS {
                    data_size = size_buffer.iter().enumerate().fold(0, |acc, (i, &bit)| acc | ((bit as usize) << (SIZE_BITS - 1 - i)));
                    println!("[DECODER] Data size detected: {} bits", data_size);
                    decoded_bits.extend(&size_buffer);
                    size_buffer.clear();
                    state = DecoderState::DecodingData;
                }
            },
            DecoderState::DecodingData => {
                if decoded_bits.len() - PREAMBLE.len() - SIZE_BITS == data_size {
                    state = DecoderState::SearchingPostamble;
                } else {
                    decoded_bits.push(bit);
                }
            },
            DecoderState::SearchingPostamble => {
                if bit_buffer.len() == POSTAMBLE.len() && compare_pattern(&bit_buffer.make_contiguous(), &POSTAMBLE) >= PREAMBLE_MATCH_THRESHOLD {
                    println!("[DECODER] Postamble detected, decoding complete");
                    decoded_bits.extend(&POSTAMBLE);
                    break;
                }
            }
        }

        current_position += SAMPLES_PER_BIT as usize;
    }

    println!("[DECODER] Decoded bits length: {}", decoded_bits.len());
    if decoded_bits.len() < PREAMBLE.len() + SIZE_BITS + POSTAMBLE.len() {
        return Err("File does not contain valid encoded data".into());
    }

    print_bits(&decoded_bits);

    let data_start = PREAMBLE.len() + SIZE_BITS;
    let data_end = decoded_bits.len() - POSTAMBLE.len();
    let bytes: Vec<u8> = decoded_bits[data_start..data_end]
        .chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b as u8))
        .collect();
    println!("[DECODER] Converted bits to bytes: {:?}", bytes);

    let decoded_string = String::from_utf8(bytes)?;
    println!("[DECODER] Converted bytes to string: {}", decoded_string);

    println!("[DECODER] File decoded successfully");
    Ok(decoded_string)
}
