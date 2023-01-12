use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("sh").arg("-c").arg(r"echo -e '123456\n123456\n' | smbpasswd -a -s abel").output();
    let output = match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(err) => err.to_string()
    };
    println!("{:?}", output);
}