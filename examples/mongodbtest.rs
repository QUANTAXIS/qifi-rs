use bson::Bson;
use mongodb::sync::Client;
use bson::Document;

use qifi_rs::{from_string, QIFI, from_serde_value, to_doc};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

extern crate serde;
extern crate serde_json;

fn main() {
    let client = Client::with_uri_str("mongodb://192.168.2.117:27017").unwrap();//("Failed to initialize standalone client.");
    let coll = client.database("QAREALTIME").collection("account");
    let cursor = coll.find(None, None).ok().expect("Failed to execute find.");
    // 将读取出来的数据载入为json字符串
    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    let first = &docs[0];
    let serialized = serde_json::to_string(&first).unwrap();
    //println!("{}", serialized);
    // 转换为Value
    let x = from_string(serialized).unwrap();
    // 转换为结构体
    let c: QIFI = from_serde_value(x).unwrap();
    // 转换为document
    let v = to_doc(c);
    println!("{:#?}", v);
    
    //coll.insert_one(v, None);
}