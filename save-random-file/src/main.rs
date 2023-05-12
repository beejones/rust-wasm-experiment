use std::fs::File;
use std::io::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::path::PathBuf;

fn main() {
    let rng = rand::thread_rng();
    let random_name: String = rng.sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    
    let mut file_path = PathBuf::from("/tmp");
    file_path.push(random_name);

    let mut file = File::create(&file_path).expect("Failed to create file");
    
    // Optionally, write some content to the file
    file.write_all(b"Hello, World!").expect("Failed to write to file");

    println!("File created: {:?}", file_path);
}