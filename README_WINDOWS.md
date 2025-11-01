# Windows MCP Server

Servidor MCP (Model Context Protocol) em Rust que fornece ferramentas para obter informações do sistema Windows e executar comandos no terminal com **autenticação segura via UAC**.

> 🔐 **Segurança**: Este servidor usa UAC (User Account Control) para autenticação de comandos administrativos - uma janela nativa do Windows pede sua permissão, que nunca é exposta no MCP!

## 📖 Índice

- [⚡ Quick Start](#-quick-start---executar-comandos-com-administrador)
- [🚀 Funcionalidades](#-funcionalidades)
- [📦 Compilação](#-compilação)
- [🔧 Uso](#-uso)
- [📚 Exemplos de Uso](#-exemplos-de-uso)
- [🔐 UAC - Autenticação Segura](#-uac---autenticação-segura-no-windows)
- [🔧 Troubleshooting](#-troubleshooting)
- [⚠️ Segurança](#️-segurança)

---

## ⚡ Quick Start - Executar comandos com administrador

**Para executar comandos que precisam de permissões de administrador** (como instalar software, gerenciar serviços, etc.), adicione `use_elevation: true`:

### ✅ UAC - Diálogo Nativo do Windows

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

**O que acontece**: Uma janela NATIVA do Windows aparece pedindo permissão de administrador (igual quando você instala programas). Sua permissão **nunca é enviada pelo MCP** - o Windows cuida da autenticação de forma segura.

### ⚠️ Erro Comum

❌ **ERRADO** (vai falhar com "Access denied"):

```json
{
  "command": "net",
  "args": ["start", "W3SVC"]
}
```

✅ **CORRETO** (adicione `use_elevation: true`):

```json
{
  "command": "net",
  "args": ["start", "W3SVC"],
  "use_elevation": true
}
```

---

## 🚀 Funcionalidades

- **get_system_info**: Obtém informações detalhadas do sistema

  - CPU (contagem, marca, uso)
  - Memória (total, usada, disponível, swap)
  - Discos (espaço total, disponível, sistema de arquivos, drives C:\, D:\, etc.)
  - Sistema Operacional (nome, versão, hostname)

- **execute_command**: Executa comandos no terminal Windows
  - Retorna stdout, stderr e código de saída
  - Suporta argumentos para comandos
  - **2 modos de execução**:
    - **Normal** (padrão): executa com permissões do usuário atual
    - **UAC** (`use_elevation: true`): usa UAC com diálogo gráfico nativo do Windows - RECOMENDADO para comandos que precisam de administrador
  - ⚠️ Use com cuidado - pode executar qualquer comando no sistema

## 📦 Compilação

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (versão 1.70 ou superior)
- Windows 10/11 ou Windows Server 2016+

### Compilar o projeto

```powershell
cargo build --release
```

O binário será gerado em `target\release\windows-mcp.exe`

## 🔧 Uso

### Executar o servidor

```powershell
.\target\release\windows-mcp.exe
```

O servidor se comunica via stdio (stdin/stdout) seguindo o protocolo MCP.

### Integração com Claude Desktop

#### 1. Compilar o binário

```powershell
cargo build --release
```

#### 2. Localizar o arquivo de configuração

O arquivo de configuração do Claude Desktop no Windows está em:

```
%APPDATA%\Claude\claude_desktop_config.json
```

Ou o caminho completo:

```
C:\Users\{SEU_USUARIO}\AppData\Roaming\Claude\claude_desktop_config.json
```

#### 3. Editar a configuração

Abra o arquivo `claude_desktop_config.json` e adicione:

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

**⚠️ IMPORTANTE**: Use barras invertidas duplas (`\\`) nos caminhos do Windows!

**Exemplo completo**:

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

#### 4. Reiniciar Claude Desktop

1. Feche COMPLETAMENTE o Claude Desktop (verifique na bandeja do sistema)
2. Abra novamente
3. Verifique se o ícone 🔌 (plug) aparece no chat
4. Clique no 🔌 para ver se "windows-mcp" está conectado

### Integração com VS Code / Cursor IDE

#### Opção 1: Configuração por projeto

Crie o arquivo `.vscode\mcp.json` (VS Code) ou `.cursor\mcp_config.json` (Cursor) na raiz do seu projeto:

```powershell
# Para VS Code
mkdir .vscode
notepad .vscode\mcp.json

# Para Cursor
mkdir .cursor
notepad .cursor\mcp_config.json
```

**Conteúdo do arquivo**:

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

#### Opção 2: Configuração global

Configure globalmente em:

- VS Code: `%APPDATA%\Code\User\mcp.json`
- Cursor: `%APPDATA%\Cursor\User\mcp_config.json`

### Testar com MCP Inspector

```powershell
npx @modelcontextprotocol/inspector C:\caminho\completo\para\windows-mcp.exe
```

## 📚 Exemplos de Uso

### Obter informações completas do sistema

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "all"
  }
}
```

### Obter apenas informações de CPU

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "cpu"
  }
}
```

### Obter informações de discos (drives C:\, D:\, etc.)

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "disk"
  }
}
```

### Executar comando normal (sem administrador)

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "dir",
    "args": ["C:\\Users"]
  }
}
```

### Executar comando PowerShell

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "powershell",
    "args": ["-Command", "Get-Process | Select-Object -First 5"]
  }
}
```

### ⭐ Executar comando com UAC (para comandos que precisam de administrador)

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

**Resultado**: Janela nativa do Windows pede permissão de administrador → comando executado com segurança ✅

### Exemplos de comandos que precisam de elevação

| Comando                                                                                 | Descrição                        |
| --------------------------------------------------------------------------------------- | -------------------------------- |
| `"command": "net", "args": ["start", "W3SVC"], "use_elevation": true`                   | Iniciar serviço IIS              |
| `"command": "sc", "args": ["query", "wuauserv"], "use_elevation": true`                 | Verificar serviço Windows Update |
| `"command": "reg", "args": ["query", "HKLM\\SOFTWARE"], "use_elevation": true`          | Ler registro do sistema          |
| `"command": "netsh", "args": ["interface", "show", "interface"], "use_elevation": true` | Ver interfaces de rede           |

#### ⚠️ IMPORTANTE: Adicione `use_elevation: true` para comandos administrativos

Comandos que precisam de administrador **devem** incluir elevação:

| Comando sem elevação ❌                          | Comando correto ✅                                                      |
| ------------------------------------------------ | ----------------------------------------------------------------------- |
| `"command": "net", "args": ["start", "W3SVC"]`   | `"command": "net", "args": ["start", "W3SVC"], "use_elevation": true`   |
| `"command": "sc", "args": ["query", "wuauserv"]` | `"command": "sc", "args": ["query", "wuauserv"], "use_elevation": true` |

---

## 🔐 UAC - Autenticação Segura no Windows

UAC (User Account Control) é o sistema nativo do Windows para autenticação de privilégios administrativos.

### ⭐ Por que usar UAC?

- ✅ **Seguro**: Diálogo gráfico de autenticação - permissão nunca exposta nos logs
- ✅ **Controle granular**: Pode aceitar ou negar cada solicitação
- ✅ **Auditoria**: Registro completo no Event Viewer do Windows
- ✅ **Nativo**: Interface oficial do Windows (amarelo/azul)
- ✅ **Proteção**: Previne execução não autorizada de código privilegiado

### 🚀 Uso Básico

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

**O que acontece:**

1. O MCP executa `powershell.exe Start-Process ... -Verb RunAs`
2. Uma **janela NATIVA do Windows** aparece (diálogo UAC azul/amarelo)
3. Você clica em "Sim" para permitir
4. O Windows valida e executa o comando com privilégios
5. O resultado retorna para o MCP

**Aparência da janela UAC:**

- Fundo escurecido
- Diálogo azul/amarelo perguntando "Você deseja permitir que este aplicativo faça alterações no seu dispositivo?"
- Botões "Sim" e "Não"

✅ **Sua permissão nunca é enviada pelo MCP** - o Windows cuida da autenticação!

---

## 🔧 Troubleshooting

### ❌ Problema: Diálogo UAC não aparece

**Causa**: O UAC pode estar desativado no sistema.

**Solução 1: Verificar se UAC está ativo**

1. Abrir `Painel de Controle` → `Contas de Usuário` → `Alterar Configurações de Controle de Conta de Usuário`
2. Certifique-se de que o controle deslizante NÃO está em "Nunca notificar"
3. Recomendado: deixar em "Notificar-me sempre"

**Solução 2: Verificar via PowerShell**

```powershell
# Verificar status do UAC
Get-ItemProperty HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System -Name EnableLUA

# EnableLUA = 1 significa que UAC está ativado
# EnableLUA = 0 significa que UAC está desativado
```

**Ativar UAC via Registry (requer reinicialização)**:

```powershell
Set-ItemProperty -Path HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System -Name EnableLUA -Value 1
```

---

### ❌ Problema: Erro "Access denied" ou "Permission denied"

**Causa**: Você esqueceu de adicionar `use_elevation: true`.

**Exemplo do erro**:

```json
{
  "exit_code": 5,
  "stderr": "System error 5 has occurred.\n\nAccess is denied.",
  "elevation_method": "none"
}
```

**Solução**: Adicionar elevação:

```json
{
  "command": "net",
  "args": ["start", "W3SVC"],
  "use_elevation": true // ← ADICIONE ISSO!
}
```

---

### ❌ Problema: Claude Desktop não detecta o servidor

**Causa**: Caminho incorreto ou formato JSON inválido.

**Solução**:

1. Verifique se o caminho está com barras invertidas duplas: `\\`
2. Verifique se o arquivo `.exe` existe no caminho especificado
3. Teste o comando manualmente no PowerShell:

```powershell
C:\Users\Marcos\Documents\windows-mcp\target\release\windows-mcp.exe
```

4. Valide o JSON em um validador online (ex: jsonlint.com)
5. Reinicie o Claude Desktop COMPLETAMENTE (incluindo processos em segundo plano)

---

### ❌ Problema: Comando não executa ou trava

**Causa**: Alguns comandos podem precisar de interatividade ou podem travar esperando entrada.

**Solução**:

1. Use flags não-interativas:

   - PowerShell: adicione `-NonInteractive`
   - Comandos batch: use `/Y` para confirmações automáticas

2. Evite comandos que abrem janelas GUI ou esperam entrada do usuário

---

### 🔍 Debug: Ver o que está acontecendo

```powershell
# Ver logs de eventos de segurança (UAC)
Get-WinEvent -LogName Security -MaxEvents 20 | Where-Object {$_.Id -eq 4688}

# Ver processos do windows-mcp
Get-Process | Where-Object {$_.ProcessName -like "*windows-mcp*"}

# Testar comando manualmente com elevação
Start-Process powershell -Verb RunAs -ArgumentList "-Command", "net start W3SVC"

# Verificar se executável existe
Test-Path "C:\caminho\para\windows-mcp.exe"
```

---

## ⚠️ Segurança

O tool `execute_command` pode executar qualquer comando no sistema.

**Segurança implementada**:

- ✅ **UAC**: Autenticação via janela nativa do Windows - permissão nunca exposta
- ✅ **Sem armazenamento de credenciais**: Nenhuma senha ou token é armazenado
- ✅ **Auditoria**: Todos os comandos com UAC são registrados no Event Viewer
- ✅ **Controle**: Usuário decide permitir ou negar cada comando privilegiado

**Boas práticas**:

- Use UAC (`use_elevation: true`) para todos os comandos que precisam de administrador
- Monitore logs do Event Viewer regularmente
- Nunca execute o servidor com privilégios permanentes de administrador
- Revise comandos antes de permitir elevação via UAC

**Ver logs de auditoria**:

```powershell
# Abrir Event Viewer
eventvwr

# Ou via PowerShell
Get-WinEvent -LogName Security -MaxEvents 50 | Where-Object {$_.Id -eq 4688} | Format-Table TimeCreated, Message -AutoSize
```

---

## 📝 Compatibilidade

- **Windows 10** (versão 1809 ou superior)
- **Windows 11** (todas as versões)
- **Windows Server 2016+**

---

## 📝 Licença

Este projeto é de código aberto e está disponível sob a licença MIT.

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.

---

## 🆚 Diferenças entre Linux MCP e Windows MCP

| Recurso                 | Linux MCP             | Windows MCP                |
| ----------------------- | --------------------- | -------------------------- |
| Elevação de privilégios | PolicyKit (pkexec)    | UAC (User Account Control) |
| Shell padrão            | bash                  | cmd.exe / PowerShell       |
| Formato de caminhos     | `/home/user`          | `C:\Users\user`            |
| Variáveis de ambiente   | DISPLAY, XAUTHORITY   | Não necessário             |
| Diálogo de autenticação | Polkit Agent (GTK/Qt) | UAC nativo do Windows      |
| Logs de auditoria       | journalctl            | Event Viewer               |

---

## 🔗 Links Úteis

- [Documentação do UAC](https://docs.microsoft.com/en-us/windows/security/identity-protection/user-account-control/)
- [Model Context Protocol (MCP)](https://modelcontextprotocol.io/)
- [Claude Desktop](https://claude.ai/download)
