mod build;
mod clean;
mod lints;
mod opts;
mod tools;

pub use opts::Opts;
pub use tools::*;

pub fn run_command(command: &opts::Command) -> anyhow::Result<()> {
    match command {
        opts::Command::Check => lints::run_check()?,
        opts::Command::Clippy => lints::run_clippy()?,
        opts::Command::Clean => clean::clean()?,
        opts::Command::BuildFirmware { debug } => build::build_firmware(!debug)?,
        opts::Command::DumpFirmware => build::dump_firmware()?,
        opts::Command::RunArchTests => build::run_arch_tests()?,
        opts::Command::SetupRiscvTools => tools::setup_riscv_tools()?,
        opts::Command::BuildOs { debug } => {
            let _ = debug;
            unimplemented!("BuildOs command is not implemented yet")
        }
        opts::Command::BuildAll { debug } => {
            let _ = debug;
            unimplemented!("BuildAll command is not implemented yet")
        }
        opts::Command::Cli { debug, args } => build::run_cli(*debug, args)?,
        opts::Command::Debugger { debug, args } => build::run_debugger(*debug, args)?,
    };

    Ok(())
}
