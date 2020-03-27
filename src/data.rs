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
    #[serde(default = 0f64)]
    high: f64,
    #[serde(default = 0f64)]
    low: f64,
    #[serde(default = 0f64)]
    close: f64,
    #[serde(default = 0f64)]
    open: f64,
    #[serde(default = 0i64)]
    volume: i64,
    #[serde(default = 0f64)]
    amount: f64,
}

/// Tick格式的数据
///     price: 最新价
///     open_interest: 持仓量
///     high: 高
///     low: 低
///     close: 收
///     open: 开
///     volume: 成交量
///     amount: 成交金额
/// Examples
/// ```
///
/// ```
#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Tick {
    #[serde(default = 0f64)]
    price: f64,
    #[serde(default = 0i128)]
    open_interest: i128,
    #[serde(default = 0f64)]
    high: f64,
    #[serde(default = 0f64)]
    low: f64,
    #[serde(default = 0f64)]
    close: f64,
    #[serde(default = 0f64)]
    open: f64,
    #[serde(default = 0i64)]
    volume: i64,
    #[serde(default = 0f64)]
    amount: f64,
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
