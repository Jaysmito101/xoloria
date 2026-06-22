fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    panic!(
        "This crate is not meant to be built directly. Please use `cargo xtask` to run the archtests."
    );

    Ok(())
}
