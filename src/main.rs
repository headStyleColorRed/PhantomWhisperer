use std::env;
use std::process;
// Import helper functions
mod helpers;


fn print_usage() {
    println!("Usage: phantom-pulse <command> [options]");
    println!("Commands:");
    println!("  encode <message>    Encode a message");
    println!("  modulate <file>     Modulate an encoded file");
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
            if args.len() < 3 {
                println!("Error: No file provided for modulation.");
                process::exit(1);
            }
            modulate_file(&args[2], &args[3]);
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
