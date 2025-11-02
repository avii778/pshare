use std::alloc::System;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::OpenOptions;

use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;

pub struct Connection {
    bind: String, // should be some ip
    listener: TcpListener,
}

impl Connection {

    pub async fn new_connection(bind: String) -> Result<Connection, io::Error>{

        let listener = TcpListener::bind(&bind).await?;

        // If bind succeeded, return Ok(Connection)
        Ok(Connection {
            bind,
            listener,
        })

    }
    pub async fn read_file<P: AsRef<Path>>(&self, path_to_write: P) -> io::Result<()>{

        let (mut socket, _) = self.listener.accept().await?;

        let mut file =
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path_to_write)
                .await?;

        tokio::io::copy(&mut socket, &mut file).await?;

        file.flush().await?;

        Ok(())

    }
}


pub struct Postman {
    send_to: String,
    connection: TcpStream
}

impl Postman{

    pub async fn new_postman(send_to: String) -> Result<Postman, io::Error> {

        let mut stream = TcpStream::connect(send_to).await?;

        Ok(
            Postman {send_to: send_to, connection: stream}
        )
    }

    pub async fn send_file<P: AsRef<Path>>(&mut self, p: P) -> io::Result<()>{

        let mut file = OpenOptions::new()
            .read(true)
            .truncate(false)
            .open(p).await?;

        tokio::io::copy(&mut file, &mut self.connection).await?;
        self.connection.shutdown().await?;
        Ok(())
    }
}
