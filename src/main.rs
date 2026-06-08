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

    tracing::warn!("Opts: {:?}", opts);

    Ok(())
}
