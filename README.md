# Linux MCP Server

Servidor MCP (Model Context Protocol) em Rust que fornece ferramentas para obter informações do sistema Linux e executar comandos no terminal.

## 📖 Documentação

- 🚀 **[Configurar no Claude Desktop](CLAUDE_DESKTOP_SETUP.md)** - Guia passo a passo completo
- 📋 **[Referência Rápida](QUICK_REFERENCE.md)** - Exemplos prontos de uso
- 🔐 **[Guia PolicyKit](examples/polkit/README_POLKIT.md)** - Configuração de segurança avançada

---

## ⚡ Quick Start - Executar comandos com root

**Para executar comandos que precisam de permissões de administrador** (como `apt update`, `systemctl restart`, etc.), você DEVE adicionar um método de elevação:

### ✅ Método Recomendado: PolicyKit (janela gráfica nativa)

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt",
    "args": ["update"],
    "use_polkit": true
  }
}
```

**O que acontece**: Uma janela NATIVA do seu sistema operacional aparece pedindo senha (igual quando você instala programas). Sua senha **nunca é enviada pelo MCP**.

### ⚠️ Erro Comum

❌ **ERRADO** (vai falhar com "Permission denied"):

```json
{
  "command": "apt",
  "args": ["update"]
}
```

✅ **CORRETO** (adicione `use_polkit: true`):

```json
{
  "command": "apt",
  "args": ["update"],
  "use_polkit": true
}
```

---

## 🚀 Funcionalidades

- **get_system_info**: Obtém informações detalhadas do sistema

  - CPU (contagem, marca, uso)
  - Memória (total, usada, disponível, swap)
  - Discos (espaço total, disponível, sistema de arquivos)
  - Sistema Operacional (nome, versão do kernel, hostname)

- **execute_command**: Executa comandos no terminal
  - Retorna stdout, stderr e código de saída
  - Suporta argumentos para comandos
  - **3 métodos de elevação de privilégios**:
    - **Normal**: executa com permissões do usuário atual
    - **sudo** (`use_sudo: true`): usa sudo (requer senha ou NOPASSWD)
    - **PolicyKit** (`use_polkit: true`): usa pkexec com diálogo gráfico (RECOMENDADO)
  - ⚠️ Use com cuidado - pode executar qualquer comando no sistema

## 📦 Compilação

```bash
cargo build --release
```

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
      "env": {
        "DISPLAY": ":0",
        "XAUTHORITY": "/home/seu_usuario/.Xauthority",
        "DBUS_SESSION_BUS_ADDRESS": "unix:path=/run/user/1000/bus"
      }
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
      "env": {
        "DISPLAY": ":0",
        "XAUTHORITY": "/home/marcos/.Xauthority",
        "DBUS_SESSION_BUS_ADDRESS": "unix:path=/run/user/1000/bus"
      }
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

### Executar comando normal (sem root)

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "ls",
    "args": ["-la", "/home"]
  }
}
```

### ⭐ Executar comando com PolicyKit (RECOMENDADO para root)

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt",
    "args": ["update"],
    "use_polkit": true
  }
}
```

**Resultado**: Janela nativa do sistema pede senha → comando executado com segurança ✅

### Executar comando com sudo (alternativa)

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl",
    "args": ["status", "nginx"],
    "use_sudo": true
  }
}
```

**Nota**: Requer NOPASSWD configurado no sudoers ou fornecer `sudo_password`

### Executar comando com sudo e senha

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt",
    "args": ["update"],
    "use_sudo": true,
    "sudo_password": "sua_senha_aqui"
  }
}
```

⚠️ **Aviso de Segurança**: Fornecer a senha em texto plano é um risco de segurança. Use apenas em ambientes controlados e considere usar NOPASSWD no sudoers ou PolicyKit para ambientes de produção.

### Executar comando com PolicyKit (RECOMENDADO) 🔐

**PolicyKit abre uma janela NATIVA do sistema para autenticação** - não expõe sua senha!

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt",
    "args": ["update"],
    "use_polkit": true
  }
}
```

Quando executado, uma **janela gráfica oficial do sistema** aparecerá pedindo sua senha de administrador (igual quando você instala programas pela Central de Aplicativos).

#### ⚠️ IMPORTANTE: Adicione `use_polkit: true` ou `use_sudo: true`

Comandos que precisam de root (como `apt update`, `systemctl restart`, etc.) **devem** incluir um método de elevação:

| Comando sem elevação ❌                                | Comando correto ✅                                                         |
| ------------------------------------------------------ | -------------------------------------------------------------------------- |
| `"command": "apt", "args": ["update"]`                 | `"command": "apt", "args": ["update"], "use_polkit": true`                 |
| `"command": "systemctl", "args": ["restart", "nginx"]` | `"command": "systemctl", "args": ["restart", "nginx"], "use_polkit": true` |

✅ **PolicyKit é mais seguro**: Apresenta um diálogo gráfico de autenticação e permite controle granular de permissões. Veja [Guia Completo de PolicyKit](examples/polkit/README_POLKIT.md) para instruções detalhadas.

## 🔐 Configuração de Permissões Root

Para executar comandos que precisam de permissões root, você tem 4 opções:

### Opção 1: PolicyKit/pkexec (⭐ RECOMENDADO)

**A opção mais segura e moderna**. PolicyKit permite:

- ✅ Diálogo gráfico de autenticação (não expõe senha)
- ✅ Controle granular por comando e usuário
- ✅ Auditoria completa no journal do sistema
- ✅ Timeout automático de credenciais
- ✅ Sem necessidade de configuração de sudoers

#### Instalação

```bash
# Ubuntu/Debian
sudo apt install polkitd policykit-1

# Fedora/RHEL
sudo dnf install polkit

# Arch Linux
sudo pacman -S polkit
```

#### Uso Básico

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl",
    "args": ["restart", "nginx"],
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

#### Configuração Avançada

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

### Opção 2: Fornecer senha do sudo (use_sudo + sudo_password)

### Opção 2: Fornecer senha do sudo (use_sudo + sudo_password)

**⚠️ Menos seguro - use apenas em dev/teste**. Você pode passar a senha diretamente:

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl",
    "args": ["restart", "nginx"],
    "use_sudo": true,
    "sudo_password": "sua_senha"
  }
}
```

⚠️ **Riscos**:

- A senha fica exposta nos logs do MCP
- Pode ser interceptada se a comunicação não estiver criptografada
- **Use apenas em ambientes de desenvolvimento/teste**

### Opção 3: Configurar sudo sem senha

### Opção 3: Configurar sudo sem senha

**Mais simples, mas menos granular que PolicyKit**. Edite o arquivo sudoers com `sudo visudo`:

```bash
# Permite que seu usuário execute comandos específicos sem senha
seu_usuario ALL=(ALL) NOPASSWD: /usr/bin/systemctl, /usr/bin/apt, /usr/bin/docker

# OU permite todos os comandos sem senha (menos seguro)
seu_usuario ALL=(ALL) NOPASSWD: ALL
```

Depois use sem fornecer a senha:

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl",
    "args": ["restart", "nginx"],
    "use_sudo": true
  }
}
```

### Opção 4: Executar o servidor como root (❌ NÃO recomendado)

```bash
sudo ./target/release/linux-mcp
```

⚠️ **Atenção**: Executar como root é uma prática de segurança ruim. Use apenas em ambientes controlados.

---

### 📊 Comparação de Métodos de Segurança

| Método                  | Segurança      | Facilidade | Interface | Auditoria | Uso Recomendado                 |
| ----------------------- | -------------- | ---------- | --------- | --------- | ------------------------------- |
| **PolicyKit (pkexec)**  | 🟢 **Alta**    | ✅ Fácil   | Gráfica   | ✅ Sim    | **Produção (RECOMENDADO)**      |
| senha via sudo_password | 🔴 Baixa       | ✅ Fácil   | Nenhuma   | ❌ Não    | Apenas dev/teste local          |
| NOPASSWD no sudoers     | 🟡 Média       | ✅ Fácil   | Terminal  | 🟡 Básica | Produção (comandos específicos) |
| Executar como root      | 🔴 Muito Baixa | ✅ Fácil   | Terminal  | ❌ Não    | **Nunca**                       |

### 🎯 Quando usar cada método?

#### Use PolicyKit quando:

- ✅ Estiver em ambiente com interface gráfica
- ✅ Precisar de controle granular de permissões
- ✅ Quiser auditoria completa de comandos privilegiados
- ✅ Não quiser expor senhas em logs

#### Use sudo com NOPASSWD quando:

- ✅ Estiver em servidor sem interface gráfica
- ✅ Comandos específicos precisam rodar automaticamente
- ✅ Ambiente controlado com poucos usuários

#### Use sudo com senha quando:

- ⚠️ Estiver em ambiente de desenvolvimento local
- ⚠️ For apenas testar funcionalidade rapidamente
- ❌ **NUNCA em produção ou ambientes compartilhados**

---

### Exemplo completo com PolicyKit

### Exemplo completo com PolicyKit

Configure PolicyKit (veja guia completo em `examples/polkit/`):

```bash
sudo cp examples/polkit/50-linux-mcp.rules /etc/polkit-1/rules.d/
sudo systemctl restart polkit
```

Ao chamar o tool `execute_command`, adicione `"use_polkit": true`:

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl",
    "args": ["restart", "nginx"],
    "use_polkit": true
  }
}
```

O comando será executado como: `pkexec systemctl restart nginx`

Uma janela de autenticação aparecerá solicitando senha do administrador.

### Exemplo com sudo (alternativa)

Se preferir usar sudo, adicione `"use_sudo": true`:

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt",
    "args": ["update"],
    "use_sudo": true
  }
}
```

O comando será executado como: `sudo systemctl restart nginx`

---

### Como funciona o PolicyKit?

PolicyKit (pkexec) funciona de maneira diferente do sudo:

1. **Cliente MCP** → envia `{"use_polkit": true}`
2. **Servidor** inicia: `pkexec systemctl restart nginx`
3. **PolicyKit** verifica regras em `/etc/polkit-1/rules.d/`
4. Se permitido, **mostra diálogo gráfico** pedindo senha
5. Usuário autentica → PolicyKit executa o comando
6. **Resultado** retorna ao cliente

**Vantagens**:

- ✅ Senha nunca passa pela rede ou logs
- ✅ Interface gráfica amigável
- ✅ Credenciais podem ser "lembradas" por alguns minutos
- ✅ Auditoria completa no journal: `journalctl -u polkit`

### Como funciona o sudo com senha?

Quando você fornece `sudo_password`, o servidor:

1. Executa `sudo -S comando` (o flag `-S` faz sudo ler senha do stdin)
2. Envia a senha pela entrada padrão do processo
3. O sudo autentica e executa o comando
4. Retorna o resultado normalmente

**Exemplo de fluxo**:

```
Cliente MCP → {"use_sudo": true, "sudo_password": "senha"}
           → Servidor inicia: sudo -S systemctl restart nginx
           → Servidor envia: "senha\n" para stdin do sudo
           → Sudo executa o comando
           → Resultado retorna ao cliente
```

## 🛠️ Tecnologias Utilizadas

- **rmcp**: SDK oficial do Model Context Protocol para Rust
- **tokio**: Runtime assíncrono para Rust
- **sysinfo**: Biblioteca para obter informações do sistema
- **serde & serde_json**: Serialização/deserialização JSON
- **anyhow**: Tratamento de erros

## 🔧 Troubleshooting PolicyKit

### ❌ Problema: Janela de autenticação não aparece

**Causa**: O MCP está rodando sem acesso à sessão gráfica.

**Solução 1: Use o wrapper script** (recomendado)

```bash
# O wrapper já está incluído no projeto
./linux-mcp-wrapper.sh
```

Configure no MCP para usar o wrapper:

```json
{
  "command": "/caminho/completo/para/linux-mcp-wrapper.sh"
}
```

**Solução 2: Configure variáveis de ambiente no MCP**

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

**Solução 3: Verificar se o agente polkit está rodando**

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

**Causa**: Você esqueceu de adicionar `use_polkit: true` ou `use_sudo: true`.

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
  "command": "apt",
  "args": ["update"],
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

O tool `execute_command` pode executar qualquer comando no sistema com as permissões do usuário que está executando o servidor. Use com responsabilidade:

- Nunca execute o servidor com privilégios elevados (root) a menos que seja absolutamente necessário
- Considere adicionar validação/whitelist de comandos para ambientes de produção
- Monitore os logs e atividades do servidor

## 📝 Licença

Este projeto é de código aberto e está disponível sob a licença MIT.

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.
