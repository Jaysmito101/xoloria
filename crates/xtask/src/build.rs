use crate::tools::setup_riscv_tools;

pub fn run_cli(debug: bool, args: Vec<String>) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("run").arg("--package").arg("cli");
    if !debug {
        cmd.arg("--release");
    }

    cmd.arg("--");

    for arg in args {
        cmd.arg(arg);
    }
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("cargo run failed with status: {}", status);
    }
    Ok(())
}

pub fn run_debugger(debug: bool, args: Vec<String>) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("run").arg("--package").arg("debugger");
    if !debug {
        cmd.arg("--release");
    }

    cmd.arg("--");

    for arg in args {
        cmd.arg(arg);
    }
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("cargo run failed with status: {}", status);
    }
    Ok(())
}

pub fn build_firmware(release: bool) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build")
        .arg("--package")
        .arg("firmware")
        .arg("--target")
        .arg("riscv64imac-unknown-none-elf");
    if release {
        cmd.arg("--release");
    }

    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("cargo build failed with status: {}", status);
    }

    let target_dir = if release {
        "target/riscv64imac-unknown-none-elf/release"
    } else {
        "target/riscv64imac-unknown-none-elf/debug"
    };

    let mut objcopy_cmd = std::process::Command::new("rust-objcopy");
    objcopy_cmd
        .arg("--strip-all")
        .arg("-O")
        .arg("binary")
        .arg(format!("{}/firmware", target_dir))
        .arg(format!("{}/firmware.bin", target_dir));
    let objcopy_status = objcopy_cmd.status()?;
    if !objcopy_status.success() {
        anyhow::bail!("rust-objcopy failed with status: {}", objcopy_status);
    }

    std::fs::copy(format!("{}/firmware.bin", target_dir), "firmware.bin")?;

    Ok(())
}

pub fn dump_firmware() -> anyhow::Result<()> {
    let firmware_path = "target/riscv64imac-unknown-none-elf/release/firmware";
    let mut cmd = std::process::Command::new("rust-objdump");
    cmd.arg("-d").arg(firmware_path);

    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("rust-objdump failed with status: {}", status);
    }

    Ok(())
}

pub fn run_arch_tests() -> anyhow::Result<()> {
    setup_riscv_tools()?;

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("run")
        .arg("--package")
        .arg("archtests")
        .arg("--release");

    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("cargo run failed with status: {}", status);
    }

    Ok(())
}
