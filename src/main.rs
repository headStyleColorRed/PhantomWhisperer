use std::env;
use std::process;
mod helpers;

fn print_usage() {
    println!("Usage: phantom-pulse <command> [options]");
    println!("Commands:");
    println!("  encode <message> <output_file>    Encode a message");
    println!("  modulate <input_file> <output_file>  Modulate an encoded file");
    println!("  decode <input_file> <output_file>    Decode a modulated file");
    println!("  transmit <file>     Transmit a modulated file");
    println!("  help                Show this help message");
}

fn encode_message(message: &str, output_file: &str) {
    match helpers::encoder::encode_message(message, output_file) {
        Ok(_) => {
            println!("[ENCODER]: Encoded message written to file: {}", output_file);
            modulate_file(output_file, "src/files/modulated.wav");
        },
        Err(e) => eprintln!("Error encoding message: {}", e),
    }
}

fn modulate_file(input_file: &str, output_file: &str) {
    println!("Modulating file: {} to {}", input_file, output_file);
    match helpers::modulator::modulate_file(input_file, output_file) {
        Ok(_) => println!("Modulation completed successfully. File created in path '{}'", output_file),
        Err(e) => eprintln!("Error during modulation: {}", e),
    }
}

fn decode_file(input_file: &str, output_file: &str) {
    println!("Decoding file: {} to {}", input_file, output_file);
    match helpers::decoder::decode_file(input_file, output_file) {
        Ok(_) => println!("Decoding completed successfully. File created in path '{}'", output_file),
        Err(e) => eprintln!("Error during decoding: {}", e),
    }
}

fn transmit_file(file: &str) {
    println!("Transmitting file: {}", file);
    // TODO: Implement actual transmission logic
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: No command provided.");
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "encode" => {
            if args.len() < 4 {
                println!("Error: Encoding requires a message and an output file name.");
                process::exit(1);
            }
            encode_message(&args[2], &args[3]);
        }
        "modulate" => {
            if args.len() < 4 {
                println!("Error: Modulation requires input and output file names.");
                process::exit(1);
            }
            modulate_file(&args[2], &args[3]);
        }
        "decode" => {
            if args.len() < 4 {
                println!("Error: Decoding requires input and output file names.");
                process::exit(1);
            }
            decode_file(&args[2], &args[3]);
        }
        "transmit" => {
            if args.len() < 3 {
                println!("Error: No file provided for transmission.");
                process::exit(1);
            }
            transmit_file(&args[2]);
        }
        "help" => {
            print_usage();
        }
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}
