# AGENTS.md

Este documento fornece instruções para agentes de IA e assistentes de código trabalharem com este projeto.

## 📝 Visão Geral do Projeto

**Linux MCP Server** é um servidor [Model Context Protocol (MCP)](https://github.com/modelcontextprotocol/spec) escrito em **Rust** que expõe ferramentas para obter informações do sistema Linux e executar comandos no terminal com autenticação segura via PolicyKit.

## 🏗️ Estrutura do Projeto

```
os-mcp/
├── src/
│   ├── main.rs           # Ponto de entrada e definição do servidor MCP
│   ├── tools/            # Implementação das ferramentas (get_system_info, execute_command)
│   ├── resources/        # Implementação dos recursos de leitura (logs, config, status)
│   └── prompts/          # Implementação dos prompts interativos
├── examples/
│   └── polkit/           # Configurações de exemplo para PolicyKit
├── Cargo.toml            # Dependências e configuração do projeto
├── README.md             # Documentação principal
└── QUICK_REFERENCE.md    # Referência rápida de uso
```

## 🔧 Stack Tecnológica

| Tecnologia | Versão | Uso |
|------------|--------|-----|
| Rust | 2021 edition | Linguagem principal |
| rmcp | 0.8+ | Framework MCP para Rust |
| tokio | 1.42+ | Runtime async |
| sysinfo | 0.32+ | Informações do sistema |
| PolicyKit | - | Autenticação de privilégios |

## 📦 Comandos Essenciais

```bash
# Compilar o projeto
cargo build --release

# Executar o servidor
./target/release/linux-mcp

# Verificar código
cargo check

# Executar testes
cargo test

# Testar com MCP Inspector
npx @modelcontextprotocol/inspector ./target/release/linux-mcp
```

## 🛠️ Arquitetura MCP

O servidor implementa 3 tipos de capacidades MCP:

### Tools (Ferramentas)
Definidas em `src/tools/`:
- `get_system_info` - Obtém info de CPU, memória, disco, OS
- `execute_command` - Executa comandos no terminal (com suporte a PolicyKit)

### Resources (Recursos)
Definidos em `src/resources/`:
- `linux://logs/system` - Logs do sistema
- `linux://logs/auth` - Logs de autenticação
- `linux://config/network` - Configuração de rede
- `linux://processes/top` - Top processos por memória
- `linux://system/status` - Status geral

### Prompts (Fluxos)
Definidos em `src/prompts/`:
- `system_troubleshooting` - Diagnóstico de problemas
- `security_audit` - Auditoria de segurança
- `service_management` - Gerenciamento de serviços
- `log_analysis` - Análise de logs
- `disk_cleanup` - Limpeza de disco

## ⚠️ Diretrizes para Modificações

### Ao adicionar novas Tools:
1. Crie a implementação em `src/tools/`
2. Registre a tool no `#[tool_router] impl LinuxMcpServer`
3. Use a macro `#[tool(description = "...")]` para documentação
4. Defina os argumentos como struct com `#[derive(serde::Deserialize)]`

### Ao adicionar novos Resources:
1. Implemente em `src/resources/`
2. Adicione à lista em `list_resources()`
3. Adicione o handler em `read_resource()`
4. Use URIs no formato `linux://categoria/nome`

### Ao adicionar novos Prompts:
1. Implemente em `src/prompts/`
2. Adicione à lista em `list_prompts()`
3. Adicione o handler em `get_prompt()`

## 🔐 Segurança

**Pontos críticos de segurança:**
- O `execute_command` pode executar qualquer comando - sempre validar inputs
- Comandos que precisam de root devem usar `use_polkit: true`
- Nunca armazenar ou transmitir senhas pelo MCP
- PolicyKit gerencia autenticação de forma segura

## 🧪 Testando

Para testar o servidor MCP:

1. **Manualmente via stdio**: Execute o binário e envie JSON-RPC via stdin
2. **MCP Inspector**: Use `npx @modelcontextprotocol/inspector` para UI interativa
3. **Integração**: Configure no Claude Desktop ou Cursor IDE

## 📚 Referências

- [Especificação MCP](https://github.com/modelcontextprotocol/spec)
- [rmcp (Rust MCP)](https://crates.io/crates/rmcp)
- [PolicyKit](https://www.freedesktop.org/wiki/Software/polkit/)
