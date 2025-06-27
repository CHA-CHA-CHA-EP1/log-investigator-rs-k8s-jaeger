use crate::models::LogEntry;
use std::fs;

pub fn load_logs(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut logs = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<LogEntry>(line) {
            Ok(log_entry) => {
                logs.push(log_entry);
            }
            Err(e) => {
                eprintln!("⚠️  Line {}: Failed to parse JSON - {}", line_num + 1, e);
                eprintln!("    Content: {}", line);
            }
        }
    }

    println!("📊 Parsed {} lines successfully", logs.len());
    Ok(logs)
}