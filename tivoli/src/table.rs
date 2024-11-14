use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::result::Result::Ok;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, BufReader, BufWriter};
#[derive(Serialize, Deserialize)]
struct TableSchema {
    columns: Vec<Variable>,
}
/*
* Little rant, We will come across the issue of searching without a key
* (This is how we search BTW), I aim to fix this via sharding this table.
* This might be excessive, and I dont know how to do this logically,
* But this is a thought, as I feel having an in memory copy of the entire log
* defeats the purpose of having the log there in the first place
* and async as a whole, ig it makes it durable ¯\_(ツ)_/¯, When i think
* about building stuff i think about Big Bezos would build it, yk? Thats all...
*/
pub struct Table {
    //find better datatype/structure than this, maybe stack
    pub rows: Vec<Vec<String>>,
    pub index: BTreeMap<String, usize>,
    pub schema: TableSchema,
    pub offset: usize,
    path: PathBuf,
}
#[derive(Serialize, Deserialize)]
pub enum Variable {
    VC(String),
    TF(String),
    NU(String),
}

impl Table {
    pub async fn new(table_name: &str, schema: Vec<Variable>) -> Result<Table> {
        let table_dir = format!("./data/{}", table_name);
        let path = PathBuf::from(table_dir);
        if !Path::new(&path).exists() {
            fs::create_dir_all(&path).await?;
        }
        let schema = TableSchema { columns: schema };

        let rows = Vec::new();

        Ok(Table {
            //rows is treated like cashe, its purpose is to hold data, to then dump it in the csv
            rows,
            index: BTreeMap::new(),
            schema,
            offset: 0,
            path: PathBuf::from(path),
        })
    }

    pub async fn load_schema(schema_path: &str) -> Result<TableSchema> {
        let file = File::open(schema_path).await?;
        let mut reader = BufReader::new(file);
        let mut cnt = String::new();
        reader.read_to_string(&mut cnt).await?;
        let schema: TableSchema = serde_json::from_str(&cnt)?;
        Ok(schema)
    }

    pub async fn save_schema(&self) -> Result<()> {
        let schema_file = format!("{}/{}.schema", self.path.display(), "table");
        println!("Saving Schema to: {}", schema_file);
        let file = File::create(schema_file).await?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.schema.columns)?;
        Ok(())
    }
}
