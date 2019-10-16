extern crate challenges;
use challenges::matrix::correct_path;
use challenges::string::reverse;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_string = args[1].to_string();
    //TODO check that is string and contain only 'durl?'
    //TODO The first argument is the path that was used to call the program.
    println!("The path to correct is {}.", args[1]);

    reverse::make("my crate".to_string());
    correct_path::make(input_string);
}
