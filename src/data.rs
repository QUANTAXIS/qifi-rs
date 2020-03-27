/*
此处描述了quantaxis-rs标准的期货/股票/数据结构
*/
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct DataItem {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Bar {
    high: f64,
    low: f64,
    close: f64,
    open: f64,
    volume: i64,
    amount: i64,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Tick {
    price: f64,
    high: f64,
    low: f64,
    close: f64,
    open: f64,
    volume: i64,
    amount: i64,
}


// trait DataApi {
//     fn as_data_item(&mut self) -> DataItem {
//         // 转换为所需要的DataItem
//         DataItem { open: self.open, high: self.high, close: self.close, low: self.low, volume: self.volume }
//     }
//
//     fn from_vec(data) -> () {
//         // 重写vec方法
//     }
//
//     fn from_data_item(data: DataItem) -> () {
//
//
//     }
// }

// impl DataApi for Bar {
//
// }