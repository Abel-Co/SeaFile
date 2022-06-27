
use crypto::digest::Digest;
use crypto::md5::Md5;

/**
 * md5 加密工具类
 * Demo: encryption::md5(password) == user.password.clone().unwrap()
 */
pub fn md5<S: Into<String>>(input: S) -> String {
  let mut md5 = Md5::new();
  md5.input_str(&input.into());
  md5.result_str()
}