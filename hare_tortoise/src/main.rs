use sha1::{Digest, Sha1};

fn genpasswd(cur: u64) -> u64 {
    // Make sure that we only have the first 36 bits
    let cur_bits = cur &  ((1 << 36) - 1);

    // Bytes 4, 5, 6, 7, 8
    let cur_bytes: Vec<u8> = cur_bits.to_be_bytes()[4..8].to_vec();

    let mut hasher = Sha1::new();
    hasher.update(cur_bytes);
    let result = hasher.finalize();

    // Return the bottom 36 bits as an u64
    return u64::from_be_bytes([0, 0, 0, result[0], result[1], result[2], result[3], result[4]]) >> 4;
}

fn main() {
    let mut hare: u64 = genpasswd(genpasswd(0));
    let mut tortoise: u64 = genpasswd(0);
    let mut steps = 0;

    while hare != tortoise {
        hare = genpasswd(genpasswd(hare));
        tortoise = genpasswd(tortoise);
        steps += 1;
    }

    tortoise = 0;

    while genpasswd(hare) != genpasswd(tortoise) {
        hare = genpasswd(hare);
        tortoise = genpasswd(tortoise);
        steps += 1;
    }

    println!(
        "Passwords with matching hash: h({:X}) = h({:X}) = {:X}",
        hare,
        tortoise,
        genpasswd(tortoise),
    );

    println!("Number of steps taken: {}", steps);
}
