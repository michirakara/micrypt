pub fn encrypt(text: String) -> String {
    let mut to_return: String = String::new();
    for chr in text.bytes() {
        if (65..=90).contains(&chr) {
            let encrypted: &str = &(((chr - 65 + 3) % 26 + 65) as char).to_string();
            to_return += encrypted;
        } else if (97..=122).contains(&chr) {
            let encrypted: &str = &(((chr - 97 + 3) % 26 + 97) as char).to_string();
            to_return += encrypted;
        } else {
            to_return += &(chr as char).to_string();
        }
    }
    to_return
}
pub fn decrypt(text: String) -> String {
    let mut to_return: String = String::new();
    for chr in text.bytes() {
        if (65..=90).contains(&chr) {
            let encrypted: &str = &(((chr - 65 + 23) % 26 + 65) as char).to_string();
            to_return += encrypted;
        } else if (97..=122).contains(&chr) {
            let encrypted: &str = &(((chr - 97 + 23) % 26 + 97) as char).to_string();
            to_return += encrypted;
        } else {
            to_return += &(chr as char).to_string();
        }
    }
    to_return
}