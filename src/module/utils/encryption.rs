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

use std::sync::Arc;

use aes::cipher::{KeyIvInit, StreamCipher};
use base64::{Engine as _, engine::general_purpose, engine::general_purpose::STANDARD as BASE64};
use once_cell::sync::OnceCell;
use rbatis::crud::{CRUD, Skip};
use rbson::Bson;

use crate::boot::c::RB;
use crate::module::utils::{Base, BaseLabel};
use crate::module::utils::BaseLabel::GLOBAL;

pub static AES_CONF: OnceCell<Arc<(Vec<u8>, Vec<u8>)>> = OnceCell::new();

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

pub fn aes(plaintext: &str) -> String {
    let mut buf = plaintext.as_bytes().to_vec();
    let (key, iv): &(Vec<u8>, Vec<u8>) = AES_CONF.get().unwrap();
    let mut cipher = Aes128Ctr64LE::new(key[..].into(), iv[..].into());
    cipher.apply_keystream(&mut buf);   // 完成加密
    BASE64.encode(buf)
}

pub fn unaes(ciphertext: &str) -> String {
    let (key, iv): &(Vec<u8>, Vec<u8>) = AES_CONF.get().unwrap();
    let mut cipher = Aes128Ctr64LE::new(key[..].into(), iv[..].into());
    let mut buf = general_purpose::STANDARD.decode(ciphertext).unwrap();
    buf.chunks_mut(3).for_each(|chunk| cipher.apply_keystream(chunk));
    String::from_utf8_lossy(&buf).to_string()
}

pub async fn init_aes_conf() {
    if RB.fetch_count_by_wrapper::<Base>(RB.new_wrapper()).await.unwrap() == 0 {
        RB.save(&Base::new(), &[Skip::Value(Bson::Null)]).await.unwrap().rows_affected;
    }
    let c = RB.fetch_by_column::<Base, BaseLabel>("name", GLOBAL).await.unwrap();
    let key_iv = (BASE64.decode(c.key).unwrap(), BASE64.decode(c.iv).unwrap());
    assert!(AES_CONF.set(Arc::new(key_iv)).is_ok())
}

#[cfg(test)]
pub mod test_encryption {
    use crate::module::utils::encryption::{aes, unaes};

    #[test]
    fn test_the_new_aes_unaes() {
        // let plaintext = "hello world! this is my plaintext.";
        // // let ciphertext = "940ac6da4641be05aa2665a736f36c9799b697554ba18bf1939725f0a75730422727";    // hex
        // let ciphertext = "lArG2kZBvgWqJmWnNvNsl5m2l1VLoYvxk5cl8KdXMEInJw==";    // base64
        //
        // let cipher = aes(plaintext);
        // println!(">>> cipher: {:?}", cipher);
        //
        // let plaintext = unaes(ciphertext);
        // println!(">>> plaintext: {:?}", plaintext);
    }
}