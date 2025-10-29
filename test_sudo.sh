#!/bin/bash

# Script de teste para o comando com sudo

echo "=== Teste 1: Comando simples sem sudo ==="
echo "Comando: whoami"
echo ""

echo "=== Teste 2: Comando com sudo (precisa configurar NOPASSWD ou fornecer senha) ==="
echo "Exemplo de chamada MCP:"
cat << 'EOF'
{
  "name": "execute_command",
  "arguments": {
    "command": "whoami",
    "use_sudo": true
  }
}
EOF
echo ""

echo "=== Teste 3: Comando com sudo e senha ==="
echo "Exemplo de chamada MCP:"
cat << 'EOF'
{
  "name": "execute_command",
  "arguments": {
    "command": "apt",
    "args": ["list", "--installed"],
    "use_sudo": true,
    "sudo_password": "SUA_SENHA_AQUI"
  }
}
EOF
echo ""

echo "=== Aviso de Segurança ==="
echo "⚠️  NUNCA compartilhe sua senha em produção!"
echo "⚠️  Use NOPASSWD no sudoers para ambientes seguros"
echo ""

echo "Para testar manualmente, execute:"
echo "echo 'sua_senha' | sudo -S whoami"
