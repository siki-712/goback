use std::process::Command;

pub fn get_reflog() -> Result<String, String> {
    let output = Command::new("git")
        .args(["reflog"])
        .output()
        .map_err(|e| format!("failed to execute git reflog: {}", e))?;

    if !output.status.success() {
        return Err("failed to execute git reflog".to_string());
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("failed to parse output: {}", e))
}

pub fn checkout_branch(branch: &str) -> Result<(), String> {
    let status = Command::new("git")
        .args(["checkout", branch])
        .status()
        .map_err(|e| format!("failed to execute git checkout: {}", e))?;

    if !status.success() {
        return Err(format!("failed to switch to branch '{}'", branch));
    }

    Ok(())
}
