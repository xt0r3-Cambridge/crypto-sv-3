extern crate rand;
extern crate sha1;
use rand::Rng;
use sha1::{Digest, Sha1};
use std::collections::HashMap;

fn genpasswd(input: u64) -> u64 {
    // Ensure we're working with only the lower 36 bits of the input
    let input_bits = input & ((1 << 36) - 1);

    // Convert the 36-bit input to bytes. Assuming big-endian for this example.
    let input_bytes: Vec<u8> = input_bits.to_be_bytes()[4..8].to_vec(); // Adjusted slicing

    // Create a Sha1 object and compute the hash
    let mut hasher = Sha1::new();
    hasher.update(input_bytes);
    let result = hasher.finalize();

    // Truncate to 36 bits and convert back to u64
    let output = u64::from_be_bytes([
        0, 0, 0, result[0], result[1], result[2], result[3], result[4],
    ]) >> 4; // Adjusted to directly use bytes
    return output;
}

fn geninput() -> u64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..((1 << 36) - 1));
}

fn main() {
    let mut hashes = HashMap::new();

    let mut key: u64 = geninput();
    let mut hash = None;
    while hash == None || hash == Some(key) {
        hashes.insert(genpasswd(key), key);
        key = geninput();
        hash = hashes.get(&genpasswd(key)).copied();
    }

    println!("Number of tries: {}", hashes.len() + 1);
    println!("Expected number of tries: {}", 1 << 18);
    match hashes.get(&genpasswd(key)) {
        None => panic!("Element not found in map"),
        Some(hash) => println!(
            "Passwords with matching hash: h({:X}) = h({:X}) = {:X}",
            key,
            hash,
            genpasswd(key)
        ),
    }
}
