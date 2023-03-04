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

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

lazy_static! {
    pub static ref KEY: String = String::from("BBBBBBBBBBBBBBBB");
}

// key: &String
pub fn aes(plaintext: &str) -> String {
    let iv = [0x24; 16];
    let mut buf = plaintext.as_bytes().to_vec();
    let mut cipher = Aes128Ctr64LE::new(KEY.as_bytes().into(), &iv.into());
    cipher.apply_keystream(&mut buf); // 完成加密
    hex::encode(buf)
}

// key: &String
pub fn unaes(ciphertext: &str) -> String {
    let iv = [0x24; 16];
    let mut buf = hex::decode(ciphertext).unwrap();
    let mut cipher = Aes128Ctr64LE::new(KEY.as_bytes().into(), &iv.into());
    buf.chunks_mut(3).for_each(|chunk| cipher.apply_keystream(chunk));
    String::from_utf8_lossy(&buf).to_string()
}

// pub fn rng

// -------------------------------------------------------------------------------------------------
#[cfg(test)]
pub mod test_encryption {
    use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};

    use crate::module::utils::encryption::{aes, unaes};

    type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

    #[test]
    #[allow(unused)]
    fn test_aes_unaes() {
        let key = String::from_utf8_lossy(&[0x42; 16]);
        let key = key.as_ref();
        let key = "BBBBBBBBBBBBBBBB".to_string();
        let plaintext = "hello world! this is my plaintext.";
        // let cipher = aes(&key, plaintext);
        let cipher = aes(plaintext);
        println!(">>> cipher: {:?}", cipher);

        let ciphertext = "3357121ebb5a29468bd861467596ce3da59bdee42dcc0614dea955368d8a5dc0cad4";
        // let plaintext = unaes(&key, ciphertext);
        let plaintext = unaes(ciphertext);
        println!(">>> plaintext: {:?}", plaintext);
    }

    #[test]
    fn test_aes_origin() {
        let key = [0x42; 16];
        let iv = [0x24; 16];
        let plaintext = *b"hello world! this is my plaintext.";
        let ciphertext = hex::decode("3357121ebb5a29468bd861467596ce3da59bdee42dcc0614dea955368d8a5dc0cad4").unwrap();

        // encrypt in-place
        let mut buf = plaintext.to_vec();
        let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
        cipher.apply_keystream(&mut buf); // 完成加密
        println!("{:?}, {:?}", (buf[..] == ciphertext[..]), buf);

        // CTR mode can be used with streaming messages
        let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
        for chunk in buf.chunks_mut(3) {
            cipher.apply_keystream(chunk);
        }
        println!("{:?}, {:?}", (buf[..] == plaintext[..]), String::from_utf8_lossy(&buf));


        // -----------------------------------------------------------------------------------------
        // CTR mode supports seeking. The parameter is zero-based _bytes_ counter (not _blocks_).
        cipher.seek(0u32);

        // encrypt/decrypt from buffer to buffer
        // buffer length must be equal to input length
        let mut buf1 = [0u8; 34];
        cipher.apply_keystream_b2b(&plaintext, &mut buf1).unwrap();
        assert_eq!(buf1[..], ciphertext[..]);

        let mut buf2 = [0u8; 34];
        cipher.seek(0u32);
        cipher.apply_keystream_b2b(&buf1, &mut buf2).unwrap();
        assert_eq!(buf2[..], plaintext[..]);
    }
}