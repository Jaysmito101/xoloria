pub fn setup_riscv_tools() -> anyhow::Result<()> {
    let config_dir = directories::ProjectDirs::from("com", "xoloria", "xoloria")
        .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
        .config_dir()
        .to_path_buf();

    if config_dir.join("Tools/riscv-toolchain").exists() {
        unsafe {
            std::env::set_var(
                "RISCV_TOOLCHAIN_PATH",
                config_dir.join("Tools/riscv-toolchain"),
            );
        }
    }

    if let Ok(riscv_toolchain_path) = std::env::var("RISCV_TOOLCHAIN_PATH") {
        if std::path::Path::new(&riscv_toolchain_path).exists() {
            return Ok(());
        } else {
            anyhow::bail!(
                "RISCV_TOOLCHAIN_PATH is set to {}, but the path does not exist",
                riscv_toolchain_path
            );
        }
    }

    anyhow::bail!(
        "RISCV_TOOLCHAIN_PATH is not set and the default path does not exist, please set it up manually or run cargo xtask setup-riscv-tools to set it up automatically"
    )
}
