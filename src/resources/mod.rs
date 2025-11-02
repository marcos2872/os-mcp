use anyhow::{Context, Result};
use rmcp::model::{Annotated, RawResource};
use std::process::Command;
use sysinfo::System;

/// Lista todos os resources disponíveis
pub fn list_resources() -> Vec<Annotated<RawResource>> {
    vec![
        Annotated::new(
            RawResource {
                uri: "linux://logs/system".to_string(),
                name: "System Logs".to_string(),
                title: Some("System Logs".to_string()),
                description: Some(
                    "Últimas 100 linhas dos logs do sistema (journalctl)".to_string(),
                ),
                mime_type: Some("text/plain".to_string()),
                size: None,
                icons: None,
            },
            None,
        ),
        Annotated::new(
            RawResource {
                uri: "linux://logs/auth".to_string(),
                name: "Authentication Logs".to_string(),
                title: Some("Authentication Logs".to_string()),
                description: Some("Últimas 50 linhas dos logs de autenticação SSH".to_string()),
                mime_type: Some("text/plain".to_string()),
                size: None,
                icons: None,
            },
            None,
        ),
        Annotated::new(
            RawResource {
                uri: "linux://config/network".to_string(),
                name: "Network Configuration".to_string(),
                title: Some("Network Configuration".to_string()),
                description: Some("Configuração de rede (ip addr show)".to_string()),
                mime_type: Some("text/plain".to_string()),
                size: None,
                icons: None,
            },
            None,
        ),
        Annotated::new(
            RawResource {
                uri: "linux://processes/top".to_string(),
                name: "Top Processes".to_string(),
                title: Some("Top Processes".to_string()),
                description: Some("Top 10 processos por uso de memória".to_string()),
                mime_type: Some("text/plain".to_string()),
                size: None,
                icons: None,
            },
            None,
        ),
        Annotated::new(
            RawResource {
                uri: "linux://system/status".to_string(),
                name: "System Status".to_string(),
                title: Some("System Status".to_string()),
                description: Some("Status geral do sistema (CPU, memória, uptime)".to_string()),
                mime_type: Some("text/plain".to_string()),
                size: None,
                icons: None,
            },
            None,
        ),
    ]
}

/// Lê o conteúdo de um resource
pub async fn read_resource(uri: &str) -> Result<String> {
    match uri {
        "linux://logs/system" => {
            let output = Command::new("journalctl")
                .args(["-n", "100", "--no-pager"])
                .output()
                .context("Failed to execute journalctl")?;

            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        "linux://logs/auth" => {
            let output = Command::new("journalctl")
                .args(["-u", "ssh", "-n", "50", "--no-pager"])
                .output()
                .context("Failed to execute journalctl")?;

            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        "linux://config/network" => {
            let output = Command::new("ip")
                .args(["addr", "show"])
                .output()
                .context("Failed to execute ip")?;

            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        "linux://processes/top" => {
            let output = Command::new("ps")
                .args(["aux", "--sort=-%mem"])
                .output()
                .context("Failed to execute ps")?;

            let full_output = String::from_utf8_lossy(&output.stdout).to_string();
            let lines: Vec<&str> = full_output.lines().collect();
            Ok(if lines.len() > 11 {
                lines[..11].join("\n")
            } else {
                full_output
            })
        }
        "linux://system/status" => {
            let mut sys = System::new_all();
            sys.refresh_all();

            let total_mem = sys.total_memory();
            let used_mem = sys.used_memory();
            let mem_percent = (used_mem as f64 / total_mem as f64) * 100.0;

            let cpu_count = sys.cpus().len();
            let cpu_usage: f32 =
                sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpu_count as f32;

            Ok(format!(
                "=== System Status ===\n\
                CPU Usage: {:.1}%\n\
                Memory: {:.1}% ({} MB / {} MB)\n\
                Uptime: {} seconds",
                cpu_usage,
                mem_percent,
                used_mem / 1024 / 1024,
                total_mem / 1024 / 1024,
                System::uptime()
            ))
        }
        _ => Err(anyhow::anyhow!("Unknown resource: {}", uri)),
    }
}
