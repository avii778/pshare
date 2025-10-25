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
    pub async fn send_file<P: AsRef<Path>>(&self, file: P) -> io::Result<()>{
        let (mut socket, _) = self.listener.accept().await?;

        let mut file = OpenOptions::new()
            .read(true)
            .create(true)// send an empty file if needed
            .truncate(false)  // DO NOT WIPE CONTENTS. DO NOT CHANGE
            .open(file)
            .await?;


        tokio::io::copy(&mut file, &mut socket).await?;
        Ok(())
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

