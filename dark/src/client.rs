
use anyhow::{
    Result,
    anyhow
};
use reqwest::blocking::Client;
use serde_json::json;
use serde_json::Value;
/*
establish con at addr
parse as json
send it across "protocol"
get response
    OKclient.rs(37, 27): you might be missing a string literal to format with: `"{} {}", `
View Problem (Alt+F8)
Quick Fix... (Ctrl+.)

    Err

*/

#[derive(Debug)]
pub struct KvsClient {
    client: Client,
    url: String,
}

impl KvsClient {
    pub fn connect(addr_str: &String) -> Result<Self> {
        let client = Client::new();
        let url = format!("http://{}", addr_str);

        Ok(KvsClient { client, url })
    }

    pub fn set(&self, k: &String, v: &String) {
        let payload = json!({
            "cmd" : "set".to_owned(),
            "key": k,
            "value": v
        });

        let response = self
            .client
            .get(&format!("{}/SET", self.url))
            .json(&payload)
            .send()
            .expect("set, error, response");

        if response.status().is_success() {
            println!("Bueno");
        } else {
            eprintln!("Err")
        }
    }
    pub fn rm(&self, k: &String) {
        let payload = json!({
            "cmd" : "rm".to_owned(),
            "key" : k,
        });

        let response = self
            .client
            .get(&format!("{}/SET", self.url))
            .json(&payload)
            .send()
            .expect("remove, error, response");

        if response.status().is_success() {
            println!("Bueno");
        } else {
            eprintln!("Err")
        }
    }
    pub fn get(self, k: &String) -> Result<String> {
        let payload = json!({
            "cmd" : "get".to_owned(),
            "key" : k
        });

        let response = self
            .client
            .get(&format!("{}/GET", self.url))
            .json(&payload)
            .send()
            .expect("get, error, response");

        if response.status().is_success() {
            let txt = response.text().expect("fail to stringify");
            let json :Value = serde_json::from_str(&txt).expect("msg");

            if let Some(val) = json.get("value"){
                return Ok(val.as_str().unwrap().to_string());            
            }else{
                return Err(anyhow!("error, key not found"));
            }
        } else {
            return Err(anyhow!("Error, getting not gotted"));
        }
    }
}
