use anyhow::Result;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::schemars::JsonSchema;
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::Command;
use std::sync::Arc;
use sysinfo::{Disks, System};
use tokio::sync::Mutex;

mod windows_elevation;

/// Estrutura para os argumentos do tool de informações do sistema
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[schemars(crate = "rmcp::schemars")]
struct SystemInfoArgs {
    #[serde(default)]
    info_type: Option<String>,
}

/// Estrutura para os argumentos do tool de execução de comandos
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[schemars(crate = "rmcp::schemars")]
struct ExecuteCommandArgs {
    command: String,
    #[serde(default)]
    args: Option<Vec<String>>,
    /// Se true, usa UAC (User Account Control) para autenticação com diálogo gráfico do Windows
    #[serde(default)]
    use_elevation: Option<bool>,
}

/// Servidor MCP Windows
#[derive(Clone)]
pub struct WindowsMcpServer {
    tool_router: ToolRouter<Self>,
    system: Arc<Mutex<System>>,
}

#[tool_router]
impl WindowsMcpServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            system: Arc::new(Mutex::new(System::new_all())),
        }
    }

    /// Obtém informações do sistema Windows
    #[tool(
        description = "Obtém informações do sistema Windows como CPU, memória, discos e sistema operacional. Você pode especificar o tipo de informação: 'cpu', 'memory', 'disk', 'os' ou 'all' (padrão)."
    )]
    async fn get_system_info(
        &self,
        Parameters(args): Parameters<SystemInfoArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut sys = self.system.lock().await;
        sys.refresh_all();

        let info_type = args.info_type.as_deref().unwrap_or("all");

        let info = match info_type {
            "cpu" => {
                let cpu_info = json!({
                    "cpu_count": sys.cpus().len(),
                    "cpu_brand": sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Unknown"),
                    "cpu_usage": sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect::<Vec<_>>(),
                });
                json!({ "cpu": cpu_info })
            }
            "memory" => {
                let memory_info = json!({
                    "total_memory_bytes": sys.total_memory(),
                    "used_memory_bytes": sys.used_memory(),
                    "available_memory_bytes": sys.available_memory(),
                    "total_swap_bytes": sys.total_swap(),
                    "used_swap_bytes": sys.used_swap(),
                    "total_memory_gb": format!("{:.2}", sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                    "used_memory_gb": format!("{:.2}", sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                    "available_memory_gb": format!("{:.2}", sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                });
                json!({ "memory": memory_info })
            }
            "disk" => {
                let disks = Disks::new_with_refreshed_list();
                let disk_info: Vec<_> = disks
                    .iter()
                    .map(|disk| {
                        json!({
                            "name": disk.name().to_string_lossy(),
                            "mount_point": disk.mount_point().to_string_lossy(),
                            "total_space_bytes": disk.total_space(),
                            "available_space_bytes": disk.available_space(),
                            "total_space_gb": format!("{:.2}", disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0),
                            "available_space_gb": format!("{:.2}", disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0),
                            "file_system": disk.file_system().to_string_lossy().to_string(),
                        })
                    })
                    .collect();
                json!({ "disks": disk_info })
            }
            "os" => {
                let os_info = json!({
                    "name": System::name().unwrap_or_else(|| "Unknown".to_string()),
                    "kernel_version": System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
                    "os_version": System::os_version().unwrap_or_else(|| "Unknown".to_string()),
                    "host_name": System::host_name().unwrap_or_else(|| "Unknown".to_string()),
                });
                json!({ "os": os_info })
            }
            "all" | _ => {
                // Informações completas
                let disks = Disks::new_with_refreshed_list();
                let disk_info: Vec<_> = disks
                    .iter()
                    .map(|disk| {
                        json!({
                            "name": disk.name().to_string_lossy(),
                            "mount_point": disk.mount_point().to_string_lossy(),
                            "total_space_bytes": disk.total_space(),
                            "available_space_bytes": disk.available_space(),
                            "total_space_gb": format!("{:.2}", disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0),
                            "available_space_gb": format!("{:.2}", disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0),
                            "file_system": disk.file_system().to_string_lossy().to_string(),
                        })
                    })
                    .collect();

                json!({
                    "os": {
                        "name": System::name().unwrap_or_else(|| "Unknown".to_string()),
                        "kernel_version": System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
                        "os_version": System::os_version().unwrap_or_else(|| "Unknown".to_string()),
                        "host_name": System::host_name().unwrap_or_else(|| "Unknown".to_string()),
                    },
                    "cpu": {
                        "cpu_count": sys.cpus().len(),
                        "cpu_brand": sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Unknown"),
                        "cpu_usage": sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect::<Vec<_>>(),
                    },
                    "memory": {
                        "total_memory_bytes": sys.total_memory(),
                        "used_memory_bytes": sys.used_memory(),
                        "available_memory_bytes": sys.available_memory(),
                        "total_swap_bytes": sys.total_swap(),
                        "used_swap_bytes": sys.used_swap(),
                        "total_memory_gb": format!("{:.2}", sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                        "used_memory_gb": format!("{:.2}", sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                        "available_memory_gb": format!("{:.2}", sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                    },
                    "disks": disk_info,
                })
            }
        };

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&info).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Failed to serialize system info: {}", e),
                    None,
                )
            })?,
        )]))
    }

    /// Executa um comando no terminal
    #[tool(
        description = "Executa um comando no terminal Windows e retorna o resultado incluindo stdout, stderr e código de saída. ATENÇÃO: Use com cuidado, pois pode executar qualquer comando no sistema. \
        \n\nMétodos de execução:\
        \n- Normal (padrão): executa com permissões do usuário atual\
        \n- use_elevation=true: usa UAC (User Account Control) com diálogo gráfico nativo do Windows para autenticação (recomendado para comandos que precisam de administrador)"
    )]
    async fn execute_command(
        &self,
        Parameters(args): Parameters<ExecuteCommandArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        if args.use_elevation.unwrap_or(false) {
            self.execute_elevated_command(&args).await
        } else {
            self.execute_normal_command(&args).await
        }
    }

    /// Executa um comando normal sem elevação de privilégios
    async fn execute_normal_command(
        &self,
        args: &ExecuteCommandArgs,
    ) -> Result<CallToolResult, ErrorData> {
        let mut cmd = Command::new(&args.command);

        if let Some(cmd_args) = &args.args {
            cmd.args(cmd_args);
        }

        let output = cmd.output().map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to execute command: {}", e),
                None,
            )
        })?;

        let result = json!({
            "command": format!("{} {}", args.command, args.args.clone().unwrap_or_default().join(" ")),
            "elevation_method": "none",
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout).to_string(),
            "stderr": String::from_utf8_lossy(&output.stderr).to_string(),
            "success": output.status.success(),
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&result).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Failed to serialize command result: {}", e),
                    None,
                )
            })?,
        )]))
    }

    /// Executa um comando usando UAC (User Account Control) do Windows
    /// UAC apresenta um diálogo gráfico de autenticação padrão do Windows
    async fn execute_elevated_command(
        &self,
        args: &ExecuteCommandArgs,
    ) -> Result<CallToolResult, ErrorData> {
        let command = args.command.clone();
        let cmd_args = args.args.clone();
        
        // Executar em uma thread bloqueante para não bloquear o runtime assíncrono
        let result = tokio::task::spawn_blocking(move || {
            windows_elevation::execute_elevated(&command, cmd_args.as_ref())
        })
        .await
        .map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to spawn elevation task: {}", e),
                None,
            )
        })?
        .map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to execute elevated command: {}. Make sure you're running in an environment with UAC enabled.", e),
                None,
            )
        })?;

        let (exit_code, stdout, stderr, success) = result;

        let result_json = json!({
            "command": format!("{} {}", args.command, args.args.clone().unwrap_or_default().join(" ")),
            "elevation_method": "UAC (User Account Control)",
            "exit_code": exit_code,
            "stdout": stdout,
            "stderr": stderr,
            "success": success,
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&result_json).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Failed to serialize command result: {}", e),
                    None,
                )
            })?,
        )]))
    }
}

#[tool_handler]
impl ServerHandler for WindowsMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Este servidor MCP fornece ferramentas para obter informações do sistema Windows \
                 e executar comandos no terminal.\n\n\
                 Ferramentas disponíveis:\n\
                 - get_system_info: Obtém informações sobre CPU, memória, discos ou sistema operacional\n\
                 - execute_command: Executa comandos no terminal e retorna o resultado\n\n\
                 Para executar comandos que precisam de privilégios de administrador, use 'use_elevation: true' \
                 para acionar o UAC (User Account Control) do Windows."
                    .to_string(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Criar o servidor
    let server = WindowsMcpServer::new();

    // Criar transporte stdio (stdin/stdout)
    let transport = (tokio::io::stdin(), tokio::io::stdout());

    // Executar o servidor
    let service = server.serve(transport).await?;

    // Aguardar até o servidor terminar
    service.waiting().await?;

    Ok(())
}
