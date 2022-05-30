use argh::FromArgs;
use crate::args::Runnable;

/// run a tuber
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "tuber")]
pub struct Tuber {
    /// name of the tuber to run
    #[argh(positional)]
    name: String,

    /// use verbose logging during execution
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// run the tuber but make no changes
    #[argh(switch, short = 'd')]
    dry_run: bool
}

impl Runnable for Tuber {
    fn run(&self) -> crate::error::Result<()> {
        Ok(())
    }
}
