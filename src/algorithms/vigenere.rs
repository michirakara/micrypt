pub fn encrypt(key: String, text: String) -> String {
    // key must be all uppercase
    let mut to_return: String = String::new();

    let mut key_index: usize = 0;
    for chr in text.bytes() {
        if (65..=90).contains(&chr) {
            let step: u8 = key.chars().nth(key_index).unwrap() as u8 - 65;
            let encrypted: &str = &(((chr - 65 + step) % 26 + 65) as char).to_string();
            to_return += encrypted;
            key_index = (key_index + 1) % key.len();
        } else if (97..=122).contains(&chr) {
            let step: u8 = key.chars().nth(key_index).unwrap() as u8 - 65;
            let encrypted: &str = &(((chr - 97 + step) % 26 + 97) as char).to_string();
            to_return += encrypted;
            key_index = (key_index + 1) % key.len();
        } else {
            to_return += &(chr as char).to_string();
        }
    }
    to_return
}
pub fn decrypt(key: String, text: String) -> String {
    // key must be all uppercase
    let mut to_return: String = String::new();

    let mut key_index: usize = 0;
    for chr in text.bytes() {
        if (65..=90).contains(&chr) {
            let step: u8 = key.chars().nth(key_index).unwrap() as u8 - 65;
            let encrypted: &str = &(((chr - 65 + 26 - step) % 26 + 65) as char).to_string();
            to_return += encrypted;
            key_index = (key_index + 1) % key.len();
        } else if (97..=122).contains(&chr) {
            let step: u8 = key.chars().nth(key_index).unwrap() as u8 - 65;
            let encrypted: &str = &(((chr - 97 + 26 - step) % 26 + 97) as char).to_string();
            to_return += encrypted;
            key_index = (key_index + 1) % key.len();
        } else {
            to_return += &(chr as char).to_string();
        }
    }
    to_return
}
