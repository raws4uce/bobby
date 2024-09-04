use core::panic;
use regex::Regex;
use serde::Deserialize;
use serde_json::{json, Value};
// use sled::Db;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    result::Result::Ok,
    str::FromStr,
};
// use std::{collections::HashMap, process::Command};
use crate::slog::Drain;
use crate::threadpools::TP::TP;
use crate::{engines::kv::KvStore, threadpools::ThreadPool};
use anyhow::Result;

use std::sync::Arc;
/*
init ip addy
requests comes in threadpools
deseralise jsons
do request (return a value/update log)



afterthoughts: shutting down gracefully
*/

pub struct KvsServer {
    engine: Engine,
    _log : slog::Logger,
}

impl KvsServer {
    pub fn new(e: Engine) -> Self {
        //self._log
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        KvsServer { engine: e, _log: slog::Logger::root(drain, o!()) }
    }

    pub fn run(self, addr: &String) {
        let arc = Arc::new(self);
        let listener = TcpListener::bind(addr).unwrap();
        let pool = <TP as ThreadPool>::new(4).unwrap();
        for stream in listener.incoming().take(100) {
            match stream {
                Ok(stream) => {
                    let clone: Arc<KvsServer> = Arc::clone(&arc);

                    pool.spawn(move || clone.serve(stream))
                }
                Err(e) => {
                    error!(arc._log, "I THINK TCPSTREAM ISN'T A VALID STREAM")
                }
            }
        }
        println!("shtudown");
    }

    pub fn serve(&self, mut stream: TcpStream) {
        let mut buf_reader = BufReader::new(&stream);
        // let mut request_line = String::new();

        // buf_reader.read_line(&mut request_line).unwrap();
        // println!("Request Line: {}", request_line.trim());

        //headers
        let mut head = String::new();
        let mut len = 0;
        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line).unwrap();
            if line == "\r\n" {
                break; // end of headers
            }
            if line.to_lowercase().trim().starts_with("content-length:") {
                let re = Regex::new(r"\d+").unwrap();
                let cap = re.captures(&line).unwrap();
                len = cap.get(0).unwrap().as_str().parse::<u64>().unwrap();
            }
            head.push_str(&line);
        }
        // println!("headers {}", head);

        let mut body = String::new();
        buf_reader
            .take(len as u64)
            .read_to_string(&mut body)
            .unwrap();

        let json = serde_json::from_str::<Value>(&body).unwrap();
        let de: Jason = serde_json::from_value(json).unwrap();
        let response_body = json!({ "status": "success" });
        let mut response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            response_body.to_string().len(),
            response_body
        );
        info!(self._log, "{de:?}");
        let mut db_kvs: KvStore = KvStore::new();
        // let mut db_sled: SledKvsEngine = SledKvsEngine::new(Db{
        //     x
        //     y
        //     z
        // });

        match self.engine {
            Engine::Kvs => {
                info!(self._log, "KVS");
                match de.cmd.as_str() {
                    "set" => {
                        db_kvs.set(&de.key, &de.value.unwrap());
                        info!(self._log, "value set in log");
                    }
                    "get" => {
                        if let Some(val) = db_kvs.get(&de.key) {
                            info!(self._log, "This value does exist {val:?}");
                            let rb = json!({"value" : val});
                            response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                                rb.to_string().len(),
                                rb
                            );
                        } else {
                            warn!(self._log, "This value does not exist");
                            response = format!(
                                "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {}\r\n\r\n{}",
                                response_body.to_string().len(),
                                response_body
                            )
                        }
                    }
                    "rm" => {
                        db_kvs.rm(&de.key);
                        info!(self._log, "value removed");
                    }
                    _ => {}
                }
            }
            Engine::Sled => {
                // info!(self._log, "SLED");
                // match de.cmd.as_str(){
                //     "set"=>{
                //         db_sled.set(&de.key, &de.value.unwrap());
                //         info!(self._log, "value set in log");
                //     }
                //     "get" =>{
                //         if let Some(val) = db_sled.get(&de.key){
                //             info!(self._log, "This value does exist {val:?}");
                //             let rb = json!({"value" : val});
                //             response = format!(
                //                 "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                //                 rb.to_string().len(),
                //                 rb
                //             );
                //         }else{
                //             warn!(self._log, "This value does not exist");
                //             response = format!(
                //                 "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {}\r\n\r\n{}",
                //                 response_body.to_string().len(),
                //                 response_body
                //             )
                //         }
                //     }
                //     "rm"=>{
                //         db_sled.rm(&de.key);
                //         info!(self._log, "value removed");
                //     }
                //     _=>{}
                // }
            }
        }
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

#[derive(Deserialize, Debug)]
struct Jason {
    cmd: String,
    key: String,
    value: Option<String>,
}

impl FromStr for Engine {
    type Err = String;

    fn from_str(s: &str) -> Result<Engine, Self::Err> {
        match s.to_lowercase().as_str() {
            "kvs" => Ok(Engine::Kvs),
            "sled" => Ok(Engine::Sled),
            _ => Err(format!("'{}' is not a valid engine", s)),
        }
    }
}

#[derive(Debug)]
pub enum Engine {
    Kvs,
    Sled,
}
