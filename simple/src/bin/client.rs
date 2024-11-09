use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple::{
    connection::{Command, R},
    ds::NodeLL,
};
use tokio::{
    self,
    sync::{mpsc, oneshot},
};
#[derive(Debug)]
enum CMD {
    Insert {
        json: Command,
        rx: oneshot::Sender<R>,
    },
    Search {
        json: Command,
        rx: oneshot::Sender<R>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<CMD>(32);
    let manager = tokio::spawn(async move {
        let client = Client::new();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                CMD::Insert { json, rx } => {
                    //serde
                    let response = client
                        .post("http://127.0.0.1:7274/")
                        .json(&json)
                        .send()
                        .await
                        .expect("Failed to send request");

                    let byt = response
                        .bytes()
                        .await
                        .expect("failed to read response bytes");
                    match serde_json::from_slice::<R>(&byt) {
                        Ok(res) => {
                            println!("{:?}", res);
                        }
                        Err(e) => {
                            eprint!("{:?}", e);
                        }
                    }
                }
                CMD::Search { json, rx } => {
                    let response = client
                        .post("http://127.0.0.1:7274/")
                        .json(&json)
                        .send()
                        .await
                        .expect("fail to send req");

                    let byt = response
                        .bytes()
                        .await
                        .expect("failed to read response bytes");
                    match serde_json::from_slice::<R>(&byt) {
                        Ok(res) => {
                            println!("{res:?}");
                        }
                        Err(e) => {
                            eprint!("{e:?}")
                        }
                    }
                }
            }
        }
    });

    let tx2 = tx.clone();
    //
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let json = Command::INSERT(34);
        let cmd = CMD::Insert {
            json: json,
            rx: resp_tx,
        };

        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
    });
    //
    //
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let json = Command::SEARCH(35);
        let cmd = CMD::Search {
            json: json,
            rx: resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("resp = {res:?}");
    });
    //
    //
    let x = t1.await;
    let y = t2.await;
    let c = manager.await;
}
