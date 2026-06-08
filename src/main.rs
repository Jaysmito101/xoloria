use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Opts {
    #[arg(short)]
    binary: String,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    let binary = std::fs::read(opts.binary)?;

    let machine = xoloria::MachineBuilder::new("Xoloria/VM")
        .with_harts(0)?
        .build()?;

    Ok(())
}
