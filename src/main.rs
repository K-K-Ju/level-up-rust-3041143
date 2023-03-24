mod vigenere {

    fn clean_input(input: &str) -> String {
        input.chars()
            .filter(|&c| c.is_ascii_alphabetic())
            .map(|ch| ch.to_ascii_lowercase())
            .collect()
    }

    pub fn encrypt(plain_text: &str, key: &str) -> String {
        let plain_text = clean_input(plain_text);
        let key = clean_input(key);
        let key_len = key.len();
        if key_len == 0 {return String::from(plain_text);}

        let mut index = 0;

        plain_text
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphabetic() {
                    let first = if ch.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shift = key.as_bytes()[index % key_len] - b'a';
                    index += 1;

                    (first + (ch as u8 + shift - first) % 26) as char
                } else {
                    ch
                }
            })
            .collect()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let ciphertext = clean_input(ciphertext);
        let key = clean_input(key);
        let key_len = key.len();
        if key_len == 0 {return String::from(ciphertext);}

        let mut index = 0;

        ciphertext
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphabetic() {
                    let first = if ch.is_ascii_lowercase() { b'a' } else { b'A' };
                    let mut shift = (ch as i16) - (key.as_bytes()[index % key_len] as i16);
                    if shift < 0 {shift += 26;}
                    index += 1;

                    (first + (shift as u8) % 26) as char
                } else {
                    ch
                }
            })
            .collect()
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";

    let key1 = "KEY";
    let text = "DCODE";
    // let plaintext = vigenere::decrypt(&ciphertext, key);
    // let plaintext = vigenere::encrypt(text, key1);
    let decr = vigenere::decrypt(ciphertext, key);

    println!("{decr}");
}
