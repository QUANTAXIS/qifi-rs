/*
此处描述了quantaxis-rs标准的期货/股票/数据结构
*/
use serde::{Serialize, Deserialize};

fn default_f64() -> f64 {
    0f64
}

fn default_i64() -> i64 {
    0i64
}

fn default_bool() -> bool {
    false
}

fn default_i128() -> i128 {
    0i128
}

fn default_i32() -> i32 {
    0i32
}

fn default_string() -> String {
    "".to_string()
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct DataItem {
    #[serde(default = "default_f64")]
    open: f64,
    #[serde(default = "default_f64")]
    high: f64,
    #[serde(default = "default_f64")]
    low: f64,
    #[serde(default = "default_f64")]
    close: f64,
    #[serde(default = "default_i64")]
    volume: i64,
}

/// Bar格式的数据
///     high: 高
///     low: 低
///     close: 收
///     open: 开
///     volume: 成交量
///     amount: 成交金额
/// Examples
/// ```
/// use crate::qifi_rs::{Bar, from_str,from_serde_value};
/// use serde_json::Value;
/// let test_json_str = r#"{
///     "high":2900,
///     "low":2880,
///     "open": 2891,
///     "close": 2899,
///     "volume":100,
///     "amount": 51454131
/// }"#;
/// let val:Value = from_str(test_json_str).unwrap();
/// let bar:Bar = from_serde_value(val).unwrap();
/// println!("{:?}",  bar);
///
/// ```
#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Bar {
    #[serde(default = "default_f64")]
    high: f64,
    #[serde(default = "default_f64")]
    low: f64,
    #[serde(default = "default_f64")]
    close: f64,
    #[serde(default = "default_f64")]
    open: f64,
    #[serde(default = "default_i64")]
    volume: i64,
    #[serde(default = "default_f64")]
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
    #[serde(default = "default_f64")]
    price: f64,
    #[serde(default = "default_i128")]
    open_interest: i128,
    #[serde(default = "default_f64")]
    high: f64,
    #[serde(default = "default_f64")]
    low: f64,
    #[serde(default = "default_f64")]
    close: f64,
    #[serde(default = "default_f64")]
    open: f64,
    #[serde(default = "default_i64")]
    volume: i64,
    #[serde(default = "default_f64")]
    amount: f64,
}

/// 获取当前code的代码以及限制数量
///     code: 代码名称
///     ip: ip地址
///     limit: 限制数量
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Scode {
    code: String,
    ip: String,
    limit: i32,
}

/// L2行情数据， 注意我为此都添加大量的接口
///     index: 索引
///     time: 时间
///     price: 最新价格
///     isbuy:是否买
///     vol: 成交量
///     buyno:
///     sellno:
///     buyprice:
///     sellprice:
///     buyvol:
///     sellvol:
///     marketname: 市场名称
///     code:
#[derive(Serialize, Deserialize, Debug)]
struct L2X {
    #[serde(default = "default_i32")]
    index: i32,
    #[serde(default = " default_string")]
    time: String,
    #[serde(default = "default_f64")]
    price: f64,
    #[serde(default = "default_bool")]
    isbuy: bool,
    #[serde(default = "default_i32")]
    vol: i32,
    #[serde(default = "default_i32")]
    buyno: i32,
    #[serde(default = "default_i32")]
    sellno: i32,
    #[serde(default = "default_f64")]
    buyprice: f64,
    #[serde(default = "default_f64")]
    sellprice: f64,
    #[serde(default = "default_i32")]
    buyvol: i32,
    #[serde(default = "default_i32")]
    sellvol: i32,
    #[serde(default = " default_string")]
    marketname: String,
    #[serde(default = " default_string")]
    code: String,
}