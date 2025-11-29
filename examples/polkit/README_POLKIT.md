# Configurando PolicyKit para Linux MCP

PolicyKit (polkit) é um sistema de autorização que fornece controle granular sobre privilégios de sistema no Linux. É mais seguro que sudo porque:

1. **Autenticação visual**: Mostra diálogos gráficos pedindo senha
2. **Controle granular**: Você pode permitir comandos específicos sem dar acesso total
3. **Auditoria**: Todas as ações são registradas no journal do sistema
4. **Timeout de autenticação**: Credenciais expiram automaticamente

## 📋 Pré-requisitos

### Instalar PolicyKit

#### Ubuntu/Debian

```bash
sudo apt install polkitd policykit-1
```

#### Fedora/RHEL

```bash
sudo dnf install polkit
```

#### Arch Linux

```bash
sudo pacman -S polkit
```

### Verificar instalação

```bash
which pkexec
systemctl status polkit
```

## 🚀 Configuração Básica

### 1. Criar arquivo de regras

Copie o arquivo de exemplo para o diretório de regras do PolicyKit:

```bash
sudo cp examples/polkit/50-linux-mcp.rules /etc/polkit-1/rules.d/
```

### 2. Ajustar permissões

```bash
sudo chmod 644 /etc/polkit-1/rules.d/50-linux-mcp.rules
sudo chown root:root /etc/polkit-1/rules.d/50-linux-mcp.rules
```

### 3. Reiniciar o serviço polkit

```bash
sudo systemctl restart polkit
```

## 📝 Personalizar Regras

Edite o arquivo `/etc/polkit-1/rules.d/50-linux-mcp.rules` e substitua `seu_usuario` pelo seu nome de usuário:

```bash
sudo nano /etc/polkit-1/rules.d/50-linux-mcp.rules
```

### Exemplo: Permitir systemctl sem senha

```javascript
polkit.addRule(function (action, subject) {
  if (
    action.id == "org.freedesktop.policykit.exec" &&
    action.lookup("program") == "/usr/bin/systemctl" &&
    subject.user == "marcos"
  ) {
    return polkit.Result.YES;
  }
});
```

### Exemplo: Permitir comandos específicos para um grupo

```javascript
polkit.addRule(function (action, subject) {
  if (
    action.id == "org.freedesktop.policykit.exec" &&
    subject.isInGroup("admin")
  ) {
    var allowed = ["/usr/bin/systemctl", "/usr/bin/docker"];
    if (allowed.indexOf(action.lookup("program")) !== -1) {
      return polkit.Result.YES;
    }
  }
});
```

## 🔍 Como usar no MCP

### Executar comando com PolicyKit

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl status nginx",
    "use_polkit": true
  }
}
```

Isso abrirá uma janela de autenticação gráfica solicitando a senha do administrador.

### Comparação: sudo vs PolicyKit

#### Com sudo (senha em texto plano - INSEGURO)

```json
{
  "command": "systemctl restart nginx",
  "use_sudo": true,
  "sudo_password": "minhasenha123"
}
```

#### Com PolicyKit (diálogo gráfico - SEGURO)

```json
{
  "command": "systemctl restart nginx",
  "use_polkit": true
}
```

## 🛡️ Níveis de Autorização

PolicyKit oferece diferentes níveis de autorização:

| Nível             | Descrição                          | Uso                                        |
| ----------------- | ---------------------------------- | ------------------------------------------ |
| `YES`             | Permite sem senha                  | Comandos seguros para usuários específicos |
| `NO`              | Nega completamente                 | Bloquear ações perigosas                   |
| `AUTH_ADMIN`      | Requer senha de admin (uma vez)    | Ações administrativas                      |
| `AUTH_ADMIN_KEEP` | Requer senha de admin (lembrada)   | Múltiplas ações admin                      |
| `AUTH_SELF`       | Requer senha do usuário            | Ações do próprio usuário                   |
| `AUTH_SELF_KEEP`  | Requer senha do usuário (lembrada) | Múltiplas ações do usuário                 |

## 🔍 Debug e Testes

### Testar manualmente uma regra

```bash
pkexec systemctl status nginx
```

### Ver logs do PolicyKit

```bash
sudo journalctl -u polkit -f
```

### Listar todas as ações disponíveis

```bash
pkaction
```

### Ver detalhes de uma ação específica

```bash
pkaction --verbose --action-id org.freedesktop.policykit.exec
```

## 🎯 Ações PolicyKit Comuns

### Gerenciamento de Serviços

- `org.freedesktop.systemd1.manage-units` - Controlar serviços systemd
- `org.freedesktop.systemd1.reload-daemon` - Recarregar daemon do systemd

### Gerenciamento de Rede

- `org.freedesktop.NetworkManager.network-control` - Controlar NetworkManager
- `org.freedesktop.NetworkManager.settings.modify.system` - Modificar configurações de rede

### Gerenciamento de Pacotes

- `org.debian.apt.install-or-remove-packages` - Instalar/remover pacotes (Debian/Ubuntu)

### Execução Geral

- `org.freedesktop.policykit.exec` - Executar comandos via pkexec

## ⚠️ Considerações de Segurança

### ✅ Boas Práticas

- Sempre especifique o caminho completo do comando (`/usr/bin/systemctl`)
- Use `subject.user` para limitar a usuários específicos
- Prefira `AUTH_ADMIN_KEEP` a `YES` para comandos críticos
- Teste regras em ambiente de desenvolvimento primeiro
- Mantenha logs habilitados para auditoria

### ❌ Evite

- Não use `return polkit.Result.YES` para todos os comandos
- Não permita comandos genéricos como `/bin/bash` ou `/bin/sh`
- Não desabilite autenticação para comandos que modificam o sistema
- Não confie apenas em verificações de grupo para ações críticas

## 🐛 Solução de Problemas

### Erro: "Authentication agent not available"

**Causa**: Você está em um ambiente sem interface gráfica

**Solução**:

```bash
# Verifique se há um agente polkit rodando
ps aux | grep polkit

# Para GNOME
/usr/libexec/polkit-gnome-authentication-agent-1 &

# Para KDE
/usr/lib/polkit-kde-authentication-agent-1 &

# Para XFCE
/usr/lib/xfce-polkit/xfce-polkit &
```

### Erro: "Not authorized"

**Causa**: Regras não estão configuradas ou sintaxe incorreta

**Solução**:

```bash
# Verificar sintaxe das regras
sudo pkaction --verbose | grep -i error

# Ver logs detalhados
sudo journalctl -u polkit -n 50

# Testar regra manualmente
pkexec --user root /caminho/do/comando
```

### Regras não são aplicadas

**Causa**: Arquivo não foi recarregado

**Solução**:

```bash
sudo systemctl restart polkit
# ou
sudo killall -HUP polkitd
```

## 📚 Recursos Adicionais

- [PolicyKit Manual](https://www.freedesktop.org/software/polkit/docs/latest/index.html)
- [PolicyKit JavaScript API](https://www.freedesktop.org/software/polkit/docs/latest/polkit.8.html)
- [Arch Wiki - PolicyKit](https://wiki.archlinux.org/title/Polkit)

## 🎓 Exemplo Completo de Uso

### 1. Criar a regra

```bash
sudo tee /etc/polkit-1/rules.d/50-linux-mcp.rules << 'EOF'
polkit.addRule(function(action, subject) {
    if (action.id == "org.freedesktop.policykit.exec" &&
        subject.user == "marcos") {

        var allowed = [
            "/usr/bin/systemctl",
            "/usr/bin/docker",
            "/usr/bin/apt"
        ];

        if (allowed.indexOf(action.lookup("program")) !== -1) {
            return polkit.Result.AUTH_ADMIN_KEEP;
        }
    }
});
EOF

sudo chmod 644 /etc/polkit-1/rules.d/50-linux-mcp.rules
sudo systemctl restart polkit
```

### 2. Testar no terminal

```bash
pkexec systemctl status nginx
# Abrirá um diálogo pedindo senha
```

### 3. Usar no MCP

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "systemctl restart nginx",
    "use_polkit": true
  }
}
```

Pronto! Agora você tem controle seguro e granular sobre privilégios de sistema. 🎉
