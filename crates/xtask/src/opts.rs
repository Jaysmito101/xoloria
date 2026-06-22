#[derive(clap::Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
    Check,
    Clippy,
    Clean,
    RunArchTests,
    BuildFirmware {
        #[clap(short, long, default_value_t = false)]
        debug: bool,
    },
    DumpFirmware,
    BuildOs {
        #[clap(short, long, default_value_t = false)]
        debug: bool,
    },
    BuildAll {
        #[clap(short, long, default_value_t = false)]
        debug: bool,
    },
    Cli {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
        #[clap(short, long, default_value_t = false)]
        debug: bool,
    },
    Debugger {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
        #[clap(short, long, default_value_t = false)]
        debug: bool,
    },
}
