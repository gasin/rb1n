//! Rot13 Cipher

pub fn rot13(input: &str) -> String {
    input.chars()
         .map(|c: char| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() {b'a'} else {b'A'};
                ((c as u8 - base + 13) % 26 + base) as char
            } else {
                c
            }
         })
         .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(rot13("Hoge"), "Ubtr")
    }

    #[test]
    fn not_alphabet() {
        assert_eq!(rot13("H0ge_!"), "U0tr_!")
    }

}
