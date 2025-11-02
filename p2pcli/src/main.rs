mod cli;
mod networks;

use cli::{CLI, Cmd};
use clap::Parser;
use crate::networks::{Connection, Postman};

#[tokio::main]
async fn main() {
    let args = CLI::parse();

    match args.cmd {

        Cmd::Receive { bind, out_file } => {

            match Connection::new_connection(bind).await {

                Err(e) => {
                    println!("Error communicating with address: {:?}", e);
                }

                Ok(connection) => {

                    match connection.read_file(out_file.clone()).await {
                        Err(e) => println!("Error reading from stream: {:?}", e),

                        Ok(_) => println!("Received file, stored at: {}", out_file.display()),

                    }
                }
            }
        }

        Cmd::Send { send_to, file } => {

            match Postman::new_postman(send_to).await {

                Err(e) => {
                    println!("Error communicating with address: {:?}", e);
                }

                Ok(mut postman) => {
                    match postman.send_file(file).await {
                        Err(e) => println!("Error writing to stream: {:?}", e),
                        Ok(_) => println!("Sent file successfully"),
                    }
                }

            }
        }
    }
}
