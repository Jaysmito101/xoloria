pub fn ensure_riscv_tools() -> anyhow::Result<()> {
    let toolchain_dir =
        std::path::PathBuf::from(format!("tools/riscv-toolchain/{}", std::env::consts::OS));

    if toolchain_dir.exists() {
        unsafe {
            std::env::set_var("RISCV_TOOLCHAIN_PATH", toolchain_dir);
        }
    }

    if let Ok(riscv_toolchain_path) = std::env::var("RISCV_TOOLCHAIN_PATH") {
        if std::path::Path::new(&riscv_toolchain_path).exists() {
            ensure_tool_installed(format!("{}/bin/clang", riscv_toolchain_path).as_str())?;
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

    tracing::info!("Preparing to build the LLVM RISC-V toolchain...");
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

    tracing::info!("Building the LLVM RISC-V toolchain...");
    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let build_cmd = std::process::Command::new("cmake")
        .arg("--build")
        .arg(llvm_build_dir)
        .arg("-j")
        .arg(num_cpus.to_string())
        .status()?;
    if !build_cmd.success() {
        anyhow::bail!("Build failed with status: {}", build_cmd);
    }

    tracing::info!("Checking if RISC-V toolchain is properly built...");
    ensure_tool_installed("tools/llvm-project/build/bin/clang")?;

    tracing::info!("Copying RISC-V toolchain to tools/riscv-toolchain...");
    let build_bin_dir = std::path::Path::new("tools/llvm-project/build/bin");
    let riscv_toolchain_dir = format!("tools/riscv-toolchain/{}/bin", std::env::consts::OS);
    let riscv_toolchain_dir = std::path::Path::new(&riscv_toolchain_dir);
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

pub fn ensure_tool_installed(tool: &str) -> anyhow::Result<()> {
    match std::process::Command::new(tool)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg("--version")
        .status()
    {
        Ok(status) if status.success() => {
            tracing::info!("Found {} installed.", tool);
            Ok(())
        }
        Ok(_) => anyhow::bail!("{} is not installed or not found in PATH", tool),
        Err(_) => anyhow::bail!("{} is not installed or not found in PATH", tool),
    }
}

pub fn clone_repo(repo_url: &str, shallow: bool, dest_dir: &std::path::Path) -> anyhow::Result<()> {
    ensure_tool_installed("git")?;

    if dest_dir.exists() {
        std::fs::remove_dir_all(dest_dir)?;
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

pub fn is_git_repo(path: &std::path::Path) -> bool {
    let git_dir = path.join(".git");
    git_dir.exists() && git_dir.is_dir()
}
