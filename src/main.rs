use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the name of the program.
    // The remaining arguments are the user-supplied arguments.
    if args.len() > 1 {
        for arg in &args[1..] {
            println!("Argument: {}", arg);
        }
    } else {
        println!("No arguments passed!");
    }
}
