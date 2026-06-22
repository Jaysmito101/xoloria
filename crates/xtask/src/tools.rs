pub fn ensure_riscv_tools() -> anyhow::Result<()> {
    let toolchain_dir = std::path::PathBuf::from("tools/riscv-toolchain");

    if toolchain_dir.exists() {
        unsafe {
            std::env::set_var("RISCV_TOOLCHAIN_PATH", toolchain_dir);
        }
    }

    if let Ok(riscv_toolchain_path) = std::env::var("RISCV_TOOLCHAIN_PATH") {
        if std::path::Path::new(&riscv_toolchain_path).exists() {
            ensure_tool_installed(format!("{}/bin/clang", riscv_toolchain_path).as_str())?;
            ensure_tool_installed(format!("{}/bin/ld", riscv_toolchain_path).as_str())?;
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

pub fn setup_riscv_tools() -> anyhow::Result<()> {
    ensure_tool_installed("cmake")?;
    ensure_tool_installed("ninja")?;
    ensure_tool_installed("clang")?;
    clone_repo(
        "https://github.com/llvm/llvm-project.git",
        true,
        std::path::Path::new("tools/llvm-project"),
    )?;

    tracing::info!("Building RISC-V toolchain...");
    let llvm_source_dir = std::path::Path::new("tools/llvm-project/llvm");
    let llvm_build_dir = std::path::Path::new("tools/llvm-project/build");
    let cmake_cmd = std::process::Command::new("cmake")
        .arg("-S")
        .arg(llvm_source_dir)
        .arg("-B")
        .arg(llvm_build_dir)
        .arg("-G")
        .arg("Ninja")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DLLVM_ENABLE_PROJECTS=clang;lld")
        .arg("-DLLVM_TARGETS_TO_BUILD=RISCV")
        .status()?;
    if !cmake_cmd.success() {
        anyhow::bail!("CMake configuration failed with status: {}", cmake_cmd);
    }

    if !llvm_build_dir.exists() {
        std::fs::create_dir_all(llvm_build_dir)?;
    }

    tracing::info!("Checking if RISC-V toolchain is properly built...");
    ensure_tool_installed("tools/llvm-project/build/bin/clang")?;
    ensure_tool_installed("tools/llvm-project/build/bin/ld")?;

    tracing::info!("Copying RISC-V toolchain to tools/riscv-toolchain...");
    let build_bin_dir = std::path::Path::new("tools/llvm-project/build/bin");
    let riscv_toolchain_dir = std::path::Path::new("tools/riscv-toolchain/bin");
    if riscv_toolchain_dir.exists() {
        std::fs::remove_dir_all(riscv_toolchain_dir)?;
    }
    std::fs::create_dir_all(riscv_toolchain_dir)?;
    for entry in std::fs::read_dir(build_bin_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            std::fs::copy(&path, riscv_toolchain_dir.join(file_name))?;
        }
    }

    std::fs::remove_dir_all("tools/llvm-project")?;

    Ok(())
}

fn ensure_tool_installed(tool: &str) -> anyhow::Result<()> {
    tracing::info!("Checking if {} is installed...", tool);
    let status = std::process::Command::new(tool).arg("--version").status()?;

    if !status.success() {
        anyhow::bail!("{} is not installed or not found in PATH", tool);
    }

    tracing::info!("Found {} installed.", tool);
    Ok(())
}

fn clone_repo(repo_url: &str, shallow: bool, dest_dir: &std::path::Path) -> anyhow::Result<()> {
    ensure_tool_installed("git")?;

    if dest_dir.exists() {
        anyhow::bail!(
            "Destination directory {} already exists",
            dest_dir.display()
        );
    }

    tracing::info!(
        "Cloning repository {} into {}...",
        repo_url,
        dest_dir.display()
    );

    let status = std::process::Command::new("git")
        .arg("clone")
        .arg(if shallow { "--depth" } else { "" })
        .arg(if shallow { "1" } else { "" })
        .arg(repo_url)
        .arg(dest_dir)
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to clone repository: {}", repo_url);
    }

    Ok(())
}
