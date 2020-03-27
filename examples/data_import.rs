use qifi_rs::{Bar, from_str, from_serde_value};
use serde_json::Value;
fn main() {
    let test_json_str = r#"{
     "high":2900,
     "low":2880,
     "open": 2891,
     "close": 2899,
     "volume":100,
     "amount": 51454131
    }"#;
    let val:Value = from_str(test_json_str).unwrap();
    let bar:Bar = from_serde_value(val).unwrap();
    println!("{:?}", bar);
}
