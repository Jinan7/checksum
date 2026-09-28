use std::{path::{Path}};
use base64::prelude::*;
use sha2::{Digest, Sha256};

fn main() {
    let mut path_string = String::new();
    std::io::stdin().read_line(&mut path_string).expect("failed to read path string");
    let path = Path::new(&path_string);
    let bytes = std::fs::read(path).expect("Failed to read file");
    
    let hash = Sha256::digest(bytes);
    let checksum = BASE64_STANDARD.encode(hash);
    println!("{:?}", hash);
    println!("{}", checksum);
}
