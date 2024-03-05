pub fn encrypt(key: u8, text: Vec<u8>) -> Vec<u8> {
    let mut to_return: Vec<u8> = Vec::new();
    for chr in text {
        if (65..=90).contains(&chr) {
            to_return.push((chr - 65 + key) % 26 + 65);
        } else if (97..=122).contains(&chr) {
            to_return.push((chr - 97 + key) % 26 + 97);
        } else {
            to_return.push(chr);
        }
    }
    to_return
}
pub fn decrypt(key: u8, text: Vec<u8>) -> Vec<u8> {
    let mut to_return: Vec<u8> = Vec::new();
    for chr in text {
        if (65..=90).contains(&chr) {
            to_return.push((chr - 65 + 26 - key) % 26 + 65);
        } else if (97..=122).contains(&chr) {
            to_return.push((chr - 97 + 26 - key) % 26 + 97);
        } else {
            to_return.push(chr);
        }
    }
    to_return
}
