
fn main() {
    // Simulator for is_safe_rm logic (copied from implementation for verification)
    fn is_safe_rm(command_line: &str) -> bool {
        let parts: Vec<&str> = command_line.trim().split_whitespace().collect();
        let targets: Vec<&str> = parts.iter()
            .skip(1)
            .filter(|arg| !arg.starts_with('-'))
            .map(|s| *s)
            .collect();

        if targets.is_empty() { return false; }

        for target in targets {
            if target.contains("..") { return false; }
            let is_safe = target.starts_with("/tmp/") ||
                          target.starts_with("/var/tmp/") ||
                          target.starts_with("/var/log/") ||
                          target.contains("/.cache/") ||
                          target.contains("/.local/share/Trash/");
            if !is_safe { return false; }
        }
        true
    }

    let tests = vec![
        // Allowed
        ("rm -rf /tmp/junk", true),
        ("rm /var/log/syslog.1", true),
        ("rm -f /home/user/.cache/mozilla/firefox/cache2", true),
        ("rm /home/user/.local/share/Trash/files/deleted.txt", true),
        ("rm -rf /var/tmp/temp_dir", true),
        
        // Blocked
        ("rm -rf /", false),
        ("rm /etc/passwd", false),
        ("rm /home/user/Documents/secret.txt", false),
        ("rm -rf /tmp/../etc/passwd", false),
        ("rm", false),
        ("rm -rf", false),
        ("rm /tmp/safe /etc/unsafe", false), // One unsafe fails all
    ];

    let mut failed = false;
    for (cmd, expected) in tests {
        let result = is_safe_rm(cmd);
        if result != expected {
            println!("FAIL: '{}' -> got {}, expected {}", cmd, result, expected);
            failed = true;
        } else {
            println!("PASS: '{}'", cmd);
        }
    }

    if failed {
        std::process::exit(1);
    }
}
