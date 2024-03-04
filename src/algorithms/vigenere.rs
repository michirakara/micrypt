pub fn encrypt(key: Vec<u8>, text: Vec<u8>) -> Vec<u8> {
    // key must be all uppercase
    let mut to_return: Vec<u8> = Vec::new();

    let mut key_index: usize = 0;
    for chr in text {
        if (65..=90).contains(&chr) {
            let step: u8 = key[key_index] - 65;
            to_return.push((chr - 65 + step) % 26 + 65);
            key_index = (key_index + 1) % key.len();
        } else if (97..=122).contains(&chr) {
            let step: u8 = key[key_index] - 65;
            to_return.push((chr - 97 + step) % 26 + 97);
            key_index = (key_index + 1) % key.len();
        } else {
            to_return.push(chr);
        }
    }
    to_return
}
pub fn decrypt(key: Vec<u8>, text: Vec<u8>) -> Vec<u8> {
    // key must be all uppercase
    let mut to_return: Vec<u8> = Vec::new();

    let mut key_index: usize = 0;
    for chr in text {
        if (65..=90).contains(&chr) {
            let step: u8 = key[key_index] - 65;
            to_return.push((chr - 65 + 26 - step) % 26 + 65);
            key_index = (key_index + 1) % key.len();
        } else if (97..=122).contains(&chr) {
            let step: u8 = key[key_index] - 65;
            to_return.push((chr - 97 + 26 - step) % 26 + 97);
            key_index = (key_index + 1) % key.len();
        } else {
            to_return.push(chr);
        }
    }
    to_return
}