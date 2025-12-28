use anyhow::Result;
use rmcp::model::{GetPromptResult, Prompt, PromptMessage, PromptMessageRole};
use std::collections::HashMap;

pub fn list_prompts() -> Vec<Prompt> {
    vec![
        Prompt {
            name: "system_troubleshooting".to_string(),
            title: Some("System Troubleshooting".to_string()),
            description: Some(
                "Guia interativo para diagnóstico e solução de problemas no sistema Linux"
                    .to_string(),
            ),
            arguments: None,
            icons: None,
        },
        Prompt {
            name: "security_audit".to_string(),
            title: Some("Security Audit".to_string()),
            description: Some("Realiza uma auditoria básica de segurança do sistema".to_string()),
            arguments: None,
            icons: None,
        },
        Prompt {
            name: "service_management".to_string(),
            title: Some("Service Management".to_string()),
            description: Some("Gerenciamento de serviços do sistema".to_string()),
            arguments: None,
            icons: None,
        },
        Prompt {
            name: "log_analysis".to_string(),
            title: Some("Log Analysis".to_string()),
            description: Some("Análise de logs do sistema com filtros e busca".to_string()),
            arguments: None,
            icons: None,
        },
        Prompt {
            name: "disk_cleanup".to_string(),
            title: Some("Disk Cleanup".to_string()),
            description: Some("Identificação e limpeza segura de espaço em disco".to_string()),
            arguments: None,
            icons: None,
        },
    ]
}

pub fn get_prompt(name: &str, _arguments: HashMap<String, String>) -> Result<GetPromptResult> {
    match name {
        "system_troubleshooting" => {
            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                "Preciso diagnosticar um problema no sistema Linux. Por favor, me ajude a:\n\n\
                 1. Identificar o tipo de problema (CPU, memória, disco, rede, processos, etc.)\n\
                 2. Executar comandos de diagnóstico apropriados\n\
                 3. Interpretar os resultados\n\
                 4. Sugerir soluções baseadas nos dados coletados\n\
                 5. Recomendar medidas preventivas\n\n\
                 Use comandos compatíveis com qualquer distribuição Linux (ps, top, df, free, dmesg, etc.)".to_string(),
            )];

            Ok(GetPromptResult {
                description: Some("Guia de troubleshooting para sistema Linux".to_string()),
                messages,
            })
        }
        "security_audit" => {
            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                "Execute uma auditoria de segurança abrangente do sistema Linux:\n\n\
                 1. Verificar atualizações pendentes (detectar gerenciador de pacotes automaticamente)\n\
                 2. Listar usuários e verificar contas suspeitas\n\
                 3. Verificar portas abertas e serviços em execução\n\
                 4. Analisar logs de autenticação e tentativas de login\n\
                 5. Verificar permissões de arquivos críticos (/etc/passwd, /etc/shadow, etc.)\n\
                 6. Listar processos com privilégios elevados\n\
                 7. Verificar configurações de firewall (iptables/nftables/ufw)\n\
                 8. Identificar pacotes e arquivos modificados\n\n\
                 Use comandos genéricos que funcionem em qualquer distro Linux.".to_string(),
            )];

            Ok(GetPromptResult {
                description: Some("Auditoria de segurança do sistema".to_string()),
                messages,
            })
        }
        "service_management" => {
            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                "Ajude-me a gerenciar serviços no sistema Linux:\n\n\
                 1. Detectar o sistema de init usado (systemd, sysvinit, OpenRC, etc.)\n\
                 2. Listar todos os serviços disponíveis\n\
                 3. Verificar status de serviços específicos\n\
                 4. Iniciar, parar ou reiniciar serviços conforme necessário\n\
                 5. Habilitar ou desabilitar serviços no boot\n\
                 6. Analisar logs dos serviços\n\
                 7. Diagnosticar problemas de serviços que não iniciam\n\n\
                 Adapte os comandos ao sistema de init detectado."
                    .to_string(),
            )];

            Ok(GetPromptResult {
                description: Some("Gerenciamento de serviços Linux".to_string()),
                messages,
            })
        }
        "log_analysis" => {
            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                "Analise logs do sistema Linux de forma abrangente:\n\n\
                 1. Detectar sistema de logs (journald, rsyslog, syslog-ng, etc.)\n\
                 2. Analisar logs do sistema geral\n\
                 3. Verificar logs de autenticação e segurança\n\
                 4. Analisar logs do kernel (dmesg)\n\
                 5. Verificar logs de serviços específicos\n\
                 6. Identificar erros, avisos e mensagens críticas\n\
                 7. Buscar padrões e anomalias\n\
                 8. Sugerir ações corretivas baseadas nos logs\n\n\
                 Use comandos genéricos (grep, tail, journalctl quando disponível, /var/log/, etc.)".to_string(),
            )];

            Ok(GetPromptResult {
                description: Some("Análise de logs do sistema".to_string()),
                messages,
            })
        }
        "disk_cleanup" => {
            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                "Ajude-me a liberar espaço em disco de forma segura:\n\n\
                 1. Analisar uso de disco (df, du) e identificar diretórios grandes\n\
                 2. Detectar gerenciador de pacotes e limpar cache\n\
                 3. Remover pacotes órfãos (se aplicável)\n\
                 4. Limpar logs antigos de forma segura\n\
                 5. Identificar arquivos grandes e duplicados\n\
                 6. Limpar diretórios temporários seguros (/tmp, /var/tmp)\n\
                 7. Limpar cache do usuário (~/.cache)\n\
                 8. Esvaziar lixeira (~/.local/share/Trash)\n\n\
                 IMPORTANTE:\n\
                 - Sempre confirme antes de deletar arquivos\n\
                 - Evite usar rm em locais não autorizados\n\
                 - Adapte comandos ao gerenciador de pacotes detectado\n\
                 - NÃO execute limpeza em discos removíveis (USB, SD cards, discos externos)\n\
                 - Verifique com 'df -h' ou 'lsblk' para identificar pontos de montagem removíveis"
                    .to_string(),
            )];

            Ok(GetPromptResult {
                description: Some("Limpeza segura de espaço em disco".to_string()),
                messages,
            })
        }
        _ => Err(anyhow::anyhow!("Unknown prompt: {}", name)),
    }
}
