mod vigenere {
    const A: u8 = b'A'; 
    const a: u8 = b'a'; 
    const ALPHA_LEN: u8 = 26;

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

        let mut key_iter = key.chars()
            .map(|ch| ch as u8)
            .cycle();

        plain_text
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphabetic() {
                    let first = if ch.is_ascii_lowercase() { a } else { A };
                    let shift = key_iter.next().unwrap() - a;

                    (first + (ch as u8 + shift - first) % ALPHA_LEN) as char
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

        let mut key_iter = key.chars()
            .map(|ch| ch as u8)
            .cycle();

        ciphertext
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphabetic() {
                    let first = if ch.is_ascii_lowercase() { a } else { A };
                    let mut shift = (ch as i16) - (key_iter.next().unwrap() as i16);
                    if shift < 0 {shift += ALPHA_LEN as i16;}

                    (first + (shift as u8) % ALPHA_LEN) as char
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
