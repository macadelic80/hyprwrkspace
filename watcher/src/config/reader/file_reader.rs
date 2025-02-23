use std::fs;
use std::thread;
use std::time::Duration;

pub fn read_file(path: String) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(file) => Ok(file),
        Err(err) => Err(format!("Err read_file {}", err))
    }
}

pub fn wait_for_file_ready(path: &str, max_attempts: u32) -> bool {
    for index in 0..max_attempts {
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > 0 {
                return true;
            } else {
                println!("Pas bon pour la: {}: Metadata {}", index, metadata.len());
            }
        } else {
            println!("Pas bon pour la: {}: Metadata pas bon", index);
        }
        thread::sleep(Duration::from_millis(50));
    }
    false
}
