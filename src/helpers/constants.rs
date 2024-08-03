pub const MAX_PAYLOAD_SIZE: usize = 256;
pub const SAMPLE_RATE: u32 = 44100;
pub const MARK_FREQ: f32 = 1200.0;
pub const SPACE_FREQ: f32 = 2200.0;
pub const BAUD_RATE: f32 = 1200.0;
pub const SAMPLES_PER_BIT: usize = (SAMPLE_RATE as f32 / BAUD_RATE) as usize;
pub const BITS_PER_SAMPLE: u16 = 16;
pub const FLAG: u8 = 0x7E;
