use std::fs::metadata;
use owo_colors::OwoColorize;
use std::path::Path;

// Convert bytes to larger units
pub fn convert_bytes(bytes: usize) -> String {
    if bytes < 1_000 {
        format!("{} B", bytes)
    } else if bytes < 1_000_000 {
        format!("{:.2} KB", bytes as f32 / 1_000.0)
    } else if bytes < 1_000_000_000 {
        format!("{:.2} MB", bytes as f32 / 1_000_000.0)
    } else {
        format!("{:.2} GB", bytes as f32 / 1_000_000_000.0)
    }
}

// Get compression ratio of file
pub fn percent_change(raw: usize, compressed: usize) -> Option<f32> {
    if raw == 0{
        return None;
    }
    let diff = raw.checked_sub(compressed)?;
    Some((diff as f32 / raw as f32) * 100.0)
}

// Checks if file is valid and outputs errors otherwise, this is used in main()
pub fn check_path(file: &str) -> bool{
    if !Path::new(&file).exists() {
        println!(
            "{}{}{}",
            "File '".red().bold(),
            file.red().bold(),
            "' does not exist; Skipping".bold().red()
        );
        return false;
    }
    match metadata(file) {
        Ok(md) => {
            if md.is_dir() {
                println!(
                    "{}{}{}",
                    "'".red().bold(),
                    file.red().bold(),
                    "' is a directory; Skipping".red().bold()
                );
                return false;
            }

            let extension = Path::new(file).extension().unwrap().to_str().unwrap();
            if extension != "zip" {
                println!("{} {}", file.red().bold(), "has an incompatible file extension; Skipping".red().bold());
                return false;
            }
        }
        Err(e) => {
            println!("{} {} '{}' {}", "Error getting metadata for file:".red().bold(), file.bold(), e, "Skipping".red().bold()); 
            return false;
        }
    }
    true
}
