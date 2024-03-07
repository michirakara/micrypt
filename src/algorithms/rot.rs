pub fn encrypt(key: u8, text: &[u8]) -> Vec<u8> {
    text.iter()
        .map(|chr| {
            if (65..=90).contains(chr) {
                (chr - 65 + key) % 26 + 65
            } else if (97..=122).contains(chr) {
                (chr - 97 + key) % 26 + 97
            } else {
                *chr
            }
        })
        .collect()
}
pub fn decrypt(key: u8, text: &[u8]) -> Vec<u8> {
    text.iter()
        .map(|chr| {
            if (65..=90).contains(chr) {
                (chr - 65 + 26 - key) % 26 + 65
            } else if (97..=122).contains(chr) {
                (chr - 97 + 26 - key) % 26 + 97
            } else {
                *chr
            }
        })
        .collect()
}
