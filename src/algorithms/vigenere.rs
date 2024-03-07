pub fn encrypt(key: &[u8], text: &[u8]) -> Vec<u8> {
    // key must be all uppercase
    let mut key_index: usize = 0;
    text.iter()
        .map(|chr| {
            if (65..=90).contains(chr) {
                let step = key[key_index] - 65;
                key_index = (key_index + 1) % key.len();
                (chr - 65 + step) % 26 + 65
            } else if (97..=122).contains(chr) {
                let step = key[key_index] - 65;
                key_index = (key_index + 1) % key.len();
                (chr - 97 + step) % 26 + 97
            } else {
                *chr
            }
        })
        .collect()
}
pub fn decrypt(key: &[u8], text: &[u8]) -> Vec<u8> {
    // key must be all uppercase
    let mut key_index: usize = 0;
    text.iter()
        .map(|chr| {
            if (65..=90).contains(chr) {
                let step = key[key_index] - 65;
                key_index = (key_index + 1) % key.len();
                (chr - 65 + 26 - step) % 26 + 65
            } else if (97..=122).contains(chr) {
                let step = key[key_index] - 65;
                key_index = (key_index + 1) % key.len();
                (chr - 97 + 26 - step) % 26 + 97
            } else {
                *chr
            }
        })
        .collect()
}
