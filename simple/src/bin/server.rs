use anyhow::{anyhow, Error, Result};
use simple::{
    connection::{Command, Connection, R},
    ds::NodeLL,
};
use std::{fmt::format, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

type Db = Arc<Mutex<NodeLL>>;
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7274").await?;
    let ll = Arc::new(Mutex::new(NodeLL::new(7274)));

    loop {
        let (socket, _) = listener.accept().await?;
        let ll = Arc::clone(&ll);
        let mut connection = Connection::new(socket);

        tokio::spawn(async move {
            let cmd = connection.read_request().await;
            let res: R = match process(cmd, ll).await {
                Ok(Some(node)) => {
                    //search, transmit response back to client
                    R::Node(node.value.to_string())
                }
                Ok(None) => {
                    //insert
                    R::Ok
                }
                Err(e) => {
                    //eprintln!("{e}");
                    R::Err(e.to_string())
                }
            };

            if let Ok(res) = serde_json::to_string(&res) {
                // Send a basic HTTP response back
                println!("this its sriew  {:?}", res);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    res.len(),
                    res.to_string());
                connection.stream.write_all(response.as_bytes()).await;
            }

            //let s = connection.send_frame(&res).await;
            //println!("{:?}", s);
        });
    }
}

async fn process(cmd: Result<Command>, ll: Db) -> Result<Option<NodeLL>> {
    match cmd {
        Ok(Command::INSERT(val)) => {
            ll.lock().await.insert(val).await;
            ll.lock().await.traverse().await;
            return Ok(None);
        }
        Ok(Command::SEARCH(val)) => {
            ll.lock().await.traverse().await;
            return Ok(ll.lock().await.search(val).await);
        }
        Err(e) => {
            return Err(e);
        }
    }
    //Err(anyhow!("now summet boog  /z 'appen"))
}
