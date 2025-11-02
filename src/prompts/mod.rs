use anyhow::Result;
use rmcp::model::{GetPromptResult, Prompt, PromptArgument, PromptMessage, PromptMessageRole};
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
            arguments: Some(vec![PromptArgument {
                name: "problem_type".to_string(),
                title: Some("Tipo de Problema".to_string()),
                description: Some(
                    "Tipo de problema: cpu, memory, disk, network, process".to_string(),
                ),
                required: Some(true),
            }]),
            icons: None,
        },
        Prompt {
            name: "security_audit".to_string(),
            title: Some("Security Audit".to_string()),
            description: Some("Realiza uma auditoria básica de segurança do sistema".to_string()),
            arguments: Some(vec![PromptArgument {
                name: "scope".to_string(),
                title: Some("Escopo".to_string()),
                description: Some("Escopo da auditoria: basic, full".to_string()),
                required: Some(false),
            }]),
            icons: None,
        },
        Prompt {
            name: "service_management".to_string(),
            title: Some("Service Management".to_string()),
            description: Some("Gerenciamento de serviços systemd".to_string()),
            arguments: Some(vec![
                PromptArgument {
                    name: "service_name".to_string(),
                    title: Some("Nome do Serviço".to_string()),
                    description: Some("Nome do serviço systemd".to_string()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "action".to_string(),
                    title: Some("Ação".to_string()),
                    description: Some(
                        "Ação: status, start, stop, restart, enable, disable".to_string(),
                    ),
                    required: Some(true),
                },
            ]),
            icons: None,
        },
        Prompt {
            name: "log_analysis".to_string(),
            title: Some("Log Analysis".to_string()),
            description: Some("Análise de logs do sistema com filtros e busca".to_string()),
            arguments: Some(vec![
                PromptArgument {
                    name: "log_type".to_string(),
                    title: Some("Tipo de Log".to_string()),
                    description: Some("Tipo de log: system, auth, kernel, app".to_string()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "priority".to_string(),
                    title: Some("Prioridade".to_string()),
                    description: Some(
                        "Prioridade mínima: emerg, alert, crit, err, warning, notice, info, debug"
                            .to_string(),
                    ),
                    required: Some(false),
                },
            ]),
            icons: None,
        },
        Prompt {
            name: "disk_cleanup".to_string(),
            title: Some("Disk Cleanup".to_string()),
            description: Some("Identificação e limpeza segura de espaço em disco".to_string()),
            arguments: Some(vec![PromptArgument {
                name: "aggressive".to_string(),
                title: Some("Agressivo".to_string()),
                description: Some("Modo agressivo (remove mais arquivos): true/false".to_string()),
                required: Some(false),
            }]),
            icons: None,
        },
    ]
}

pub fn get_prompt(name: &str, arguments: HashMap<String, String>) -> Result<GetPromptResult> {
    match name {
        "system_troubleshooting" => {
            let problem_type = arguments
                .get("problem_type")
                .map(|s| s.as_str())
                .unwrap_or("general");

            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Preciso diagnosticar um problema de {}. Por favor, me guie através de:\n\
                     1. Comandos para verificar o estado atual\n\
                     2. Interpretação dos resultados\n\
                     3. Possíveis soluções\n\
                     4. Como prevenir o problema no futuro",
                    problem_type
                ),
            )];

            Ok(GetPromptResult {
                description: Some(format!("Troubleshooting de problema: {}", problem_type)),
                messages,
            })
        }
        "security_audit" => {
            let scope = arguments
                .get("scope")
                .map(|s| s.as_str())
                .unwrap_or("basic");

            let mut checks = vec![
                "1. Verificar atualizações pendentes de segurança",
                "2. Listar usuários com login ativo",
                "3. Verificar portas abertas e serviços expostos",
                "4. Checar logs de autenticação falhada",
            ];

            if scope == "full" {
                checks.extend_from_slice(&[
                    "5. Verificar permissões de arquivos críticos",
                    "6. Listar processos com privilégios elevados",
                    "7. Verificar configurações do firewall",
                    "8. Analisar integridade de pacotes do sistema",
                ]);
            }

            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Execute uma auditoria de segurança ({} scope):\n\n{}",
                    scope,
                    checks.join("\n")
                ),
            )];

            Ok(GetPromptResult {
                description: Some(format!("Auditoria de segurança - escopo: {}", scope)),
                messages,
            })
        }
        "service_management" => {
            let service_name = arguments
                .get("service_name")
                .ok_or_else(|| anyhow::anyhow!("service_name is required"))?;

            let action = arguments
                .get("action")
                .ok_or_else(|| anyhow::anyhow!("action is required"))?;

            let command = match action.as_str() {
                "status" => format!("systemctl status {}", service_name),
                "start" => format!("systemctl start {}", service_name),
                "stop" => format!("systemctl stop {}", service_name),
                "restart" => format!("systemctl restart {}", service_name),
                "enable" => format!("systemctl enable {}", service_name),
                "disable" => format!("systemctl disable {}", service_name),
                _ => return Err(anyhow::anyhow!("Invalid action: {}", action)),
            };

            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Execute a seguinte ação no serviço '{}':\n\
                     Ação: {}\n\
                     Comando: {}\n\n\
                     Por favor:\n\
                     1. Execute o comando\n\
                     2. Verifique o resultado\n\
                     3. Confirme o novo status do serviço",
                    service_name, action, command
                ),
            )];

            Ok(GetPromptResult {
                description: Some(format!("Gerenciar serviço: {} - {}", service_name, action)),
                messages,
            })
        }
        "log_analysis" => {
            let log_type = arguments
                .get("log_type")
                .ok_or_else(|| anyhow::anyhow!("log_type is required"))?;

            let priority = arguments
                .get("priority")
                .map(|s| s.as_str())
                .unwrap_or("info");

            let command = match log_type.as_str() {
                "system" => format!("journalctl -p {} -n 100 --no-pager", priority),
                "auth" => format!("journalctl -u ssh -u sshd -p {} -n 50 --no-pager", priority),
                "kernel" => format!("journalctl -k -p {} -n 100 --no-pager", priority),
                "app" => {
                    let app_name = arguments
                        .get("app_name")
                        .map(|s| s.as_str())
                        .unwrap_or("apache2");
                    format!(
                        "journalctl -u {} -p {} -n 100 --no-pager",
                        app_name, priority
                    )
                }
                _ => return Err(anyhow::anyhow!("Invalid log_type: {}", log_type)),
            };

            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Analise os logs do tipo '{}' com prioridade mínima '{}':\n\n\
                     Comando sugerido: {}\n\n\
                     Por favor:\n\
                     1. Execute o comando\n\
                     2. Identifique padrões ou problemas\n\
                     3. Sugira ações corretivas se necessário",
                    log_type, priority, command
                ),
            )];

            Ok(GetPromptResult {
                description: Some(format!(
                    "Análise de logs: {} (prioridade: {})",
                    log_type, priority
                )),
                messages,
            })
        }
        "disk_cleanup" => {
            let aggressive = arguments
                .get("aggressive")
                .map(|s| s == "true")
                .unwrap_or(false);

            let mut cleanup_steps = vec![
                "1. Limpar cache do apt: apt clean",
                "2. Remover pacotes órfãos: apt autoremove",
                "3. Limpar journald logs antigos: journalctl --vacuum-time=7d",
                "4. Encontrar arquivos grandes: find / -type f -size +100M",
            ];

            if aggressive {
                cleanup_steps.extend_from_slice(&[
                    "5. Limpar cache do pip: pip cache purge",
                    "6. Limpar arquivos temporários: rm -rf /tmp/*",
                    "7. Limpar cache do npm: npm cache clean --force",
                    "8. Limpar logs rotacionados antigos: find /var/log -name '*.gz' -delete",
                ]);
            }

            let messages = vec![PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Executar limpeza de disco (modo {}):\n\n{}\n\n\
                     ATENÇÃO: Revise cada comando antes de executar!",
                    if aggressive {
                        "agressivo"
                    } else {
                        "conservador"
                    },
                    cleanup_steps.join("\n")
                ),
            )];

            Ok(GetPromptResult {
                description: Some(format!(
                    "Limpeza de disco - modo: {}",
                    if aggressive {
                        "agressivo"
                    } else {
                        "conservador"
                    }
                )),
                messages,
            })
        }
        _ => Err(anyhow::anyhow!("Unknown prompt: {}", name)),
    }
}
