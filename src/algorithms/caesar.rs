pub fn encrypt(text: String) -> String {
    crate::algorithms::rot::encrypt(3, text)
}
pub fn decrypt(text: String) -> String {
    crate::algorithms::rot::decrypt(3, text)
}
