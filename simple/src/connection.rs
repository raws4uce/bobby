use anyhow::{anyhow, Error, Ok, Result};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

//request which gets deserialised
#[derive(Deserialize, Serialize, Debug)]
pub enum Command {
    INSERT(u64),
    SEARCH(u64),
    //probably could add traverse  
}

//response we serialise, enroute to client
#[derive(Deserialize, Serialize, Debug)]
pub enum R {
    Node(String),
    Ok,
    Err(String),
}

pub struct Connection {
    pub stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub async fn read_request(&mut self) -> Result<Command> {
        let mut reader = BufReader::new(&mut self.stream);
        let mut headers = String::new();

        //loop
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).await?;
            if line == "\r\n" {
                break;
            }
            headers.push_str(&line);
        }

        let content_length = headers
            .lines()
            .find_map(|line| {
                if line.to_lowercase().starts_with("content-length") {
                    line.split(":").nth(1)?.trim().parse().ok()
                } else {
                    None
                }
            })
            .expect("content-length header required");

        let mut json_buf = vec![0; content_length];
        reader.read_exact(&mut json_buf).await?;

        let command: Command = serde_json::from_slice(&json_buf).expect("failed to parse json");

        //// Send a basic HTTP response back
        //let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
        //self.stream.write_all(response.as_bytes()).await?;

        Ok(command)
    }

    //pub async fn read_frame(&mut self) -> Result<Option<FrameIn>>{
    //    loop {
    //        if let Some(frame) = self.read_command().await?{
    //            return Ok(Some(frame));
    //        }
    //        if 0 == self.stream.read_buf(&mut self.buf).await?{
    //            if self.buf.is_empty(){
    //                return Ok(None);
    //            }else {
    //                return Err(anyhow!(""));
    //            }
    //
    //        }
    //    }
    //}
    //format of data comming in is 3bytes as len, then json
    //async fn read_command(&mut self) -> Result<Option<FrameIn>>{
    //    let mut len_buf = [0u8; 97];
    //    let c = self.stream.read_exact(&mut len_buf).await?;
    //    println!("let me {c}");
    //    let length : usize = String::from_utf8_lossy(&len_buf).parse().expect(format!("{:?}, {:?}",c,len_buf).as_str());
    //
    //    let mut json_buf = vec![0u8; length];
    //    self.stream.read_exact(&mut json_buf).await?;
    //
    //    let frame: FrameIn = serde_json::from_slice(&json_buf)
    //        .expect("");
    //
    //    Ok(Some(frame))
    //}

//POST COMPLETION NOTE : This needs to be looked at again 
    /// sends serialized payload (frame)
    pub async fn send_frame(&mut self, data: &R) -> Result<()> {
        ////serialise data then transmit
        let data = serde_json::to_string(data).unwrap();
        let data_bytes = data.as_bytes();

        let length = data_bytes.len() as u16;
        let length_buf = length.to_be_bytes();

        // Send the length prefix first
        self.stream.write_all(&length_buf).await?;

        // Write the actual data
        self.stream.write_all(data_bytes).await?;
        self.stream.flush().await?;

        Ok(())
    }

    //parse frame

    //have this on server, not here
    //pub async fn process(&mut self) -> Result<()> {
    //    while let Some(frame) = self.read_frame().await.unwrap() {
    //        let response = handle_frame(frame);
    //
    //        self.send_frame(&response).await?;
    //    }
    //    Ok(())
    //}
}
