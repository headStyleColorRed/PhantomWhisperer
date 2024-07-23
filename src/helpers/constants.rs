// Patterns
pub const PREAMBLE: [bool; 16] = [true, true, true, false, true, true, true, false, true, true, true, false, true, true, true, false];
pub const POSTAMBLE: [bool; 16] = [false, false, false, true, false, false, false, true, false, false, true, false, false, false, true, false];
pub const SIZE_BITS: usize = 32;  // Number of bits used to represent the data size, allowing a maximum of 4GB
