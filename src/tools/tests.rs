
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_command_allowed() {
        // Allowed commands
        assert!(is_command_allowed("ls -la"));
        assert!(is_command_allowed("grep 'foo' bar.txt"));
        assert!(is_command_allowed("apt update"));
        assert!(is_command_allowed("/usr/bin/ls")); // Absolute path
        
        // Blocked commands
        assert!(!is_command_allowed("rm -rf /"));
        assert!(!is_command_allowed("chmod 777 file"));
        assert!(!is_command_allowed("./script.sh"));
        assert!(!is_command_allowed("python3 script.py"));
    }
}
