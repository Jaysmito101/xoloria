pub fn ensure_riscv_tools() -> anyhow::Result<()> {
    let toolchain_dir =
        std::path::PathBuf::from(format!("tools/riscv-toolchain/{}", std::env::consts::OS));
    if toolchain_dir.exists() {
        ensure_tool_installed(format!("{}/bin/clang", toolchain_dir.display()).as_str())?;
        unsafe {
            std::env::set_var("RISCV_TOOLCHAIN_PATH", &toolchain_dir);
        }
    } else {
        anyhow::bail!(
            "riscv llvm toolchain not found, please run `cargo xtask setup-riscv-tools` to set it up automatically"
        );
    }

    if cfg!(not(target_os = "windows")) {
        let sail_dir =
            std::path::PathBuf::from(format!("tools/sail-riscv/{}", std::env::consts::OS));
        if sail_dir.exists() {
            ensure_tool_installed(format!("{}/bin/sail_riscv_sim", sail_dir.display()).as_str())?;
            unsafe {
                std::env::set_var("SAIL_RISCV_PATH", &sail_dir);
            }
        } else {
            anyhow::bail!(
                "riscv sail not found, please run `cargo xtask setup-riscv-tools` to set it up automatically"
            );
        }
    }

    Ok(())
}

fn setup_riscv_llvm_toolchain() -> anyhow::Result<()> {
    ensure_tool_installed("cmake")?;
    ensure_tool_installed("ninja")?;
    ensure_tool_installed("clang")?;
    if !is_git_repo(std::path::Path::new("tools/llvm-project")) {
        clone_repo(
            "https://github.com/llvm/llvm-project.git",
            true,
            std::path::Path::new("tools/llvm-project"),
        )?;
    }

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

fn setup_riscv_sail(version: &str) -> anyhow::Result<()> {
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        "arm" => "arm64",
        _ => anyhow::bail!("Unsupported architecture: {}", std::env::consts::ARCH),
    };
    let os = match std::env::consts::OS {
        "macos" => "Mac",
        "linux" => "Linux",
        "windows" => "Windows",
        _ => anyhow::bail!("Unsupported operating system: {}", std::env::consts::OS),
    };
    let url = format!(
        "https://github.com/riscv/sail-riscv/releases/download/{}/sail-riscv-{}-{}.tar.gz",
        version, os, arch
    );

    let sail_riscv = download_to_temp(&url)?;

    let sail_riscv_dir =
        std::path::PathBuf::from(format!("tools/sail-riscv/{}", std::env::consts::OS));
    if !sail_riscv_dir.exists() {
        std::fs::create_dir_all(&sail_riscv_dir)?;
    }
    unzip_tar_gz(&sail_riscv, &sail_riscv_dir)?;
    std::fs::remove_file(&sail_riscv)?;

    // it unsizps to sail_riscv_dir/some_dir/stuff
    // move the contents of some_dir to sail_riscv_dir
    let extracted_dir = std::fs::read_dir(&sail_riscv_dir)?
        .next()
        .ok_or_else(|| anyhow::anyhow!("Failed to find extracted directory"))??;
    recursively_move_contents(&extracted_dir.path(), &sail_riscv_dir)?;
    std::fs::remove_dir_all(extracted_dir.path())?;

    Ok(())
}

pub fn setup_riscv_tools() -> anyhow::Result<()> {
    let clang_path = std::path::Path::new("tools/riscv-toolchain")
        .join(std::env::consts::OS)
        .join("bin")
        .join("clang");
    if !clang_path.exists() {
        setup_riscv_llvm_toolchain()?;
    } else {
        tracing::info!("RISC-V LLVM toolchain already setup, skipping...");
    }

    let sail_path = std::path::Path::new("tools/sail-riscv")
        .join(std::env::consts::OS)
        .join("bin")
        .join("sail_riscv_sim");
    if !sail_path.exists() {
        if cfg!(target_os = "windows") {
            tracing::warn!("RISC-V sial isnt supported on windows");
        } else {
            setup_riscv_sail("0.12")?;
        }
    } else {
        tracing::info!("RISC-V sail already setup, skipping...");
    }
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

pub fn download_to_temp(url: &str) -> anyhow::Result<std::path::PathBuf> {
    tracing::info!("Downloading {} to temporary directory...", url);
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("sail-riscv.tar.gz");
    let response = reqwest::blocking::get(url)?;
    let body = response.bytes()?;
    std::fs::write(&file_path, body)?;
    Ok(file_path)
}

pub fn unzip_tar_gz(
    path: &std::path::PathBuf,
    dest_dir: &std::path::PathBuf,
) -> anyhow::Result<()> {
    tracing::info!("Unzipping {} to {}...", path.display(), dest_dir.display());
    let file = std::fs::File::open(path)?;
    let tar = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(dest_dir)?;
    Ok(())
}

pub fn recursively_move_contents(
    src_dir: &std::path::Path,
    dest_dir: &std::path::Path,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let dest_path = dest_dir.join(file_name);
        if path.is_dir() {
            std::fs::create_dir_all(&dest_path)?;
            recursively_move_contents(&path, &dest_path)?;
            std::fs::remove_dir_all(&path)?;
        } else {
            std::fs::rename(&path, &dest_path)?;
        }
    }
    Ok(())
}
