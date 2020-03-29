use qifi_rs::from_string;
use serde_json::Value;

fn main() {
    let qifi_string = r#"{"account_cookie": "T01B2_IF2004_1min", "password": "T01B2_IF2004_1min", "accounts": {"user_id": "T01B2_IF2004_1min", "currency": "CNY", "pre_balance": 1000000, "deposit": 0, "withdraw": 0, "WithdrawQuota": 0, "close_profit": 0, "commission": 0, "premium": null, "static_balance": 1000000, "position_profit": 0, "float_profit": 0, "balance": 1000000, "margin": 0, "frozen_margin": 0, "frozen_commission": 0.0, "frozen_premium": 0.0, "available": 1000000, "risk_ratio": 0.0}, "bank_password": "", "bankid": "QASIM", "bankname": "QASIMBank", "banks": {"QASIM": {"id": "QASIM", "name": "QASIMBank", "bank_account": "", "fetch_amount": 0.0, "qry_count": 0}}, "broker_name": "QAPaperTrading", "capital_password": "", "databaseip": "", "events": {}, "frozen": {}, "investor_name": "", "model": "SIM", "money": 1000000, "orders": {}, "ping_gap": 5, "portfolio": "QAPaperTrade", "positions": {}, "pub_host": "", "settlement": {}, "sourceid": "QIFI_Account", "status": 200, "taskid": "", "trade_host": "", "trades": {}, "trading_day": "2020-03-26", "transfers": {}, "updatetime": "", "wsuri": "ws://www.yutiansut.com:7988"}"#;
    let string = String::from(qifi_string);
    let qifi: Value = from_string(string).unwrap();
    let _deserialize = serde_json::to_string(&qifi).expect("呀 反序列化失败,请检查你的字符串格式");
}
