use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, Write};
// use std::{collections::HashMap, process::Command};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::SeekFrom;

const PATH: &str = "/home/raws4uce/projects/dark/log.txt"; //obviously change this to a txt file in your own dir,
                                                           // yes i use linux

#[derive(Default, Debug)]
pub struct KvStore {
    pub offset_map: HashMap<String, usize>,
}
#[derive(Serialize, Deserialize)]
enum Cmd {
    Set(String, String),
    Get(String),
    Rm(String),
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            offset_map: HashMap::new(),
        }
    }

    pub fn set(&self, k: &String, v: &String) {
        let cmd = Cmd::Set(k.to_string(), v.to_string());
        //seralise
        let serialised_cmd = serde_json::to_string(&cmd).expect("Failed to serialise command");
        //write to log
        let mut log = OpenOptions::new()
            .append(true)
            .create(true)
            .open(PATH)
            .expect("fail to open log file");

        writeln!(log, "{}", serialised_cmd).expect("fail to write to log file");
        //error handling
    }

    pub fn get(&mut self, k: &String) -> Option<String> {
        let log = std::fs::read_to_string(PATH)
            .with_context(|| format!(""))
            .expect("");

        self.offset_map = KvStore::offset_map_init(&log);

        let pos = self.offset_map.get(k).copied();

        let f = File::open(PATH).expect("fail to open log file");
        let mut read = BufReader::new(f);

        if let Some(byt) = pos {
            read.seek(SeekFrom::Start(byt as u64))
                .expect("fail to seek at point in log file, potential edge case maybe");
            let mut line = String::new();
            read.read_line(&mut line)
                .expect("fail to read line from seek point (offset)");
            //got json ->  cmd : [k,v]
            let on: Value = serde_json::from_str(&line.as_str())
                .expect("fail to parse Json, pronounced /ˈdʒeɪsən/ ");
            if let Some((_, tpe)) = on.as_object().and_then(|f| f.iter().next()) {
                if let Some(kv) = tpe.as_array() {
                    if let Some(v) = kv.get(1).and_then(|d| d.as_str()) {
                        return Some(v.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn rm(&mut self, k: &String) {
        //add rm to log
        let log = std::fs::read_to_string(PATH)
            .with_context(|| format!(""))
            .expect("fail to read log file to string");

        self.offset_map = KvStore::offset_map_init(&log);

        let pos = self.offset_map.get(k).copied();

        match pos {
            Some(_) => {
                let cmd = Cmd::Rm(k.to_string());
                let serialised_json = serde_json::to_string(&cmd).expect("msg");
                let mut log = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(PATH)
                    .expect("fail to open log file");

                writeln!(log, "{}", serialised_json).expect("fail to write to log file");
            }
            None => {}
        }
    }

    pub fn offset_map_init(log: &String) -> HashMap<String, usize> {
        let mut offset_map: HashMap<String, usize> = HashMap::new();

        let mut offset = 0;
        for json in log.lines() {
            //need to get key from current json/line
            // println!("{:?}",json);
            let v: Value =
                serde_json::from_str(json).expect("fail to parse json, pronounced /ˈdʒeɪsən/ ");
            if let Some((cmd_type, cmd_args)) = v.as_object().and_then(|f| f.iter().next()) {
                if let Some(kv) = cmd_args.as_array() {
                    if let Some(k) = kv.get(0).and_then(|k| k.as_str()) {
                        match cmd_type.as_str() {
                            "Set" => {
                                offset_map.insert(k.to_string(), offset);
                            }
                            "Rm" => {
                                offset_map.remove_entry(k);
                            }
                            _ => {}
                        }
                    }
                }
            }
            offset += json.as_bytes().len() + 1;
        }
        offset_map
    }
}
