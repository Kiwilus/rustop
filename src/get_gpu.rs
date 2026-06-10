use std::process::Command;

pub fn get_gpu() -> String {
    // Run "lspci" and filter for VGA (works on Linux)
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i vga")
        .output();

    match output {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout).to_string();
            // Nimm nur den Teil nach dem ":" 
            text.split(':')
                .last()
                .unwrap_or("Unknown")
                .trim()
                .to_string()
        }
        Err(_) => "Unknown".to_string()
    }
}