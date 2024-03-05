pub fn string_to_vec(s: String) -> Vec<u8> {
    let mut to_ret: Vec<u8> = Vec::new();
    for i in s.bytes() {
        to_ret.push(i);
    }
    to_ret
}

pub fn vec_to_string(s: Vec<u8>) -> String {
    let mut to_ret: String = String::new();
    for i in s {
        to_ret += &((i as char).to_string());
    }
    to_ret
}
