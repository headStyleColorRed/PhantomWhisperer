# Phantom Pulse

Phantom Pulse is a Rust-based CLI application for encoding messages, modulating them into audio signals, and preparing them for radio transmission.

![Phantom Pulse Logo](Resources/icon.jpeg)

## Features

- String encoding to Wav
- Wav de-encoding to String

## Installation

1. Ensure you have Rust and Cargo installed on your system. If not, install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone the repository:
   ```
   git clone https://github.com/headstylecolorred/phantom-pulse.git
   cd phantom-pulse
   ```

3. Build the project:
   ```
   cargo run
   ```

## Usage

Phantom Pulse provides several commands for encoding, modulating, and decoding messages:

1. Encode a message:
   ```
   cargo run -- encode "Your secret message" encoded_message.txt
   ```

2. Modulate an encoded file:
   ```
   cargo run -- modulate encoded_message.txt modulated_signal.wav
   ```

3. Decode a modulated file:
   ```
   cargo run -- decode modulated_signal.wav decoded_message.json
   ```

4. View help:
   ```
   cargo run -- help
   ```

## Technical Details

- Encoding: Base64
- Modulation: Binary FSK (1000 Hz for 0, 2000 Hz for 1)
- Audio: 44.1 kHz sample rate, 16-bit depth
- Decoding: Goertzel algorithm for frequency detection

## Legal and Safety

**Important:** This software is designed for educational purposes only. Transmitting radio signals without proper authorization may be illegal in your jurisdiction. Always ensure you comply with local laws and regulations before attempting any radio transmissions.

## Contributing

Contributions to Phantom Pulse are welcome! Please feel free to submit pull requests, create issues or spread the word.

## License

MIT
