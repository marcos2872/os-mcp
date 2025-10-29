# Linux MCP Server

Servidor MCP (Model Context Protocol) em Rust que fornece ferramentas para obter informações do sistema Linux e executar comandos no terminal.

## 🚀 Funcionalidades

- **get_system_info**: Obtém informações detalhadas do sistema

  - CPU (contagem, marca, uso)
  - Memória (total, usada, disponível, swap)
  - Discos (espaço total, disponível, sistema de arquivos)
  - Sistema Operacional (nome, versão do kernel, hostname)

- **execute_command**: Executa comandos no terminal
  - Retorna stdout, stderr e código de saída
  - Suporta argumentos para comandos
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

Adicione ao arquivo de configuração do Claude Desktop (`~/Library/Application Support/Claude/claude_desktop_config.json` no macOS ou `%APPDATA%/Claude/claude_desktop_config.json` no Windows):

```json
{
  "mcpServers": {
    "linux-info": {
      "command": "/caminho/completo/para/linux-mcp",
      "args": []
    }
  }
}
```

### Integração com Cursor IDE

Adicione ao arquivo `.cursor/mcp_config.json` no seu projeto:

```json
{
  "mcpServers": {
    "linux-info": {
      "command": "/caminho/completo/para/linux-mcp",
      "args": []
    }
  }
}
```

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

### Executar comando

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "ls",
    "args": ["-la", "/home"]
  }
}
```

## 🛠️ Tecnologias Utilizadas

- **rmcp**: SDK oficial do Model Context Protocol para Rust
- **tokio**: Runtime assíncrono para Rust
- **sysinfo**: Biblioteca para obter informações do sistema
- **serde & serde_json**: Serialização/deserialização JSON
- **anyhow**: Tratamento de erros

## ⚠️ Segurança

O tool `execute_command` pode executar qualquer comando no sistema com as permissões do usuário que está executando o servidor. Use com responsabilidade:

- Nunca execute o servidor com privilégios elevados (root) a menos que seja absolutamente necessário
- Considere adicionar validação/whitelist de comandos para ambientes de produção
- Monitore os logs e atividades do servidor

## 📝 Licença

Este projeto é de código aberto e está disponível sob a licença MIT.

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.
