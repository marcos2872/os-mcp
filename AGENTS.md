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
- `linux://mcp/capabilities` - **NOVO**: Lista comandos permitidos e regras de segurança
- `linux://logs/system` - Logs do sistema
- `linux://logs/auth` - Logs de autenticação
- `linux://config/network` - Configuração de rede
- `linux://processes/top` - Top processos por memória
- `linux://system/status` - Status geral

<!-- ... -->

## 🔐 Segurança

**Modo de Operação Seguro (Ativo):**

1.  **Allowlist de Comandos**:
    - O servidor rejeita qualquer comando que não esteja na lista explícita (`src/tools/mod.rs`).
    - Comandos permitidos incluem: `ls`, `grep`, `apt`, `systemctl`, etc.
    - Para ver a lista completa, leia o resource `linux://mcp/capabilities`.

2.  **Política de Safe RM**:
    - O comando `rm` é **bloqueado** por padrão.
    - Exceção: Permitido apenas para limpeza recursiva em diretórios seguros:
        - `/tmp/*`, `/var/tmp/*`
        - `/var/log/*`
        - `~/.cache/*`
        - `~/.local/share/Trash/*`
    - Qualquer tentativa de `rm` fora desses caminhos (ex: `/etc`, `/home`) falhará.

3.  **PolicyKit**:
    - Comandos administrativos (como `apt update`) exigem `use_polkit: true`.
    - Isso abre uma janela nativa no sistema do usuário para autenticação de senha.

4.  **Agentes de IA**:
    - Antes de executar tarefas complexas, **sempre consulte `linux://mcp/capabilities`**.
    - Isso evita tentativas frustradas de executar comandos bloqueados.

## 🧪 Testando

Para testar o servidor MCP:

1. **Manualmente via stdio**: Execute o binário e envie JSON-RPC via stdin
2. **MCP Inspector**: Use `npx @modelcontextprotocol/inspector` para UI interativa
3. **Integração**: Configure no Claude Desktop ou Cursor IDE

## 📚 Referências

- [Especificação MCP](https://github.com/modelcontextprotocol/spec)
- [rmcp (Rust MCP)](https://crates.io/crates/rmcp)
- [PolicyKit](https://www.freedesktop.org/wiki/Software/polkit/)
