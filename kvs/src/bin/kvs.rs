use anyhow::{Context, Result};
use clap::Parser;
use kvs::kv::KvStore;
#[derive(Parser)]
struct Cli {
    cmd: String,
    k: Option<String>,
    v: Option<String>,
}
fn main(){
    let args = Cli::parse();
    let mut db = KvStore::new();
    match args.cmd.as_str() {
        "get" => {
            if let Some(ref _k) = args.k {
                println!("{:?}", db.get(_k));
            }
        }
        "rm" => {
            if let Some(ref _k) = args.k {
                db.rm(_k);
            }
        }
        "set" => {
            if let Some(ref _k) = args.k {
                if let Some(ref _v) = args.v {
                    db.set(_k, _v);
                }
            }
        }
        "-V" => {
            println!("version 1.0.1");
            println!("{:?}", db.offset_map);
        }
        _ => {}
    }
    
}
