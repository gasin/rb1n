use crate::crypto::Rot13;

pub fn attack(enc: &str) -> String {
    Rot13::decrypt(enc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attack_base() {
        let plain = "Hoge";
        let enc = Rot13::encrypt(plain);
        assert_eq!(attack(&enc), plain)
    }

    #[test]
    fn attack_not_alphabet() {
        let plain = "H0ge_!";
        let enc = Rot13::encrypt(plain);
        assert_eq!(attack(&enc), plain)
    }
}
