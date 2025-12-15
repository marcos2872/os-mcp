use anyhow::{Context, Result};
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn log_command(log_file: &str, command: &str, status: &str, details: Option<&str>) -> Result<()> {
    let config_dir = crate::config::get_config_dir()?;
    let log_path = config_dir.join(log_file);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .context("Failed to open audit log file")?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let details_str = details.unwrap_or("-");

    writeln!(
        file,
        "[{}] [{}] Command: \"{}\" | Details: {}",
        timestamp, status, command, details_str
    )
    .context("Failed to write to audit log")?;

    Ok(())
}
