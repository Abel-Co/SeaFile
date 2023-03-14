use std::process::Command;

use regex::Regex;

use crate::do_loop;
use crate::module::user;
use crate::module::utils::encryption;

lazy_static! {
    pub static ref RE_SAMBA_STATUS: Regex = Regex::new("crashed|stopped").unwrap();
    pub static ref RE_BLANK_3CHAR: Regex = Regex::new(" {3,}").unwrap();
}

/**
 * alpine samba 服务守护
 */
pub async fn daemon_smb() {
    let mut samba_status = String::new();
    do_loop!({
        if let Ok(output) = Command::new("sh").arg("-c").arg("rc-status | grep samba").output() {
            samba_status = String::from_utf8_lossy(&output.stdout).to_string();
            log::info!("rc-status samba:{}", RE_BLANK_3CHAR.replace(&samba_status, "        "));
            if let Some(_) = RE_SAMBA_STATUS.find(&samba_status) {
                log::info!("smb died, now restart ...");
                if let Ok(output) = Command::new("rc-service").arg("samba").arg("restart").output() {
                    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                    log::info!("rc-service samba restart ...\n{}", output_str)
                }
            }
        }
    } while RE_SAMBA_STATUS.is_match(&samba_status) )
}

pub async fn init_smb_account() {
    if cfg!(target_os = "linux") {
        let output = Command::new("sh").arg("-c").arg("cat /etc/os-release").output();
        if !String::from_utf8_lossy(&output.unwrap().stdout).contains("Alpine Linux") { return; }
        for user in user::dao::list().await {
            let account = user.username.unwrap();
            let _ = Command::new("adduser").arg("-D").arg(&account).output();
            if let (Some(1), Some(password)) = (user.status, user.password) {
                let password = encryption::unaes(&password);
                let shell = format!("echo -e '{}\n{}\n' | smbpasswd -a -s {}", password, password, account);
                let output = Command::new("sh").arg("-c").arg(shell).output();
                let output = match output {
                    Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
                    Err(err) => err.to_string()
                };
                log::info!("{:?}", output);
            }
        }
    }
}

/**
 * 添加系统smb账户
 */
pub async fn create(account: &str, password: Option<String>) -> Result<(), String> {
    if cfg!(target_os = "linux") {
        let output = Command::new("cat").arg("/etc/os-release").output();
        if String::from_utf8_lossy(&output.unwrap().stdout).contains("Alpine Linux") {
            if let Err(e) = Command::new("adduser").arg("-D").arg(account).output() {
                return Err(e.to_string());
            }
            if let Some(password) = password {
                let shell = format!("echo -e '{}\n{}\n' | smbpasswd -a -s {}", password, password, account);
                if let Err(e) = Command::new("sh").arg("-c").arg(shell).output() {
                    return Err(e.to_string());
                }
            }
        }
    }
    Ok(())
}

/**
 * 修改系统smb账户密码
 */
pub async fn modify_password(account: &str, password: &str) -> Result<(), String> {
    if cfg!(target_os = "linux") {
        let output = Command::new("cat").arg("/etc/os-release").output();
        if String::from_utf8_lossy(&output.unwrap().stdout).contains("Alpine Linux") {
            let shell = format!("echo -e '{}\n{}\n' | smbpasswd -a -s {}", password, password, account);
            if let Err(err) = Command::new("sh").arg("-c").arg(shell).output() {
                return Err(err.to_string());
            }
        }
    }
    Ok(())
}

/**
 * 删除系统smb账户
 */
pub async fn remove() -> Result<(), String> { Ok(()) }
