mod cli;
mod networks;

use cli::{CLI, Cmd};
use clap::Parser;

fn main() {

    let args = CLI::parse();

    match args.cmd {

        Cmd::Receive {bind, out_dir} => {

        }

        Cmd::Send {bind, file} => {

        }

    }
}
