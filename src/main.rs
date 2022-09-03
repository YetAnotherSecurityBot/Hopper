#![forbid(unsafe_code)]
#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, json::Json};
use std::net::SocketAddr;

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
    "Logged".to_string()
}

#[get("/metrics")]
fn metrics() -> String {
    format!("e")
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
    log_type: LogType
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
enum LogType {
    PFP_COMPARISON_RESULT
}