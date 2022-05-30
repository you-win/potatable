mod args;
mod error;

use crate::args::{Args, Runnable, Subcommands};

fn main() {
    let args: Args = argh::from_env();

    println!("{:?}", args);

    // match args.subcommands {
    //     Subcommands::Tuber(arg) => {}
    //     Subcommands::Meta(arg) => {}
    // }

    match args.run() {
        Ok(_) => {}
        Err(_) => {}
    }
}
