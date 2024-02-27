extern crate ethnum;
use ethnum::U256;

fn modexp(base: U256, exp: U256, modulus: U256) -> U256 {
    if exp == 0 {
        return U256::new(1);
    }
    let mut res = modexp(base, exp / 2, modulus);
    res = (res * res) % modulus;
    if exp % 2 == 1 {
        res = (res * base) % modulus;
    }
    return res;
}

fn main() {
    match (
        U256::from_str_prefixed("123456789"),
        U256::from_str_prefixed("987654321"),
        U256::from_str_prefixed("0xffffffffffffffffffff"),
    ) {
        (Ok(base), Ok(exp), Ok(modulus)) => println!("Test: {}^{} mod {} = {}",base, exp, modulus, modexp(base, exp, modulus)),
        _ => panic!("Error parsing input"),
    }
}
