use std::hash::Hasher;

fn get_config_hash() -> anyhow::Result<String> {
    // builf configdir form cargo manifest_dir
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

fn build_arch_tests(base_dir: std::path::PathBuf) -> anyhow::Result<()> {
    xtask::clone_repo(
        "https://github.com/riscv/riscv-arch-test.git",
        true,
        &base_dir.join("sources"),
    )?;

    unimplemented!("Building RISC-V architecture tests is not implemented yet");
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

    tracing::info!("Preparing to setup RISC-V architecture tests...");
    let hash = get_config_hash()?;
    let target_dir = std::env::var("OUT_DIR")
        .map(std::path::PathBuf::from)
        .expect("OUT_DIR environment variable is not set, cannot determine target directory for architecture tests");

    let test_dir = target_dir.join(format!("riscv-arch-tests-{}", hash));

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
