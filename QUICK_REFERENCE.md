# 📖 Referência Rápida - Linux MCP Server

## 🚀 Executar Comandos que Precisam de Root

### ✅ Método Recomendado: PolicyKit

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt update",
    "use_polkit": true
  }
}
```

**O que acontece:**

- 🪟 Janela NATIVA do sistema aparece pedindo senha
- 🔒 Senha nunca é enviada pelo MCP
- ✅ Mais seguro e intuitivo

---

## 📋 Exemplos Comuns

### Atualizar pacotes (Ubuntu/Debian)

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt update",
    "use_polkit": true
  }
}
```

### Instalar pacote

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "apt install -y nginx",
    "use_polkit": true
  }
}
```

### Reiniciar serviço

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl restart nginx",
    "use_polkit": true
  }
}
```

### Ver status de serviço

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl status nginx",
    "use_polkit": true
  }
}
```

### Ver logs do sistema

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "journalctl -n 50 -u nginx",
    "use_polkit": true
  }
}
```

### Gerenciar Docker

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "docker ps -a",
    "use_polkit": true
  }
}
```

### Ver configuração de rede

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "ip addr show",
    "use_polkit": true
  }
}
```

---

## 🔍 Comandos que NÃO precisam de root

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "ls -la /home"
  }
}
```

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "ps aux"
  }
}
```

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "df -h"
  }
}
```

---

## 📊 Obter Informações do Sistema

### Todas as informações

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "all"
  }
}
```

### Apenas CPU

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "cpu"
  }
}
```

### Apenas Memória

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "memory"
  }
}
```

### Apenas Discos

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "disk"
  }
}
```

### Apenas Sistema Operacional

```json
{
  "name": "get_system_info",
  "arguments": {
    "info_type": "os"
  }
}
```

---

## ⚠️ Erros Comuns

### ❌ Erro: "Permission denied"

**Problema**: Esqueceu de adicionar elevação de privilégios

**Solução**: Adicionar `use_polkit: true`

```json
// ❌ ERRADO
{
  "command": "apt update"
}

// ✅ CORRETO
{
  "command": "apt update",
  "use_polkit": true
}
```

---

### ❌ Janela de autenticação não aparece

**Soluções**:

1. **Use o wrapper script**:

   ```json
   {
     "command": "/caminho/para/linux-mcp-wrapper.sh"
   }
   ```

2. **Configure variáveis de ambiente**:

   ```json
   {
     "env": {
       "DISPLAY": ":0",
       "XAUTHORITY": "/home/seu_usuario/.Xauthority",
       "DBUS_SESSION_BUS_ADDRESS": "unix:path=/run/user/1000/bus"
     }
   }
   ```

3. **Verifique se polkit está instalado**:
   ```bash
   sudo apt install policykit-1 polkitd
   systemctl status polkit
   ```

---

## 🎯 Comparação Rápida

| Situação                                        | Use                                |
| ----------------------------------------------- | ---------------------------------- |
| Comando precisa de root + tem interface gráfica | `use_polkit: true` ⭐              |
| Comando precisa de root + servidor sem GUI      | `use_sudo: true` + NOPASSWD        |
| Comando normal (sem root)                       | Nenhum parâmetro extra             |
| Teste rápido local (dev)                        | `use_sudo: true` + `sudo_password` |
| **NUNCA em produção**                           | ❌ `sudo_password`                 |

---

## 📚 Documentação Completa

- [README Principal](README.md) - Documentação completa
- [Guia PolicyKit](examples/polkit/README_POLKIT.md) - Configuração avançada
- [Exemplos de Regras](examples/polkit/50-linux-mcp.rules) - Regras prontas

---

## 🆘 Precisa de Ajuda?

1. **Ver logs do PolicyKit**: `journalctl -u polkit -f`
2. **Testar manualmente**: `pkexec systemctl status nginx`
3. **Verificar instalação**: `which pkexec && systemctl status polkit`
4. **Ver guia completo**: Abra [examples/polkit/README_POLKIT.md](examples/polkit/README_POLKIT.md)
