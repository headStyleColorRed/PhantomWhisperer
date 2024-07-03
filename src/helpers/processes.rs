use super::encoder;
use super::modulator;
use super::decoder;

pub fn encode_message(message: &str, output_file: &str) {
    match encoder::encode_message(message, output_file) {
        Ok(_) => {
            println!("[ENCODER]: Encoded message written to file: {}", output_file);
            modulate_file(output_file, "src/files/modulated.wav");
        },
        Err(e) => eprintln!("[ENCODER]: Error encoding message: {}", e),
    }
}

pub fn modulate_file(input_file: &str, output_file: &str) {
    println!("[MODULATOR]: Modulating file: {} to {}", input_file, output_file);
    match modulator::modulate_file(input_file, output_file) {
        Ok(_) => println!("[MODULATOR]: Modulation completed successfully. File created in path '{}'", output_file),
        Err(e) => eprintln!("[MODULATOR]: Error during modulation: {}", e),
    }
}

pub fn decode_file(input_file: &str, output_file: &str) {
    println!("Decoding file: {} to {}", input_file, output_file);
    match decoder::decode_file(input_file, output_file) {
        Ok(_) => println!("Decoding completed successfully. File created in path '{}'", output_file),
        Err(e) => eprintln!("Error during decoding: {}", e),
    }
}
