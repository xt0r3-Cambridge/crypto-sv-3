use num_bigint::BigUint;

fn modexp(base: BigUint, exp: BigUint, modulus: BigUint) -> BigUint {
    if exp == BigUint::from(0u32) {
        return BigUint::from(1u32);
    }

    let mut res = modexp(
        base.clone(),
        exp.clone() / BigUint::from(2u32),
        modulus.clone(),
    );
    res = (res.clone() * res) % modulus.clone();
    if exp % BigUint::from(2u32) == BigUint::from(1u32) {
        res = (res * base) % modulus;
    }
    return res;
}

fn count_bits(source: &[u8]) -> u32{
    let mut res = 0u32;
    for &value in source.iter() {
        res += value.count_ones();
    }
    return res;
}

fn main() {
    // Test the implementation
    match (
        BigUint::parse_bytes(b"123456789", 10),
        BigUint::parse_bytes(b"987654321", 10),
        BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFF", 16),
        BigUint::parse_bytes(b"785446763117418429158664", 10),
    ) {
        (Some(base), Some(exp), Some(modulus), Some(expected_result)) => {
            assert_eq!(modexp(base, exp, modulus), expected_result)
        }
        _ => panic!("Error parsing the expected input"),
    }

    let base = BigUint::from(7u32);

    let mut exp_bytes: [u8; 66] = [0xFFu8; 66];
    exp_bytes[0] = 1;
    let exp = BigUint::from_bytes_be(&exp_bytes);

    let mut mod_bytes = [0xFFu8; 403];
    mod_bytes[0] = 1 << 2 - 1;
    let modulus = BigUint::from_bytes_be(&mod_bytes);

    // Make sure that the number of bits are right
    assert_eq!(count_bits(&exp_bytes), 521u32);
    assert_eq!(count_bits(&mod_bytes), 3217u32);


    let base_2 = modexp(base, exp, modulus);

    println!("{}", base_2 % BigUint::from(100000000u32));
}
