# Linux MCP Server

Servidor MCP (Model Context Protocol) em Rust que fornece ferramentas para obter informações do sistema Linux e executar comandos no terminal com **autenticação segura via PolicyKit**.

> 🔐 **Segurança**: Este servidor usa PolicyKit (pkexec) para autenticação de comandos root - uma janela nativa do sistema pede sua senha, que nunca é exposta no MCP!

## 📖 Documentação

- 🚀 **[Configurar no Claude Desktop](CLAUDE_DESKTOP_SETUP.md)** - Guia passo a passo completo
- 📋 **[Referência Rápida](QUICK_REFERENCE.md)** - Exemplos prontos de uso
- 🔐 **[Guia PolicyKit](examples/polkit/README_POLKIT.md)** - Configuração de segurança avançada

---

## ⚡ Quick Start - Executar comandos com root

**Para executar comandos que precisam de permissões de administrador** (como `apt update`, `systemctl restart`, etc.), adicione `use_polkit: true`:

### ✅ PolicyKit - Janela Gráfica Nativa do Sistema

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt update",
    "use_polkit": true
  }
}
```

**O que acontece**: Uma janela NATIVA do seu sistema operacional aparece pedindo senha (igual quando você instala programas). Sua senha **nunca é enviada pelo MCP** - o sistema operacional cuida da autenticação de forma segura.

### ⚠️ Erro Comum

❌ **ERRADO** (vai falhar com "Permission denied"):

```json
{
  "command": "apt update"
}
```

✅ **CORRETO** (adicione `use_polkit: true`):

```json
{
  "command": "apt update",
  "use_polkit": true
}
```

"use_polkit": true
}

````

---

## 🚀 Funcionalidades

### 🛠️ Tools (Ferramentas)

- **get_system_info**: Obtém informações detalhadas do sistema

  - CPU (contagem, marca, uso)
  - Memória (total, usada, disponível, swap)
  - Discos (espaço total, disponível, sistema de arquivos)
  - Sistema Operacional (nome, versão do kernel, hostname)

- **execute_command**: Executa comandos no terminal
  - Retorna stdout, stderr e código de saída
  - Comando completo passado como string única
  - **2 modos de execução**:
    - **Normal** (padrão): executa com permissões do usuário atual
    - **PolicyKit** (`use_polkit: true`): usa pkexec com diálogo gráfico nativo do sistema - RECOMENDADO para comandos que precisam de root
  - ⚠️ Use com cuidado - pode executar qualquer comando no sistema

### 📚 Resources (Recursos de Leitura)

Acesso rápido a informações do sistema sem executar comandos:

- **`linux://logs/system`** - Últimas 100 linhas dos logs do sistema (journalctl)
- **`linux://logs/auth`** - Últimas 50 linhas dos logs de autenticação SSH
- **`linux://config/network`** - Configuração de rede atual (ip addr show)
- **`linux://processes/top`** - Top 10 processos por uso de memória
- **`linux://system/status`** - Status geral do sistema (CPU, memória, uptime)

### 💡 Prompts (Fluxos Interativos)

Guias assistidos para tarefas comuns de administração:

- **`system_troubleshooting`** - Diagnóstico interativo de problemas (CPU, memória, disco, rede, processos)
- **`security_audit`** - Auditoria de segurança do sistema (escopo básico ou completo)
- **`service_management`** - Gerenciamento de serviços systemd (status, start, stop, restart, enable, disable)
- **`log_analysis`** - Análise de logs com filtros (system, auth, kernel, aplicações)
- **`disk_cleanup`** - Limpeza segura de disco (modo conservador ou agressivo)

## 📦 Compilação

```bash
cargo build --release
````

O binário será gerado em `target/release/linux-mcp`

## 🔧 Uso

### Executar o servidor

```bash
./target/release/linux-mcp
```

O servidor se comunica via stdio (stdin/stdout) seguindo o protocolo MCP.

### Integração com Claude Desktop

> 🤖 **Guia Detalhado**: Veja [CLAUDE_DESKTOP_SETUP.md](CLAUDE_DESKTOP_SETUP.md) para instruções passo a passo completas!

**Resumo rápido**:

1. Compilar: `cargo build --release`
2. Editar: `~/.config/Claude/claude_desktop_config.json` (Linux)
3. Adicionar configuração:

```json
{
  "mcpServers": {
    "linux-mcp": {
      "command": "/caminho/completo/para/linux-mcp-wrapper.sh",
      "args": [],
      "env": {}
    }
  }
}
```

4. Reiniciar Claude Desktop completamente

**Caminhos de configuração**:

- Linux: `~/.config/Claude/claude_desktop_config.json`
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`

### Integração com Cursor IDE / VS Code

#### Opção 1: Configuração por projeto

Crie o arquivo `.cursor/mcp_config.json` (Cursor) ou `.vscode/mcp.json` (VS Code) na raiz do seu projeto:

```bash
# Para Cursor
mkdir -p .cursor
nano .cursor/mcp_config.json

# Para VS Code
mkdir -p .vscode
nano .vscode/mcp.json
```

**Conteúdo do arquivo**:

```json
{
  "mcpServers": {
    "linux-mcp": {
      "command": "/home/marcos/Documents/Pessoal/linux-mcp/target/release/linux-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

#### Opção 2: Configuração global

Configure globalmente em `~/.config/cursor/mcp_config.json` ou `~/.config/Code/User/mcp.json`

### Testar com MCP Inspector

```bash
npx @modelcontextprotocol/inspector /caminho/completo/para/linux-mcp
```

## 📚 Exemplos de Uso

### 🛠️ Tools

#### Obter informações completas do sistema

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "all"
  }
}
```

#### Obter apenas informações de CPU

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "cpu"
  }
}
```

#### Executar comando normal (sem root)

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "ls -la /home"
  }
}
```

#### ⭐ Executar comando com PolicyKit (para comandos que precisam de root)

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt update",
    "use_polkit": true
  }
}
```

**Resultado**: Janela nativa do sistema pede senha → comando executado com segurança ✅

**O que acontece**: Uma **janela gráfica oficial do sistema** aparece pedendo sua senha de administrador (igual quando você instala programas pela Central de Aplicativos).

### 📚 Resources

Resources fornecem acesso direto a informações do sistema sem executar comandos:

#### Ler logs do sistema

```json
{
  "method": "resources/read",
  "params": {
    "uri": "linux://logs/system"
  }
}
```

#### Ver configuração de rede

```json
{
  "method": "resources/read",
  "params": {
    "uri": "linux://config/network"
  }
}
```

#### Verificar processos com mais uso de memória

```json
{
  "method": "resources/read",
  "params": {
    "uri": "linux://processes/top"
  }
}
```

#### Status geral do sistema

```json
{
  "method": "resources/read",
  "params": {
    "uri": "linux://system/status"
  }
}
```

### 💡 Prompts

Prompts guiam você através de tarefas comuns de administração:

#### Troubleshooting de CPU

```json
{
  "method": "prompts/get",
  "params": {
    "name": "system_troubleshooting",
    "arguments": {
      "problem_type": "cpu"
    }
  }
}
```

#### Auditoria de segurança completa

```json
{
  "method": "prompts/get",
  "params": {
    "name": "security_audit",
    "arguments": {
      "scope": "full"
    }
  }
}
```

#### Gerenciar serviço nginx

```json
{
  "method": "prompts/get",
  "params": {
    "name": "service_management",
    "arguments": {
      "service_name": "nginx",
      "action": "restart"
    }
  }
}
```

#### Analisar logs de autenticação

```json
{
  "method": "prompts/get",
  "params": {
    "name": "log_analysis",
    "arguments": {
      "log_type": "auth",
      "priority": "warning"
    }
  }
}
```

#### Limpeza agressiva de disco

```json
{
  "method": "prompts/get",
  "params": {
    "name": "disk_cleanup",
    "arguments": {
      "aggressive": "true"
    }
  }
}
```

#### ⚠️ IMPORTANTE: Adicione `use_polkit: true` para comandos root

Comandos que precisam de root (como `apt update`, `systemctl restart`, etc.) **devem** incluir um método de elevação:

| Comando sem elevação ❌                | Comando correto ✅                                         |
| -------------------------------------- | ---------------------------------------------------------- |
| `"command": "apt update"`              | `"command": "apt update", "use_polkit": true`              |
| `"command": "systemctl restart nginx"` | `"command": "systemctl restart nginx", "use_polkit": true` |

✅ **PolicyKit é mais seguro**: Apresenta um diálogo gráfico de autenticação e permite controle granular de permissões. Veja [Guia Completo de PolicyKit](examples/polkit/README_POLKIT.md) para instruções detalhadas.

## 🔐 PolicyKit - Autenticação Segura com Root

PolicyKit é o sistema nativo do Linux para autenticação de privilégios administrativos.

### ⭐ Por que usar PolicyKit?

### ⭐ Por que usar PolicyKit?

- ✅ **Seguro**: Diálogo gráfico de autenticação - senha nunca exposta nos logs
- ✅ **Controle granular**: Permissões por comando e usuário
- ✅ **Auditoria**: Registro completo no journal do sistema
- ✅ **Timeout automático**: Credenciais expiram automaticamente
- ✅ **Nativo**: Interface oficial do seu desktop Linux (GNOME, KDE, XFCE, etc.)

### 📦 Instalação

```bash
# Ubuntu/Debian
sudo apt install polkitd policykit-1

# Fedora/RHEL
sudo dnf install polkit

# Arch Linux
sudo pacman -S polkit
```

Verificar instalação:

```bash
which pkexec
systemctl status polkit
```

### 🚀 Uso Básico

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl restart nginx",
    "use_polkit": true
  }
}
```

**O que acontece:**

1. O MCP executa `pkexec systemctl restart nginx`
2. Uma **janela NATIVA do seu sistema operacional** aparece (não é customizada - é a oficial do Linux)
3. Você digita sua senha de administrador
4. O PolicyKit valida e executa o comando com privilégios
5. O resultado retorna para o MCP

**Aparência da janela:**

- **GNOME/Ubuntu**: Janela com escudo vermelho/amarelo "Authentication Required"
- **KDE Plasma**: Diálogo azul do KDE Polkit Agent
- **XFCE/MATE**: Janela simples do ambiente específico

✅ **Sua senha nunca é enviada pelo MCP** - o sistema operacional cuida da autenticação!

### ⚙️ Configuração Avançada (Opcional)

Para permitir comandos específicos sem senha, crie regras personalizadas:

```bash
# Copiar arquivo de exemplo
sudo cp examples/polkit/50-linux-mcp.rules /etc/polkit-1/rules.d/

# Editar para seu usuário
sudo nano /etc/polkit-1/rules.d/50-linux-mcp.rules

# Reiniciar polkit
sudo systemctl restart polkit
```

📖 **Guia Completo**: Veja [examples/polkit/README_POLKIT.md](examples/polkit/README_POLKIT.md) para instruções detalhadas, exemplos e solução de problemas.

---

## 🔧 Troubleshooting PolicyKit

### ❌ Problema: Janela de autenticação não aparece

**Causa**: O MCP está rodando sem acesso à sessão gráfica.

**Solução 1: Configure variáveis de ambiente no MCP**

```json
{
  "mcpServers": {
    "linux-mcp": {
      "command": "/caminho/para/linux-mcp",
      "env": {
        "DISPLAY": ":0",
        "XAUTHORITY": "/home/seu_usuario/.Xauthority",
        "DBUS_SESSION_BUS_ADDRESS": "unix:path=/run/user/1000/bus"
      }
    }
  }
}
```

**Solução 2: Verificar se o agente polkit está rodando**

```bash
# Verificar processo
ps aux | grep polkit

# Iniciar manualmente (GNOME/Ubuntu)
/usr/libexec/polkit-gnome-authentication-agent-1 &

# Iniciar manualmente (KDE)
/usr/lib/polkit-kde-authentication-agent-1 &
```

---

### ❌ Problema: Erro "PolicyKit (pkexec) não está instalado"

**Solução**: Instalar PolicyKit

```bash
# Ubuntu/Debian
sudo apt install policykit-1 polkitd

# Fedora/RHEL
sudo dnf install polkit

# Arch Linux
sudo pacman -S polkit
```

Verificar instalação:

```bash
which pkexec
systemctl status polkit
```

---

### ❌ Problema: Erro "Not authorized" ou "Authentication failed"

**Causa**: Seu usuário não tem permissão ou as regras do PolicyKit bloquearam.

**Solução**: Configurar regras do PolicyKit

```bash
# Copiar regras de exemplo
sudo cp examples/polkit/50-linux-mcp.rules /etc/polkit-1/rules.d/

# Editar e substituir "marcos" pelo seu usuário
sudo nano /etc/polkit-1/rules.d/50-linux-mcp.rules

# Reiniciar polkit
sudo systemctl restart polkit
```

**Ver logs de erro**:

```bash
journalctl -u polkit -f
```

---

### ❌ Problema: "Permission denied" ao executar comando

**Causa**: Você esqueceu de adicionar `use_polkit: true`.

**Exemplo do erro**:

```json
{
  "exit_code": 100,
  "stderr": "E: Could not open lock file - open (13: Permission denied)",
  "elevation_method": "none"
}
```

**Solução**: Adicionar método de elevação:

```json
{
  "command": "apt update",
  "use_polkit": true // ← ADICIONE ISSO!
}
```

---

### 🔍 Debug: Ver o que está acontecendo

```bash
# Ver logs do PolicyKit em tempo real
journalctl -u polkit -f

# Testar pkexec manualmente no terminal
pkexec systemctl status nginx

# Ver todas as ações disponíveis do PolicyKit
pkaction

# Verificar variáveis de ambiente
echo $DISPLAY
echo $DBUS_SESSION_BUS_ADDRESS
```

---

## ⚠️ Segurança

O tool `execute_command` pode executar qualquer comando no sistema.

**Segurança implementada**:

- ✅ **PolicyKit**: Autenticação via janela nativa do sistema - senha nunca exposta
- ✅ **Sem sudo/senha**: Nenhuma senha é armazenada ou transmitida pelo MCP
- ✅ **Auditoria**: Todos os comandos com PolicyKit são registrados no journal do sistema

**Boas práticas**:

- Use PolicyKit (`use_polkit: true`) para todos os comandos que precisam de root
- Configure regras do PolicyKit para comandos específicos (veja `examples/polkit/`)
- Monitore logs: `journalctl -u polkit -f`
- Nunca execute o servidor como root

## 📝 Licença

Este projeto é de código aberto e está disponível sob a licença MIT.

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.
