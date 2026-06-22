use clap::Parser;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = xtask::Opts::try_parse()?;
    xtask::run_command(&opts.command)?;
    Ok(())
}
