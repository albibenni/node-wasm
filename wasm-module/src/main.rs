use std::io::prelude::*;
use std::fs;

//Entry point for WASI applications

fn main () {
    // Print out hello world!
    // This will handle writing to stdout for us using the WASI
    println!("Hello World!");

    // Create a file
    // We are creating a `helloworld.txt` file in the `/helloworld`...
    // This code requires the Wasi host to provide a `/hellororld...`
    // If the `/helloworld` directory is not available, the unw...
    // For example in Wasmtime, if you want to map the current...
    // invoke the runtime with the flag/argument: `--mapdir /....`
    // This will map the `/helloworld` directory on the guest,
    let mut file = fs::File::create("helloworld.txt").unwrap();

    // Write the text to the file we created
    write!(file,"Hello World!").unwrap();
}