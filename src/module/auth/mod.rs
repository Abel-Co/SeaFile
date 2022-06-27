use bcrypt::BcryptResult;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod bs;
pub mod dao;

#[derive(Debug, Default, Validate, Deserialize, Serialize)]
pub struct Login {
    #[validate(required, length(max = 50, message = "username must be less than 50 chars."))]
    pub username: Option<String>,
    #[validate(required, length(min = 6, message = "password must be more than 6 chars."))]
    pub password: Option<String>,
}

impl Login {
    /// 验证密码
    pub fn verify(&self, hash: &str) -> BcryptResult<bool> {
        bcrypt::verify(self.password.as_ref().unwrap().as_bytes(), hash)
    }
}

/**
 * 生成加密密码
 * PBKDF2 < bcrypt < scrypt
 */
pub fn passhash(pass: &str) -> String {
    // let namedpass = format!("{}{}", name, pass);
    bcrypt::hash(pass.as_bytes(), bcrypt::DEFAULT_COST).unwrap()
}

/// 测试模块
#[cfg(test)]
mod auth_mod_test {
    use crate::module::auth;
    use crate::module::auth::Login;

    /// 测试 - 生成密码
    #[test]
    fn test_passhash() {
        let password = "123456";
        let passhash = auth::passhash(password);
        println!("{:?}", password);
    }

    /// 测试 - 密码校验
    #[test]
    fn test_passverify() {
        let login = Login {
            password: Some(String::from("123456")),
            ..Default::default()
        };
        let result = login.verify("$2b$12$sVUkyBqJXoEGvr9uquCX4OqIJoA35hdW6GkrA9V8EACuFWuuQyGx2");
        println!("{:?}", result);

        let login2 = Login {
            password: Some(String::from("1234567")),
            ..Default::default()
        };
        let result = login2.verify("$2b$12$sVUkyBqJXoEGvr9uquCX4OqIJoA35hdW6GkrA9V8EACuFWuuQyGx2");
        println!("{:?}", result)
    }
}