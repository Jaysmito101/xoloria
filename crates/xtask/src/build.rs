pub fn run_cli(debug: bool) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("run").arg("--package").arg("cli");
    if debug {
        cmd.arg("--debug");
    } else {
        cmd.arg("--release");
    }
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("cargo run failed with status: {}", status);
    }
    Ok(())
}
