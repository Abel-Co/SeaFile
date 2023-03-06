// use crypto::digest::Digest;
// use crypto::md5::Md5;

// /**
//  * md5 加密工具类
//  * Demo: encryption::md5(password) == user.password.clone().unwrap()
//  */
// pub fn md5<S: Into<String>>(input: S) -> String {
//     let mut md5 = Md5::new();
//     md5.input_str(&input.into());
//     md5.result_str()
// }

use aes::cipher::{KeyIvInit, StreamCipher};
use base64::{Engine as _, engine::general_purpose};

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

lazy_static! {
    pub static ref KEY: Vec<u8> = general_purpose::STANDARD.decode("QR5aWivlbewl+9mGtCs3zA==").unwrap();
    pub static ref IV: Vec<u8> = general_purpose::STANDARD.decode("NsKGlMazY0gjMkcVaAdJRg==").unwrap();
}

pub fn aes(plaintext: &str) -> String {
    let mut buf = plaintext.as_bytes().to_vec();
    let mut cipher = Aes128Ctr64LE::new((&KEY[..]).into(), (&IV[..]).into());
    cipher.apply_keystream(&mut buf);   // 完成加密
    general_purpose::STANDARD.encode(buf)
}

pub fn unaes(ciphertext: &str) -> String {
    let mut buf = general_purpose::STANDARD.decode(ciphertext).unwrap();
    let mut cipher = Aes128Ctr64LE::new((&KEY[..]).into(), (&IV[..]).into());
    buf.chunks_mut(3).for_each(|chunk| cipher.apply_keystream(chunk));
    String::from_utf8_lossy(&buf).to_string()
}

#[cfg(test)]
pub mod test_encryption {
    use crate::module::utils::debug_aes::{aes, unaes};

    #[test]
    fn test_the_new_aes_unaes() {
        let plaintext = "hello world! this is my plaintext.";
        // let ciphertext = "940ac6da4641be05aa2665a736f36c9799b697554ba18bf1939725f0a75730422727";    // hex
        let ciphertext = "lArG2kZBvgWqJmWnNvNsl5m2l1VLoYvxk5cl8KdXMEInJw==";    // base64

        let cipher = aes(plaintext);
        println!(">>> cipher: {:?}", cipher);

        let plaintext = unaes(ciphertext);
        println!(">>> plaintext: {:?}", plaintext);
    }
}