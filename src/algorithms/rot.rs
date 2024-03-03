pub fn encrypt(key: u8, text: String) -> String {
    let mut to_return: String = String::new();
    for chr in text.bytes() {
        if (65..=90).contains(&chr) {
            let encrypted: &str = &(((chr - 65 + (key) % 26) % 26 + 65) as char).to_string();
            to_return += encrypted;
        } else if (97..=122).contains(&chr) {
            let encrypted: &str = &(((chr - 97 + (key) % 26) % 26 + 97) as char).to_string();
            to_return += encrypted;
        } else {
            to_return += &(chr as char).to_string();
        }
    }
    to_return
}
pub fn decrypt(key: u8, text: String) -> String {
    let mut to_return: String = String::new();
    for chr in text.bytes() {
        if (65..=90).contains(&chr) {
            let encrypted: &str = &(((chr - 65 + 26 - (key) % 26) % 26 + 65) as char).to_string();
            to_return += encrypted;
        } else if (97..=122).contains(&chr) {
            let encrypted: &str = &(((chr - 97 + 26 - (key) % 26) % 26 + 97) as char).to_string();
            to_return += encrypted;
        } else {
            to_return += &(chr as char).to_string();
        }
    }
    to_return
}
