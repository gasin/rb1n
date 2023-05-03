use crate::math::num_trait::*;

pub fn mod_pow<
    T: PartialOrd + One + Zero + std::ops::Rem<T, Output = T> + std::ops::Div<T, Output = T> + Copy,
>(
    mut base: T,
    mut exp: T,
    modulus: T,
) -> T {
    if modulus == T::one() {
        return T::zero();
    }

    let mut result = T::one();
    let two = T::one() + T::one();

    base = base % modulus;

    while exp > T::zero() {
        if exp % two == T::one() {
            result = (result * base) % modulus;
        }

        exp = exp / two;
        base = (base * base) % modulus;
    }

    result
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
