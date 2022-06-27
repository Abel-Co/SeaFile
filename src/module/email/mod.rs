use lettre::{Message, SmtpTransport, Transport};

pub struct Email {
    pub to: String,
    pub title: String,
    pub body: String,
}

pub fn send(email: &Email) -> Result<(), String> {
    let email = Message::builder()
        .from("Aura.Co <auraco@126.com>".parse().unwrap())  // 发件人
        .to(email.to.as_str().parse().unwrap())  // 收件人
        .subject(email.title.as_str())  // 主题
        .body(email.body.to_string())          // 邮件内容
        .unwrap();
    match mailer().send(&email) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(format!("Email send failure: {}", e))
        }
    }
}

pub fn send_o(str: &str) -> Result<(), String> {
    let email = Message::builder()
        .from("Aura.Co <auraco@126.com>".parse().unwrap())  // 发件人
        .to("Xu <xuguangyuansh@126.com>".parse().unwrap())  // 收件人
        .subject("Happy new year")  // 主题
        .body("Be happy!".to_string() + str)          // 邮件内容
        .unwrap();
    match mailer().send(&email) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(format!("Email send failure: {}", e))
        }
    }
}

/**
 * 创建 mailer 对象
 */
fn mailer() -> SmtpTransport {
    use lettre::transport::smtp::authentication::Credentials;

    // 邮件服务器账号：
    // username 需包含 @xxx.com 等后缀
    // password 邮箱 -> 设置，开启 smtp -> 得到 授权密码
    let creds = Credentials::new("auraco@126.com".to_string(), "VWSPYZKQCFJJESIN".to_string());

    // Open a remote connection to gmail
    SmtpTransport::relay("smtp.126.com").unwrap().credentials(creds).build()
}

/*pub fn send_origin() {
    use lettre::{Message, SmtpTransport, Transport};
    use lettre::transport::smtp::authentication::Credentials;

    let email = Message::builder()
        .from("Aura.Co <auraco@126.com>".parse().unwrap())  // 发件人
        .to("Xu <xuguangyuansh@126.com>".parse().unwrap())  // 收件人
        .subject("Happy new year")  // 主题
        .body("Be happy!")          // 邮件内容
        .unwrap();

    // 邮件服务器账号：
    // username 需包含 @xxx.com 等后缀
    // password 邮箱 -> 设置，开启 smtp -> 得到 授权密码
    let creds = Credentials::new("auraco@126.com".to_string(), "Smtp授权密码".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.126.com").unwrap().credentials(creds).build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => log::debug!("Email sent successfully!"),
        Err(e) => log::error!("Could not send email: {:?}", e),
    }
}*/