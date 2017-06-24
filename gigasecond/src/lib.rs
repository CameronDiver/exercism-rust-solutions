extern crate chrono;
use chrono::*;
// use std::time;
// use std::time::Duration;

pub fn after(val :DateTime<UTC>) -> DateTime<UTC> {
    return val + Duration::seconds(1000000000);
}
