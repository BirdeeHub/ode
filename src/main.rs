use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {

    // Open the file
    let mut file = File::open(file_path)?;

    // Create a string buffer to store the file contents
    let mut contents = String::new();

    // Read the file into the string buffer
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Print all arguments
    println!("Arguments: {:?}", args);

    let file_path = if args.len() > 1 {
        Ok(&args[1])
    } else {
        Err("No file path provided")
    };

    match file_path {
        Ok(path) => match read_file(path) {
            Ok(contents) => println!("File contents:\n{}", contents),
            Err(e) => eprintln!("Error reading file: {}", e),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
