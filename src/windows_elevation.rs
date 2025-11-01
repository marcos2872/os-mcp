/// Módulo para elevação de privilégios no Windows usando UAC
/// 
/// Este módulo fornece funções para executar comandos com privilégios administrativos
/// no Windows usando o mecanismo UAC (User Account Control).

#[cfg(windows)]
use std::process::Command;
use anyhow::{Result, anyhow};

/// Executa um comando com elevação de privilégios usando UAC
/// 
/// No Windows, isso usa o comando `runas` ou tenta executar via ShellExecute com "runas"
/// O UAC exibirá um prompt nativo do Windows solicitando permissão do usuário.
/// 
/// # Arguments
/// 
/// * `command` - O comando a ser executado
/// * `args` - Argumentos opcionais para o comando
/// 
/// # Returns
/// 
/// Retorna uma tupla com (exit_code, stdout, stderr, success)
#[cfg(windows)]
pub fn execute_elevated(command: &str, args: Option<&Vec<String>>) -> Result<(i32, String, String, bool)> {
    use std::os::windows::process::CommandExt;
    use windows::Win32::System::Threading::CREATE_NO_WINDOW;

    // Verificar se já estamos executando como administrador
    if is_elevated()? {
        // Já somos admin, executar diretamente
        return execute_direct(command, args);
    }

    // Precisamos elevar - usar PowerShell com Start-Process -Verb RunAs
    // Isso dispara o UAC prompt
    let cmd_line = build_command_line(command, args);
    
    // Construir comando PowerShell para elevação
    let ps_script = format!(
        "Start-Process -FilePath '{}' -ArgumentList {} -Verb RunAs -Wait -WindowStyle Hidden",
        command,
        escape_powershell_args(args)
    );

    let mut cmd = Command::new("powershell.exe");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    
    // CREATE_NO_WINDOW evita janela de console extra
    cmd.creation_flags(CREATE_NO_WINDOW.0);

    match cmd.output() {
        Ok(output) => {
            let exit_code = output.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let success = output.status.success();
            
            Ok((exit_code, stdout, stderr, success))
        }
        Err(e) => Err(anyhow!("Failed to execute elevated command: {}. Make sure you're running in an environment that supports UAC dialogs.", e))
    }
}

/// Executa comando diretamente sem elevação (fallback para sistemas não-Windows)
#[cfg(not(windows))]
pub fn execute_elevated(command: &str, args: Option<&Vec<String>>) -> Result<(i32, String, String, bool)> {
    Err(anyhow!("Elevated execution is only supported on Windows"))
}

/// Executa um comando diretamente sem elevação
fn execute_direct(command: &str, args: Option<&Vec<String>>) -> Result<(i32, String, String, bool)> {
    let mut cmd = Command::new(command);
    
    if let Some(cmd_args) = args {
        cmd.args(cmd_args);
    }

    match cmd.output() {
        Ok(output) => {
            let exit_code = output.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let success = output.status.success();
            
            Ok((exit_code, stdout, stderr, success))
        }
        Err(e) => Err(anyhow!("Failed to execute command: {}", e))
    }
}

/// Verifica se o processo atual está sendo executado com privilégios de administrador
#[cfg(windows)]
fn is_elevated() -> Result<bool> {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = HANDLE::default();
        
        // Abrir o token do processo atual
        if !OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            return Ok(false);
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length = 0u32;

        // Verificar se o token está elevado
        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );

        if !result.is_ok() {
            return Ok(false);
        }

        Ok(elevation.TokenIsElevated != 0)
    }
}

#[cfg(not(windows))]
fn is_elevated() -> Result<bool> {
    Ok(false)
}

/// Constrói a linha de comando completa
fn build_command_line(command: &str, args: Option<&Vec<String>>) -> String {
    if let Some(cmd_args) = args {
        format!("{} {}", command, cmd_args.join(" "))
    } else {
        command.to_string()
    }
}

/// Escapa argumentos para uso em PowerShell
fn escape_powershell_args(args: Option<&Vec<String>>) -> String {
    if let Some(cmd_args) = args {
        let escaped: Vec<String> = cmd_args
            .iter()
            .map(|arg| {
                // Envolver em aspas simples e escapar aspas simples internas
                format!("'{}'", arg.replace("'", "''"))
            })
            .collect();
        escaped.join(",")
    } else {
        "''".to_string()
    }
}

/// Executa um comando usando cmd.exe (alternativa ao PowerShell)
#[cfg(windows)]
pub fn execute_with_cmd(command: &str, args: Option<&Vec<String>>) -> Result<(i32, String, String, bool)> {
    let cmd_line = build_command_line(command, args);
    
    let mut cmd = Command::new("cmd.exe");
    cmd.args(&["/C", &cmd_line]);

    match cmd.output() {
        Ok(output) => {
            let exit_code = output.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let success = output.status.success();
            
            Ok((exit_code, stdout, stderr, success))
        }
        Err(e) => Err(anyhow!("Failed to execute command with cmd.exe: {}", e))
    }
}

#[cfg(not(windows))]
pub fn execute_with_cmd(command: &str, args: Option<&Vec<String>>) -> Result<(i32, String, String, bool)> {
    Err(anyhow!("cmd.exe is only available on Windows"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command_line() {
        assert_eq!(build_command_line("echo", None), "echo");
        assert_eq!(
            build_command_line("echo", Some(&vec!["hello".to_string(), "world".to_string()])),
            "echo hello world"
        );
    }

    #[test]
    fn test_escape_powershell_args() {
        assert_eq!(escape_powershell_args(None), "''");
        assert_eq!(
            escape_powershell_args(Some(&vec!["test".to_string()])),
            "'test'"
        );
        assert_eq!(
            escape_powershell_args(Some(&vec!["test's".to_string()])),
            "'test''s'"
        );
    }
}
