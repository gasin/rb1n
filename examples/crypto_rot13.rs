extern crate rb1n;

fn main() {
    let plain = "Hello, world!";
    println!("plain: {}", plain);

    let encrypted = rb1n::crypto::Rot13::encrypt(plain);
    println!("encrypted: {}", encrypted);

    let decrypted = rb1n::crypto::Rot13::decrypt(&encrypted);
    println!("decrypted: {}", decrypted);
}
