//! Rot13 Cipher

pub struct Rot13;

impl Rot13 {
    pub fn encrypt(plain: &str) -> String {
        plain
            .chars()
            .map(|c: char| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    ((c as u8 - base + 13) % 26 + base) as char
                } else {
                    c
                }
            })
            .collect()
    }

    pub fn decrypt(enc: &str) -> String {
        Rot13::encrypt(enc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_base() {
        assert_eq!(Rot13::encrypt("Hoge"), "Ubtr")
    }

    #[test]
    fn encrypt_not_alphabet() {
        assert_eq!(Rot13::encrypt("H0ge_!"), "U0tr_!")
    }

    #[test]
    fn decrypt_base() {
        assert_eq!(Rot13::decrypt("Ubtr"), "Hoge")
    }

    #[test]
    fn decrypt_not_alphabet() {
        assert_eq!(Rot13::decrypt("U0tr_!"), "H0ge_!")
    }
}
