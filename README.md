# Windows MCP Server

Servidor MCP (Model Context Protocol) em Rust que fornece ferramentas para obter informações do sistema Windows e executar comandos no terminal com autenticação segura via UAC.

> 🔐 Segurança: Este servidor usa UAC (User Account Control) para autenticação de comandos administrativos — uma janela nativa do Windows pede sua permissão; nada é exposto ao MCP.

## 📖 Índice

- ⚡ Quick Start
- 🚀 Funcionalidades
- 📦 Compilação (Windows)
- 🔧 Uso
- 📚 Exemplos de Uso
- 🔐 UAC - Autenticação Segura
- 🔧 Troubleshooting
- ⚠️ Segurança

---

## ⚡ Quick Start — Executar comandos com administrador

Para executar comandos que precisam de permissões de administrador (instalar software, gerenciar serviços, etc.), adicione `use_elevation: true`:

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "net",
    "args": ["start", "W3SVC"],
    "use_elevation": true
  }
}
```

O que acontece: uma janela NATIVA do Windows aparece pedindo permissão de administrador (UAC). Sua autorização não é enviada pelo MCP — o Windows cuida de tudo com segurança.

Erro comum:

- ❌ Errado (vai falhar com "Access is denied"): sem `use_elevation: true`.
- ✅ Certo: inclua `use_elevation: true`.

---

## 🚀 Funcionalidades

- get_system_info: informações detalhadas do sistema
  - CPU (contagem, marca, uso)
  - Memória (total, usada, disponível, swap)
  - Discos (espaço total, disponível; drives C:\, D:\, etc.)
  - Sistema Operacional (nome, versão, hostname)
- execute_command: executa comandos no terminal Windows
  - Retorna stdout, stderr e código de saída
  - Suporta argumentos
  - Modos de execução:
    - Normal (padrão): permissões do usuário atual
    - UAC (`use_elevation: true`): diálogo gráfico nativo do Windows (recomendado para comandos administrativos)

## 📦 Compilação (Windows)

### Pré-requisitos

- Windows 10/11 ou Windows Server 2016+
- Rust (via rustup)
- Um toolchain C/C++ para linkedição:
  - Opção A — MSVC (recomendado):
    - Instale "Visual Studio Build Tools" com a carga de trabalho "Desktop development with C++" (inclui `link.exe`).
    - Toolchain Rust: `stable-x86_64-pc-windows-msvc` (padrão em máquinas Windows).
  - Opção B — GNU (alternativa):
    - Instale o MSYS2 e o pacote `mingw-w64-x86_64-toolchain` (fornece `gcc.exe`).
    - Configure o PATH para incluir `C:\msys64\mingw64\bin` e, se desejar, force `x86_64-pc-windows-gnu` no Cargo.

> Dica: Se você ver o erro "linker `link.exe` not found", instale os Visual C++ Build Tools (opção A) ou mude para o toolchain GNU com MSYS2 (opção B). Veja Troubleshooting.

### Compilar o projeto

```powershell
cargo build --release
```

O executável será gerado em `target\release\windows-mcp.exe`.

## 🔧 Uso

### Executar o servidor

```powershell
.\u200Btarget\release\windows-mcp.exe
```

O servidor se comunica via stdio (stdin/stdout) seguindo o protocolo MCP.

### Integração com Claude Desktop

1. Compile o binário:

```powershell
cargo build --release
```

2. Edite a configuração em:

```
%APPDATA%\Claude\claude_desktop_config.json
```

3. Adicione a configuração do servidor:

```json
{
  "mcpServers": {
    "windows-mcp": {
      "command": "C:\\caminho\\completo\\para\\windows-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

4. Reinicie o Claude Desktop completamente e verifique o ícone 🔌 no chat.

### Integração com VS Code / Cursor IDE

Configuração por projeto:

```powershell
# VS Code
mkdir .vscode
notepad .vscode\mcp.json

# Cursor
mkdir .cursor
notepad .cursor\mcp_config.json
```

Conteúdo sugerido:

```json
{
  "mcpServers": {
    "windows-mcp": {
      "command": "C:\\Users\\Marcos\\Documents\\windows-mcp\\target\\release\\windows-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

Configuração global:

- VS Code: `%APPDATA%\Code\User\mcp.json`
- Cursor: `%APPDATA%\Cursor\User\mcp_config.json`

### Testar com MCP Inspector

```powershell
npx @modelcontextprotocol/inspector C:\caminho\completo\para\windows-mcp.exe
```

## 📚 Exemplos de Uso

Obter tudo:

```json
{
  "name": "get_system_info",
  "arguments": { "info_type": "all" }
}
```

Somente CPU:

```json
{
  "name": "get_system_info",
  "arguments": { "info_type": "cpu" }
}
```

Somente discos:

```json
{
  "name": "get_system_info",
  "arguments": { "info_type": "disk" }
}
```

Comando normal (sem administrador):

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "dir",
    "args": ["C:\\Users"]
  }
}
```

Comando PowerShell:

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "powershell",
    "args": ["-Command", "Get-Process | Select-Object -First 5"]
  }
}
```

Comando com UAC (administrador):

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "net",
    "args": ["start", "W3SVC"],
    "use_elevation": true
  }
}
```

Exemplos que exigem elevação:

| Comando                                                                                 | Descrição                        |
| --------------------------------------------------------------------------------------- | -------------------------------- |
| `"command": "net", "args": ["start", "W3SVC"], "use_elevation": true`                   | Iniciar serviço IIS              |
| `"command": "sc", "args": ["query", "wuauserv"], "use_elevation": true`                 | Verificar serviço Windows Update |
| `"command": "reg", "args": ["query", "HKLM\\SOFTWARE"], "use_elevation": true`          | Ler registro do sistema          |
| `"command": "netsh", "args": ["interface", "show", "interface"], "use_elevation": true` | Ver interfaces de rede           |

---

## 🔐 UAC — Autenticação Segura no Windows

UAC (User Account Control) é o sistema nativo do Windows para autenticação de privilégios administrativos.

Por que usar UAC?

- Seguro: diálogo gráfico de autenticação; permissão nunca exposta nos logs
- Controle: aceitar ou negar cada solicitação
- Auditoria: registros no Event Viewer
- Nativo: interface oficial do Windows

---

## 🔧 Troubleshooting

### "linker `link.exe` not found"

- Instale os Visual Studio Build Tools (C++). Depois, reabra o terminal/VS Code.
- Alternativa: use o toolchain GNU com MSYS2 (`mingw-w64-x86_64-toolchain`) e garanta `C:\msys64\mingw64\bin` no PATH. Opcionalmente, crie `.cargo\config.toml` com:

```toml
[build]
target = "x86_64-pc-windows-gnu"

[target.x86_64-pc-windows-gnu]
linker = "gcc"
```

### Diálogo UAC não aparece

- Verifique se o UAC está ativado (EnableLUA=1). Painel de Controle → Contas de Usuário → Controle de Conta de Usuário.

### "Access denied"/"Permission denied"

- Inclua `use_elevation: true` ao executar comandos administrativos.

### Claude Desktop não detecta o servidor

- Caminho incorreto ou JSON inválido. Use `\\` nos caminhos e valide o JSON. Reinicie o app completamente.

---

## ⚠️ Segurança

O tool `execute_command` pode executar qualquer comando no sistema.

- UAC: autenticação via janela nativa — nenhuma credencial é armazenada
- Auditoria: comandos elevados são registrados no Event Viewer
- Boas práticas: use `use_elevation: true` quando necessário; nunca mantenha o servidor sempre elevado

---

## 📝 Compatibilidade

- Windows 10 (1809+), Windows 11, Windows Server 2016+

---

## 📝 Licença

Projeto sob licença MIT.

## 🤝 Contribuindo

Contribuições são bem-vindas! Abra issues ou pull requests.

---

## 🆚 Diferenças rápidas (Linux x Windows)

| Recurso                 | Linux MCP          | Windows MCP                |
| ----------------------- | ------------------ | -------------------------- |
| Elevação de privilégios | PolicyKit (pkexec) | UAC (User Account Control) |
| Shell padrão            | bash               | cmd.exe / PowerShell       |
| Formato de caminhos     | `/home/user`       | `C:\\Users\\user`          |
| Logs de auditoria       | journalctl         | Event Viewer               |

Links úteis: UAC, MCP, Claude Desktop.
