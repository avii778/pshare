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

    #[arg(long, value_name = "OUT_FILE")]
    out_file: std::path::PathBuf,
    },

    Send {

        #[arg(long)]
        send_to: String,

        #[arg(long)]
        file: std::path::PathBuf
    }
}



