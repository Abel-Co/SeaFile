use aes::cipher::{KeyIvInit, StreamCipher};

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

lazy_static! {
    pub static ref key: Vec<u8> = base64::decode("QR5aWivlbewl+9mGtCs3zA==").unwrap();
    pub static ref iv: Vec<u8> = base64::decode("NsKGlMazY0gjMkcVaAdJRg==").unwrap();
}

pub fn aes(plaintext: &str) -> String {
    let mut buf = plaintext.as_bytes().to_vec();
    let mut cipher = Aes128Ctr64LE::new((&key[..]).into(), (&iv[..]).into());
    cipher.apply_keystream(&mut buf);   // 完成加密
    hex::encode(buf)
}

pub fn unaes(ciphertext: &str) -> String {
    let mut buf = hex::decode(ciphertext).unwrap();
    let mut cipher = Aes128Ctr64LE::new((&key[..]).into(), (&iv[..]).into());
    buf.chunks_mut(3).for_each(|chunk| cipher.apply_keystream(chunk));
    String::from_utf8_lossy(&buf).to_string()
}

#[cfg(test)]
pub mod test_encryption {
    use crate::module::utils::debug_aes::{aes, unaes};

    #[test]
    fn test_the_new_aes_unaes() {
        let plaintext = "hello world! this is my plaintext.";
        let ciphertext = "940ac6da4641be05aa2665a736f36c9799b697554ba18bf1939725f0a75730422727";

        let cipher = aes(plaintext);
        println!(">>> cipher: {:?}", cipher);

        let plaintext = unaes(ciphertext);
        println!(">>> plaintext: {:?}", plaintext);
    }
}