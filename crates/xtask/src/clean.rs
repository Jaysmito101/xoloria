pub fn clean() -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("clean").arg("--workspace");
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("cargo clean failed with status: {}", status);
    }
    Ok(())
}
