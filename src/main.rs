use std::{fs::File, path::Path};
use base64::prelude::*;
use sha2::{Digest, Sha256};

fn main() {
    let path = Path::new("Oppenheimer.mp4");
    with_chksum(path);
    without_chksum(path);
   
}

fn without_chksum(path: &Path) {
    
    println!("Reading from path {}", path.display());
    let bytes = std::fs::read(path).expect("Failed to read file");
    
    let hash = Sha256::digest(bytes).to_vec();
    let hex_string = hash.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join("");
    let checksum = BASE64_STANDARD.encode(hash);
    println!("without_chksum: {}", checksum);
    println!("without_chksum: {}", hex_string);
}
fn with_chksum(path: &Path) {
    println!("Reading from path {}", path.display());
    let file = File::open(path).expect("failed to open file");
    let digest = chksum_sha2_256::chksum(file).expect("failed to compute checksum");
    println!("with_chksum: {}", digest);

}
