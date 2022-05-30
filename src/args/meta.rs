use argh::FromArgs;
use crate::args::Runnable;

/// metadata options regarding Potatable
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "meta")]
pub struct Meta {
    #[argh(subcommand)]
    subcommands: Options
}

impl Runnable for Meta {
    fn run(&self) -> crate::error::Result<()> {
        match &self.subcommands {
            Options::About(x) => x.run(),
            Options::Repo(x) => x.run()
        }
    }
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Options {
    About(About),
    Repo(Repo)
}

/// information about Potatable
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "about")]
pub struct About {}

impl Runnable for About {
    fn run(&self) -> crate::error::Result<()> {
        Ok(())
    }
}

/// the repo where Potatable is stored
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "repo")]
pub struct Repo {}

impl Runnable for Repo {
    fn run(&self) -> crate::error::Result<()> {
        println!("asdf");
        Ok(())
    }
}
