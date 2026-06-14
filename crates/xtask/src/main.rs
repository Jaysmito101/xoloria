use clap::Parser;

mod build;
mod clean;
mod lints;
mod opts;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = opts::Opts::try_parse()?;

    match opts.command {
        opts::Command::Check => lints::run_check()?,
        opts::Command::Clippy => lints::run_clippy()?,
        opts::Command::Clean => clean::clean()?,
        opts::Command::BuildFirmware => build::build_firmware()?,
        opts::Command::DumpFirmware => build::dump_firmware()?,
        opts::Command::BuildOs { debug } => {
            let _ = debug;
            unimplemented!("BuildOs command is not implemented yet")
        }
        opts::Command::BuildAll { debug } => {
            let _ = debug;
            unimplemented!("BuildAll command is not implemented yet")
        }
        opts::Command::Cli { debug, args } => build::run_cli(debug, args)?,
    };

    Ok(())
}
