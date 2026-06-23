use std::hash::Hasher;

fn get_config_hash() -> anyhow::Result<String> {
    let config_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("config");
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for entry in std::fs::read_dir(config_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let content = std::fs::read(&path)?;
            use std::hash::Hasher;
            hasher.write(&content);
        }
    }
    let hash = hasher.finish();
    Ok(format!("{:x}", hash))
}

fn get_arch_tests_dir() -> anyhow::Result<std::path::PathBuf> {
    let target_dir = std::env::var("OUT_DIR")?.to_string();
    let target_dir = std::path::Path::new(&target_dir)
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory of OUT_DIR"))?;
    let hash = get_config_hash().expect("Failed to compute config hash");
    Ok(target_dir.join(format!("riscv-arch-tests-{}", hash)))
}

fn build_arch_tests(base_dir: std::path::PathBuf) -> anyhow::Result<()> {
    if !xtask::is_git_repo(&base_dir.join("sources")) {
        tracing::info!("Cloning RISC-V architecture tests repository...");
        xtask::clone_repo(
            "https://github.com/riscv/riscv-arch-test.git",
            true,
            &base_dir.join("sources"),
        )?;
    } else {
        tracing::info!(
            "RISC-V architecture tests repository found at {}, skipping clone.",
            base_dir.join("sources").display()
        );
    }

    xtask::ensure_tool_installed("mise")?;

    tracing::info!("Trusting the mise configuration...");
    let command = std::process::Command::new("mise")
        .current_dir(base_dir.join("sources"))
        .arg("trust")
        .arg(".mise.toml")
        .status()?;
    if !command.success() {
        anyhow::bail!("Failed to trust the mise configuration.");
    }

    tracing::info!("Building RISC-V architecture tests...");
    let config_file = base_dir
        .join("sources")
        .join("config")
        .join("cores")
        .join("cvw")
        .join("cvw-rv64gc")
        .join("test_config.yaml");
    tracing::info!("Using config file: {}", config_file.display());
    let num_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let command = std::process::Command::new("make")
        .arg("--jobs")
        .arg(num_cores.to_string())
        .env("CONFIG_FILES", config_file)
        .current_dir(base_dir.join("sources"))
        .status()?;
    if !command.success() {
        anyhow::bail!("Build failed with status: {}", command);
    }
    panic!(
        "RISC-V architecture tests build completed. Please run the tests using the generated binaries in the 'bin' directory."
    );
    Ok(())
}

fn rerun_on_config_change() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=config");
    for entry in std::fs::read_dir("config")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if cfg!(target_os = "windows") {
        anyhow::bail!(
            "RISC-V architecture tests are not supported on Windows (atleast not without selling my soul)."
        );
    }

    tracing::info!("Preparing to setup RISC-V architecture tests...");
    let test_dir = get_arch_tests_dir()?;
    if !test_dir.join("bin").exists() {
        tracing::info!(
            "RISC-V architecture tests not found, building at: {}",
            test_dir.display()
        );
        build_arch_tests(test_dir.clone())?;
    }

    tracing::info!(
        "RISC-V architecture tests are ready at: {}",
        test_dir.display()
    );

    rerun_on_config_change()?;

    Ok(())
}
