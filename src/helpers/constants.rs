// Patterns
pub const PREAMBLE: [u8; 8] = [0, 1, 2, 3, 0, 1, 2, 3];
pub const POSTAMBLE: [u8; 8] = [3, 2, 1, 0, 3, 2, 1, 0];
pub const SIZE_BITS: usize = 32;  // Number of bits used to represent the data size, allowing a maximum of 4GB
pub const SAMPLES_PER_SYMBOL: u32 = 200;  // Double the previous SAMPLES_PER_BIT
pub const SAMPLE_RATE: u32 = 44100;     // Standard CD-quality audio sample rate


pub const MAX_PAYLOAD_SIZE: usize = 256;
