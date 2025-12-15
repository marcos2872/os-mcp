# 🛡️ Diretrizes de Segurança do Linux MCP

Este projeto foi desenhado sob o princípio de **"Privilégio Mínimo"** e **"Bloqueio por Padrão"**.
Este documento descreve as camadas de segurança implementadas para proteger o sistema contra ações maliciosas ou acidentais de agentes de IA.

---

## 🔒 1. Allowlist (Lista de Comandos Permitidos)

O servidor **rejeita** qualquer comando que não esteja explicitamente listado na `ALLOWLIST`.
Isso impede a execução de ferramentas perigosas como `dd`, `mkfs`, `nc` (netcat), shells reversos, ou compiladores.

**Categorias Permitidas:**
- **Informação**: `ls`, `cat`, `grep`, `find`, `ps`, `top`, `df`, `du`, `uname`...
- **Logs**: `journalctl`, `dmesg`, `tail`, `head`...
- **Rede (Leitura)**: `ip`, `ifconfig`, `ss`, `netstat`...
- **Serviços**: `systemctl`, `service`
- **Pacotes**: `apt`, `dnf`, `snap`, `flatpak`...

> 💡 **Consulta**: Para ver a lista exata e atualizada de comandos permitidos, leia o resource `linux://mcp/capabilities`.

---

## 🗑️ 2. Política de "Safe RM"

O comando `rm` (remoção) é **bloqueado por padrão** para evitar a exclusão acidental de arquivos do sistema ou dados do usuário.

Ele é permitido **APENAS** se todos os alvos obedecerem a estas regras estritas:

| Diretório Permitido | Propósito |
|---------------------|-----------|
| `/tmp/*` | Arquivos temporários |
| `/var/tmp/*` | Arquivos temporários do sistema |
| `/var/log/*` | Rotação e limpeza de logs |
| `~/.cache/*` | Caches de aplicativos do usuário |
| `~/.local/share/Trash/*` | Esvaziar lixeira |

**Regras Adicionais:**
- 🚫 Proibido uso de `..` (parent traversal).
- 🚫 Proibido qualquer outro caminho (`/etc`, `/home/user/documentos`, etc.).

---

## 🔑 3. Autenticação via PolicyKit

O MCP **nunca** recebe, armazena ou digita a senha de root/sudo.

- Quando um comando requer privilégios (ex: `apt update`), o agente deve enviar `use_polkit: true`.
- O servidor invoca o `pkexec`.
- O **Sistema Operacional** abre uma janela gráfica nativa (fora do controle do MCP).
- O **Usuário Humano** digita a senha na janela segura do sistema.

Se o usuário cancelar ou errar a senha, o comando falha e o agente é notificado.

---

## 🤖 4. Recursos para Agentes

Implementamos recursos de auto-documentação para que o agente possa entender seus limites:

- **Resource `linux://mcp/capabilities`**:
  Retorna um manifesto completo do que é permitido e proibido. Agentes são instruídos a ler este arquivo antes de planejar tarefas complexas.

---

## 🚫 5. O que está Bloqueado (Exemplos)

- **Exfiltração de Dados**: `curl`, `wget`, `ssh`, `scp` (bloqueados para impedir envio de dados para fora).
- **Acesso a Segredos**: Leitura de `/etc/shadow`, chaves SSH, variáveis de ambiente.
- **Destruição**: Formatação de disco, sobrescrita de dispositivos (`/dev/sda`).
- **Ofuscação**: Execução de base64 ou scripts pipeados da internet.
