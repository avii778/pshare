use std::alloc::System;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs;

use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

pub struct Connection {
    bind: String, // should be some ip
    listener: TcpListener,
}

impl Connection {

    pub async fn network (&mut self, bind: String) -> io::Result<()>{
        self.bind = bind;
        self.listener = TcpListener::bind(&self.bind).await?;

        Ok(())
    }
    pub async fn send_file(&self, file: String) -> io::Result<()>{
        let contents = tokio::fs::read_to_string(file).await?;

        let (mut socket, _) = self.listener.accept().await?;

        socket.write_all(contents.as_ref()).await?;
        Ok(())
    }

    pub async fn read_file(&self, path_to_write: String) -> io::Result<()>{

        let (mut socket, _) = self.listener.accept().await?;

        let path = Path::new(&path_to_write);

        let mut file = OpenOptions::new().write(true).open(path_to_write);
        let mut buffer = Vec::new();
        socket.read(&mut buffer).await.expect("Error reading from stream ");

        match file?.write(&*buffer).await {

            Err(e) => {println!("Error writing to file")}
        }


        Ok(())

    }
}

