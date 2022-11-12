use std::process::Command;
use crate::do_loop;


pub async fn daemon_smb() {
    let mut samba_status = String::new();
    do_loop!({
        if let Ok(output) = Command::new("rc-status").arg("|").arg("grep").arg("samba").output() {
            samba_status = String::from_utf8_lossy(&output.stdout).to_string();
            log::info!("samba-status:{}", samba_status);
            if samba_status.find("crashed|stopped").unwrap_or(0) > 0 {
                if let Ok(output) = Command::new("rc-service").arg("samba").arg("restart").output() {
                    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                    println!("{}", output_str)
                }
            }
        }
    } while samba_status.find("crashed|stopped").unwrap_or(0) > 0 );
}


