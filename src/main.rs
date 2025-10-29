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
    /// Se true, usa PolicyKit (pkexec) para autenticação com interface gráfica
    #[serde(default)]
    use_polkit: Option<bool>,
}

/// Servidor MCP Linux
#[derive(Clone)]
pub struct LinuxMcpServer {
    tool_router: ToolRouter<Self>,
    system: Arc<Mutex<System>>,
}

#[tool_router]
impl LinuxMcpServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            system: Arc::new(Mutex::new(System::new_all())),
        }
    }

    /// Obtém informações do sistema Linux
    #[tool(
        description = "Obtém informações do sistema Linux como CPU, memória, discos e sistema operacional. Você pode especificar o tipo de informação: 'cpu', 'memory', 'disk', 'os' ou 'all' (padrão)."
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
        description = "Executa um comando no terminal e retorna o resultado incluindo stdout, stderr e código de saída. ATENÇÃO: Use com cuidado, pois pode executar qualquer comando no sistema. \
        \n\nMétodos de autenticação:\
        \n- Normal (padrão): executa com permissões do usuário atual\
        \n- use_polkit=true: usa PolicyKit/pkexec com diálogo gráfico nativo do sistema para autenticação (recomendado para comandos que precisam de root)"
    )]
    async fn execute_command(
        &self,
        Parameters(args): Parameters<ExecuteCommandArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        if args.use_polkit.unwrap_or(false) {
            self.execute_polkit_command(&args).await
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

    /// Executa um comando usando PolicyKit (pkexec)
    /// PolicyKit apresenta uma interface gráfica de autenticação e é mais seguro
    async fn execute_polkit_command(
        &self,
        args: &ExecuteCommandArgs,
    ) -> Result<CallToolResult, ErrorData> {
        // Verificar se pkexec está disponível
        if Command::new("which")
            .arg("pkexec")
            .output()
            .map(|o| !o.status.success())
            .unwrap_or(true)
        {
            return Err(ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                "PolicyKit (pkexec) não está instalado no sistema. Instale o pacote 'polkit' para usar este recurso.".to_string(),
                None,
            ));
        }

        let mut pkexec_args = vec![args.command.clone()];
        if let Some(cmd_args) = &args.args {
            pkexec_args.extend(cmd_args.clone());
        }

        let mut cmd = Command::new("pkexec");
        cmd.args(&pkexec_args);

        // Importante: pkexec precisa de um ambiente gráfico ou dbus para funcionar
        // Define variáveis de ambiente necessárias
        if let Ok(display) = std::env::var("DISPLAY") {
            cmd.env("DISPLAY", display);
        }
        if let Ok(xauth) = std::env::var("XAUTHORITY") {
            cmd.env("XAUTHORITY", xauth);
        }
        if let Ok(wayland) = std::env::var("WAYLAND_DISPLAY") {
            cmd.env("WAYLAND_DISPLAY", wayland);
        }

        let output = cmd.output().map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to execute pkexec command: {}. Certifique-se de que você está em um ambiente gráfico com D-Bus rodando.", e),
                None,
            )
        })?;

        let result = json!({
            "command": format!("pkexec {} {}", args.command, args.args.clone().unwrap_or_default().join(" ")),
            "elevation_method": "pkexec (PolicyKit)",
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
}

#[tool_handler]
impl ServerHandler for LinuxMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Este servidor MCP fornece ferramentas para obter informações do sistema Linux \
                 e executar comandos no terminal.\n\n\
                 Ferramentas disponíveis:\n\
                 - get_system_info: Obtém informações sobre CPU, memória, discos ou sistema operacional\n\
                 - execute_command: Executa comandos no terminal e retorna o resultado"
                    .to_string(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Criar o servidor
    let server = LinuxMcpServer::new();

    // Criar transporte stdio (stdin/stdout)
    let transport = (tokio::io::stdin(), tokio::io::stdout());

    // Executar o servidor
    let service = server.serve(transport).await?;

    // Aguardar até o servidor terminar
    service.waiting().await?;

    Ok(())
}
