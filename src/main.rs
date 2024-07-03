use std::env;
use std::process;
mod helpers;

fn encode_message(message: &str, output_file: &str) {
    helpers::processes::encode_message(message, output_file)
}

fn modulate_file(input_file: &str, output_file: &str) {
    helpers::processes::modulate_file(input_file, output_file)
}

fn decode_file(input_file: &str, output_file: &str) {
    helpers::processes::decode_file(input_file, output_file)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: No command provided.");
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
            // transmit_file(&args[2]);
        }
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            process::exit(1);
        }
    }
}
