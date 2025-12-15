mod prompts;
mod resources;
mod tools;

use anyhow::Result;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::service::RequestContext;
use rmcp::{tool, tool_handler, tool_router, ErrorData, RoleServer, ServerHandler, ServiceExt};
use std::collections::HashMap;
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::Mutex;

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
        Parameters(args): Parameters<tools::SystemInfoArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        tools::get_system_info(self.system.clone(), args).await
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
        Parameters(args): Parameters<tools::ExecuteCommandArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        tools::execute_command(args).await
    }
}

#[tool_handler]
impl ServerHandler for LinuxMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Este servidor MCP fornece ferramentas para obter informações do sistema Linux \
                 e executar comandos no terminal.\n\n\
                 Ferramentas disponíveis:\n\
                 - get_system_info: Obtém informações sobre CPU, memória, discos ou sistema operacional\n\
                 - execute_command: Executa comandos no terminal e retorna o resultado\n\n\
                 Resources disponíveis:\n\
                 - linux://logs/system: Logs do sistema\n\
                 - linux://logs/auth: Logs de autenticação\n\
                 - linux://config/network: Configuração de rede\n\
                 - linux://processes/top: Processos usando mais recursos\n\
                 - linux://system/status: Status geral do sistema\n\n\
                 Prompts disponíveis:\n\
                 - system_troubleshooting: Guia para solução de problemas\n\
                 - security_audit: Auditoria básica de segurança\n\
                 - service_management: Gerenciamento de serviços systemd\n\
                 - log_analysis: Análise de logs do sistema\n\
                 - disk_cleanup: Limpeza segura de disco\n\n\
                 🛡️ MODO SEGURO ATIVO: Este servidor restringe a execução de comandos.\n\
                 Consulte o resource 'linux://mcp/capabilities' para ver a lista de comandos permitidos e regras de segurança."
                    .to_string(),
            ),
        }
    }

    async fn list_resources(
        &self,
        _pagination: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        Ok(ListResourcesResult {
            resources: resources::list_resources(),
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        let text = resources::read_resource(&request.uri).await.map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to read resource: {}", e),
                None,
            )
        })?;

        Ok(ReadResourceResult {
            contents: vec![ResourceContents::TextResourceContents {
                uri: request.uri,
                mime_type: Some("text/plain".to_string()),
                text,
                meta: None,
            }],
        })
    }

    async fn list_prompts(
        &self,
        _pagination: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        Ok(ListPromptsResult {
            prompts: prompts::list_prompts(),
            next_cursor: None,
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        let arguments: HashMap<String, String> = request
            .arguments
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(k, v)| {
                if let serde_json::Value::String(s) = v {
                    Some((k, s))
                } else {
                    None
                }
            })
            .collect();

        prompts::get_prompt(&request.name, arguments).map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to get prompt: {}", e),
                None,
            )
        })
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
