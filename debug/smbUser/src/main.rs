use std::process::Command;

fn main() {
	let output = Command::new("echo").arg("-e").arg("123456\n123456\n").arg("| smbpasswd").arg("-a -s").arg("abel").output();
    let output = match output {
	    Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
    	Err(err) => err.to_string()
    };
    println!("{:?}", output);
}