mod user;

use std::process::Command;
use crate::user::{Users, UserType};

fn main() {
    if cfg!(target_os = "linux") {
        let output = Command::new("sh").arg("-c").arg("cat /etc/os-release").output();
        if !String::from_utf8_lossy(&output.unwrap().stdout).contains("Alpine Linux") {return}
        for user in user_list() {
            let account = user.username.unwrap();
            let _ = Command::new("adduser").arg("-D").arg(&account).output();
            if UserType::User == user.user_type {
                let password = user.password.unwrap();
                // let double_passwd = format!("{}\\n{}\\n", password, password);
                // let output = Command::new("echo -e").arg(double_passwd).arg(" | smbpasswd -a -s").arg(&account).output();
                let output = Command::new("echo").arg("-e").arg("'123456\n123456\n'").arg("| smbpasswd").arg("-a -s").arg("abel").output();
                let output = match output {
                    Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
                    Err(err) => err.to_string()
                };
                println!("{:?}", output);
            }
        }
    }
}

fn user_list() -> Vec<Users> {
    vec![
        Users {
            username: Some("abel".to_string()),
            password: Some("123456".to_string()),
            user_type: UserType::Admin,
            status: Some(1),
            ..Default::default()
        },
        Users {
            username: Some("xugy".to_string()),
            password: Some("123456".to_string()),
            user_type: UserType::User,
            status: Some(1),
            ..Default::default()
        }
    ]
}