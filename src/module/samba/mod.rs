use std::process::Command;
use crate::do_loop;
use regex::Regex;

lazy_static! {
    pub static ref SAMBA_STATUS: Regex = Regex::new("crashed|stopped").unwrap();
}

pub async fn daemon_smb() {
    let mut samba_status = String::new();
    do_loop!({
        if let Ok(output) = Command::new("sh").arg("-c").arg("rc-status | grep samba").output() {
            samba_status = String::from_utf8_lossy(&output.stdout).to_string();
            log::info!("rc-status samba:{}", samba_status);
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


