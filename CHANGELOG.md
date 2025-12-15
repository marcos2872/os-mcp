# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.1.2] - 2025-12-15

### 🚀 Added
- **Dynamic Configuration**: Support for `~/.config/linux-mcp/config.toml` to customize the command allowlist without recompiling.
- **Audit Logging**: Comprehensive logging of all executed commands to `~/.config/linux-mcp/audit.log` (includes timestamp, command status, and user context).
- **Capability Discovery**: New resource `linux://mcp/capabilities` allowing agents to inspect active security rules and allowed commands programmatically.
- **Installation Script**: Added `install.sh` to automate build and Claude Desktop configuration injection.
- **GitHub Actions**: Added `release.yml` workflow for automated binary releases on tag push.
- **Documentation**: New `SECURITY_GUIDELINES.md` and `AGENTS.md` for better developer and agent guidance.

### 🛡️ Security
- **Command Allowlist**: Implemented strict allowlist blocking all commands by default unless listed in `config.toml` (or built-in defaults).
- **Safe RM Policy**: Restricted `rm` command usage to safe directories only (`/tmp`, `/var/log`, `~/.cache`, `Trash`). All other `rm` attempts are blocked.
- **PolicyKit Integration**: Enhanced `execute_command` to properly handle `use_polkit: true` for secure root authentication via OS native dialogs.

### 📚 Documentation
- **README Overhaul**: Simplified `README.md` with focus on quick start, JSON configuration examples, and security features.
- **Inspector Support**: Added instructions for testing via `@modelcontextprotocol/inspector`.
