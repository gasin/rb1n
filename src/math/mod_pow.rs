pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;

    base = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }

        exp = exp >> 1;
        base = (base * base) % modulus;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(mod_pow(3, 2, 7), 2)
    }

    #[test]
    fn mod_one() {
        assert_eq!(mod_pow(2, 2, 1), 0)
    }
}
