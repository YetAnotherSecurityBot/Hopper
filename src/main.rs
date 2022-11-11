#![forbid(unsafe_code)]
#[macro_use]
extern crate rocket;

use std::fmt::format;
use rocket::serde::{Deserialize, json::Json};
use std::net::SocketAddr;
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::LogType::{JOINED_SERVER, LEFT_SERVER, PFP_COMPARISON_RESULT, NEW_MESSAGE};

lazy_static! {
    static ref PFP_COMPARISON_RESULT_COUNTER: Mutex<i32> = Mutex::new(0);
    static ref JOINED_SERVER_COUNTER: Mutex<i32> = Mutex::new(0);
    static ref LEFT_SERVER_COUNTER: Mutex<i32> = Mutex::new(0);
    static ref NEW_MESSAGE_COUNTER: Mutex<i32> = Mutex::new(0);
}

#[get("/")]
fn index() -> &'static str {
    "Hopper!"
}

#[catch(404)]
fn not_found_general() -> &'static str {
    "Not Found"
}

#[catch(400)]
fn bad_request() -> &'static str {
    "Bad Request"
}

#[post("/", format = "json", data="<data>")]
fn log(data: Json<LogData>, remote_addr: SocketAddr) -> String {
    // TODO: log this somewhere
    if data.log_type == PFP_COMPARISON_RESULT {
        *PFP_COMPARISON_RESULT_COUNTER.lock().unwrap() += 1;
    }
    if data.log_type == JOINED_SERVER {
        *JOINED_SERVER_COUNTER.lock().unwrap() += 1;
    }
    if data.log_type == LEFT_SERVER {
        *LEFT_SERVER_COUNTER.lock().unwrap() += 1;
    }
    if data.log_type == NEW_MESSAGE {
        *NEW_MESSAGE_COUNTER.lock().unwrap() += 1;
    }
    "Logged".to_string()
}

#[get("/metrics")]
fn metrics() -> String {
    let mut result = String::new();
    result.push_str("# HELP yasb_pfp_comparisons_total The number of times a pfp comparison result was logged\n");
    result.push_str(format!("yasb_pfp_comparisons_total {}\n", PFP_COMPARISON_RESULT_COUNTER.lock().unwrap()).as_str());
    result.push_str("# HELP yasb_new_messages\n");
    result.push_str(format!("yasb_new_messages {}\n", NEW_MESSAGE_COUNTER.lock().unwrap()).as_str());
    result
}

#[launch]
fn rocket() -> _ {
    // Ignore the error on the underscore. It works fine, trust me.
    rocket::build()
        .mount("/", routes![index, log, metrics])
        .register("/", catchers![not_found_general, bad_request])
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct LogData {
    log_type: LogType,
    log_message: String
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
enum LogType {
    PFP_COMPARISON_RESULT,
    JOINED_SERVER,
    LEFT_SERVER,
    NEW_MESSAGE,
}