pub fn run_check() -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("check").arg("--workspace");
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("cargo check failed with status: {}", status);
    }
    Ok(())
}

pub fn run_clippy() -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("clippy")
        .arg("--workspace")
        .arg("--")
        .arg("-D")
        .arg("warnings");
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("cargo clippy failed with status: {}", status);
    }
    Ok(())
}
