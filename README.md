# Linux MCP Server 🚀

> Um servidor MCP seguro para Linux que permite a IAs (como Claude e Cursor) ler informações do sistema e executar comandos via PolicyKit.

---

## ⚡ Instalação Rápida

1. **Clone e Compile**:
```bash
git clone https://github.com/marcos2872/os-mcp.git
cd os-mcp
cargo build --release
```
*O binário estará em: `target/release/linux-mcp`*

---

## ⚙️ Configuração (JSON)

Adicione ao seu arquivo de configuração (substitua `/CAMINHO/PARA` pelo caminho real):

### Claude Desktop
`~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "linux-mcp": {
      "command": "/CAMINHO/PARA/os-mcp/target/release/linux-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

### Cursor IDE / VS Code
`.cursor/mcp_config.json` ou `.vscode/mcp.json` na raiz do projeto:

```json
{
  "mcpServers": {
    "linux-mcp": {
      "command": "/CAMINHO/PARA/os-mcp/target/release/linux-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

### Inspector (Teste Visual)
Testar o servidor diretamente no navegador:

```bash
npx @modelcontextprotocol/inspector ./target/release/linux-mcp
```

---

## 🛠️ Configuração Avançada

O servidor cria automaticamente arquivos de configuração em `~/.config/linux-mcp/`:

1.  **`config.toml`** (Allowlist Dinâmica):
    *   Lista de comandos permitidos.
    *   Você pode editar este arquivo para adicionar/remover comandos sem recompilar.
    *   Reinicie o servidor após editar.

2.  **`audit.log`** (Audit Trail):
    *   Registro completo de todos os comandos executados.
    *   Mostra data, hora, comando, status (ALLOWED/BLOCKED) e detalhes.

---

## 📚 Exemplos de Uso

### 1. Ver Informações do Sistema
```json
{
  "name": "get_system_info",
  "arguments": { "info_type": "all" }
}
```

### 2. Executar Comandos (Seguro)
Comandos permitidos (Allowlist) podem ser executados com ou sem root.

**Comando normal:**
```json
{
  "name": "execute_command",
  "arguments": { "command": "ls -la" }
}
```

**Comando com Root (Abre janela de senha):**
```json
{
  "name": "execute_command",
  "arguments": { 
    "command": "apt update",
    "use_polkit": true 
  }
}
```

---

## 🛡️ Segurança

Este servidor opera em **Modo Seguro**:

1.  **Allowlist**: Apenas comandos permitidos (ex: `ls`, `grep`, `git`, `apt`) podem ser executados.
2.  **No RM**: O comando `rm` é bloqueado, exceto em pastas temporárias (`/tmp`) e lixeira.
3.  **PolicyKit**: Comandos administrativos exigem sua senha via janela nativa do sistema.

📄 **Documentação Completa**:
- [AGENTS.md](AGENTS.md) - Guia para Agentes de IA
- [SECURITY_GUIDELINES.md](SECURITY_GUIDELINES.md) - Diretrizes de Segurança
