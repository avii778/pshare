mod cli;
mod networks;

use cli::{CLI, Cmd};
use clap::Parser;
use crate::networks::Connection;

#[tokio::main]
async fn main() {

    let args = CLI::parse();

    match args.cmd {

        Cmd::Receive {bind, out_dir} => {

            let networks = Connection::new_connection(bind).await;

            match networks.unwrap() {
                Err(E) => {println!("Error communicating with address")}

                Ok(connection) => {
                    let read = connection.read_file(out_dir).await;

                    match read.unwrap() {

                        Err(E) => {println!("Error reading from stream")}

                        Ok(T) => {println!("Recieved file, stored at: {}", out_dir.as_os_str())}
                    }
                }
            }

        }

        Cmd::Send {bind, file} => {

            let networks = Connection::new_connection(bind).await;

            match networks.unwrap() {
                Err(E) => {println!("Error communicating with address")}

                Ok(connection) => {

                    let write = connection.write_file(file);

                    match write.unwrap() {

                        Err(E) => {println!("Error writing to stream")}

                        Ok(T) => {println!("Recieved file, stored at: {}", file.as_os_str())}
                    }

                }
            }
        }

    }
}
