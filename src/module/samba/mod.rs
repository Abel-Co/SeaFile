use std::process::Command;

use regex::Regex;

use crate::do_loop;

lazy_static! {
    pub static ref SAMBA_STATUS: Regex = Regex::new("crashed|stopped").unwrap();
}

/**
 * alpine samba 服务守护
 */
pub async fn daemon_smb() {
    let mut samba_status = String::new();
    do_loop!({
        if let Ok(output) = Command::new("sh").arg("-c").arg("rc-status | grep samba").output() {
            samba_status = String::from_utf8_lossy(&output.stdout).to_string();
            log::info!("rc-status samba:{}", samba_status.replacen("   ", " ", 22));
            if let Some(_) = SAMBA_STATUS.find(&samba_status) {
                log::info!("smb died, now restart ...");
                if let Ok(output) = Command::new("rc-service").arg("samba").arg("restart").output() {
                    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                    log::info!("rc-service samba restart ...\n{}", output_str)
                }
            }
        }
    } while SAMBA_STATUS.is_match(&samba_status) )
}

/**
 * 添加系统smb账户
 */
pub async fn create(account: &str, password: &str) -> Result<(), String> {
    if cfg!(target_os = "linux") {
        let output = Command::new("cat").arg("/etc/os-release").output();
        if String::from_utf8_lossy(&output.unwrap().stdout).contains("Alpine Linux") {
            if let Err(e) = Command::new("adduser").arg("-D").arg(account).output() {
                return Err(e.to_string());
            }
            let shell = format!("echo -e '{}\n{}\n' | smbpasswd -a -s {}", password, password, account);
            if let Err(e) = Command::new("sh").arg("-c").arg(shell).output() {
                return Err(e.to_string());
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
