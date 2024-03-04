pub fn encrypt(text: Vec<u8>) -> Vec<u8> {
    crate::algorithms::rot::encrypt(3, text)
}
pub fn decrypt(text: Vec<u8>) -> Vec<u8> {
    crate::algorithms::rot::decrypt(3, text)
}
