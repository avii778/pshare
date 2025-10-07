use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "p2pcli")]
#[command(about = "Peer-to-peer file sharing CLI tool")]
pub struct CLI {
    #[command(subcommand)]
    pub cmd: Cmd
}

#[derive(Subcommand, Debug)]
pub enum Cmd {

    Receive {
    #[arg(long, default_value = "0.0.0.0:9000")]
    bind: String,

    #[arg(long, default_value = ".")]
    out_dir: std::path::PathBuf,
    },

    Send {

        #[arg(long, default_value = "0.0.0.0:9000")]
        bind: String,

        #[arg(long)]
        file: std::path::PathBuf
    }
}



