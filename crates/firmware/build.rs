fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=linker.ld");
    let out_dir = std::env::var("OUT_DIR")?;
    std::fs::copy("linker.ld", format!("{}/linker.ld", out_dir))?;

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rustc-link-arg=-nostdlib");
    println!("cargo:rustc-link-arg=--no-relax");

    Ok(())
}
