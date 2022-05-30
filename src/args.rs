mod tuber;
mod meta;

use std::borrow::Borrow;
use argh::FromArgs;

pub trait Runnable {
    fn run(&self) -> crate::error::Result<()>;
}

/// Potatable: A portable system automation tool
#[derive(FromArgs, Debug)]
pub struct Args {
    #[argh(subcommand)]
    pub subcommands: Subcommands
}

impl Runnable for Args {
    fn run(&self) -> crate::error::Result<()> {
        match &self.subcommands {
            Subcommands::Tuber(x) => x.run(),
            Subcommands::Meta(x) => x.run()
        }
    }
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Subcommands {
    Tuber(tuber::Tuber),
    Meta(meta::Meta)
}
