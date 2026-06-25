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
    Ok(target_dir.join("riscv-arch-tests-latest"))
}

fn create_arch_tests_bins(base_dir: std::path::PathBuf) -> anyhow::Result<()> {
    let mut registry = serde_json::Map::new();
    for entry in walkdir::WalkDir::new(base_dir.join("xoloria-rva23S64").join("bin"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "elf")
                .unwrap_or(false)
        })
    {
        let elf_path = entry.path();
        let bin_path = elf_path.with_extension("bin");
        tracing::info!(
            "Creating bin file {} from elf file {}",
            bin_path.display(),
            elf_path.display()
        );
        let status = std::process::Command::new("llvm-objcopy")
            .arg("-O")
            .arg("binary")
            .arg(elf_path)
            .arg(&bin_path)
            .status()?;
        if !status.success() {
            anyhow::bail!(
                "Failed to create bin file {} from elf file {}",
                bin_path.display(),
                elf_path.display()
            );
        }
        let elf_hash = {
            let content = std::fs::read(elf_path)?;
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            hasher.write(&content);
            format!("{:x}", hasher.finish())
        };
        let bin_hash = {
            let content = std::fs::read(&bin_path)?;
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            hasher.write(&content);
            format!("{:x}", hasher.finish())
        };
        registry.insert(
            elf_path.to_string_lossy().to_string(),
            serde_json::json!({
                "bin": bin_path.to_string_lossy().to_string(),
                "elf": elf_path.to_string_lossy().to_string(),
                "elf_hash": elf_hash,
                "bin_hash": bin_hash,
            }),
        );
    }
    let registry_file = base_dir.join("registry.json");
    std::fs::write(
        &registry_file,
        serde_json::to_string_pretty(&serde_json::json!({
            "config_hash": get_config_hash()?,
            "registry": registry,
        }))
        .unwrap_or_else(|_| "{}".into()),
    )?;
    Ok(())
}

fn build_arch_tests(base_dir: &std::path::Path) -> anyhow::Result<()> {
    if cfg!(target_os = "windows") {
        anyhow::bail!("Building RISC-V architecture tests are not supported on Windows yet.");
    }

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

    xtask::add_to_path_env_var(
        std::path::Path::new(&std::env::var("SAIL_RISCV_PATH")?)
            .join("bin")
            .as_path(),
    )?;

    xtask::add_to_path_env_var(
        std::path::Path::new(&std::env::var("RISCV_TOOLCHAIN_PATH")?)
            .join("bin")
            .as_path(),
    )?;

    tracing::info!("Building RISC-V architecture tests...");
    let config_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("config")
        .join("test_config.yaml");
    tracing::info!("Using config file: {}", config_file.display());
    let num_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let command = std::process::Command::new("make")
        .arg("--jobs")
        .arg(num_cores.to_string())
        .env("CONFIG_FILES", config_file)
        .env("MISE_VERBOSE", "1")
        .current_dir(base_dir.join("sources"))
        .status()?;
    if !command.success() {
        anyhow::bail!("Build failed with status: {}", command);
    }

    let build_dir = base_dir
        .join("sources")
        .join("work")
        .join("xoloria-rva23S64");
    // move it to the base_dir
    let target_dir = base_dir.join("bin");
    if target_dir.exists() {
        std::fs::remove_dir_all(&target_dir)?;
    }
    std::fs::create_dir_all(&target_dir)?;
    xtask::recursively_move_contents(&build_dir, &target_dir)?;
    create_arch_tests_bins(base_dir.to_path_buf())?;
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

fn require_tests_build() -> anyhow::Result<bool> {
    let registry = get_arch_tests_dir()?.join("registry.json");
    let config_hash = get_config_hash()?;
    if !registry.exists() {
        return Ok(true);
    }
    let registry_content = std::fs::read_to_string(&registry)?;
    let registry_json: serde_json::Value = serde_json::from_str(&registry_content)?;
    if let Some(registry_config_hash) = registry_json.get("config_hash")
        && registry_config_hash != &serde_json::Value::String(config_hash)
    {
        return Ok(true);
    }
    Ok(false)
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Preparing to setup RISC-V architecture tests...");

    let test_dir = get_arch_tests_dir()?;
    if require_tests_build()? {
        tracing::info!(
            "RISC-V architecture tests not found/outdated, building at: {}",
            test_dir.display()
        );
        build_arch_tests(&test_dir)?;
    } else {
        tracing::info!(
            "RISC-V architecture tests are up to date at: {}",
            test_dir.display()
        );
    }

    tracing::info!(
        "RISC-V architecture tests are ready at: {}",
        test_dir.display()
    );

    rerun_on_config_change()?;

    Ok(())
}
