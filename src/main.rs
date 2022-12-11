#![forbid(unsafe_code)]
#[macro_use]
extern crate rocket;

use std::env;
use ctrlc;
use rocket::serde::{Deserialize, json::Json};
use std::net::SocketAddr;
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::LogType::{JoinedServer, LeftServer, PfpComparisonResult, NewMessage};

lazy_static! {
    static ref PFP_COMPARISON_RESULT_COUNTER: Mutex<i32> = Mutex::new(0);
    static ref JOINED_SERVER_COUNTER: Mutex<i32> = Mutex::new(0);
    static ref LEFT_SERVER_COUNTER: Mutex<i32> = Mutex::new(0);
    static ref TOTAL_SERVERS: Mutex<i32> = Mutex::new(0);
    static ref NEW_MESSAGE_COUNTER: Mutex<i32> = Mutex::new(0);
    static ref SECRET_KEY: Mutex<String> = Mutex::new(String::new());
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
fn log(data: Json<LogData>, _remote_addr: SocketAddr) -> String {
    // Set secret key from env
    if *SECRET_KEY.lock().unwrap() == String::new()  {
        *SECRET_KEY.lock().unwrap() = env::var("HOPPER_SECRET_KEY").unwrap();
    }
    // Check if secret key is correct, to prevent random people from sending data
    if data.key != *SECRET_KEY.lock().unwrap() {
        return "Invalid Secret Key!".to_string();
    }
    // Do the logging things. I could probably match statement this.

    return match data.log_type {
        PfpComparisonResult => {
            *PFP_COMPARISON_RESULT_COUNTER.lock().unwrap() += 1;
            "Pfp Comparison Result Logged!".to_string()
        }
        JoinedServer => {
            *JOINED_SERVER_COUNTER.lock().unwrap() += 1;
            *TOTAL_SERVERS.lock().unwrap() += 1;
            "Joined Server Logged!".to_string()
        }
        LeftServer => {
            *LEFT_SERVER_COUNTER.lock().unwrap() += 1;
            *TOTAL_SERVERS.lock().unwrap() -= 1;
            "Left Server Logged!".to_string()
        }
        NewMessage => {
            *NEW_MESSAGE_COUNTER.lock().unwrap() += 1;
            "New Message Logged!".to_string()
        }
    }
}

#[get("/metrics")]
fn metrics() -> String {
    // Build very very simple prometheus metrics page.
    let mut result = String::new();
    result.push_str("# HELP yasb_pfp_comparisons_total The number of times a pfp comparison result was logged\n");
    result.push_str(format!("yasb_pfp_comparisons_total {}\n", PFP_COMPARISON_RESULT_COUNTER.lock().unwrap()).as_str());
    result.push_str("# HELP yasb_new_messages\n");
    result.push_str(format!("yasb_new_messages {}\n", NEW_MESSAGE_COUNTER.lock().unwrap()).as_str());
    result.push_str("# HELP yasb_joined_servers\n");
    result.push_str(format!("yasb_joined_servers {}\n", JOINED_SERVER_COUNTER.lock().unwrap()).as_str());
    result.push_str("# HELP yasb_left_servers\n");
    result.push_str(format!("yasb_left_servers {}\n", LEFT_SERVER_COUNTER.lock().unwrap()).as_str());
    result.push_str("# HELP yasb_total_servers\n");
    result.push_str(format!("yasb_total_servers {}\n", TOTAL_SERVERS.lock().unwrap()).as_str());
    result
}

#[rocket::main]
async fn main() {
    let result = rocket::build()
        .mount("/", routes![index, log, metrics])
        .register("/", catchers![not_found_general, bad_request]);

    // Ignore the error on the underscore. It works fine, trust me.
    println!("Setting up Ctrl+C handler...");
    ctrlc::set_handler(|| {
        println!("Received SIGINT/Ctrl+C! Saving values.");
        std::fs::write("/logs/pfp_comparisons_total.txt", PFP_COMPARISON_RESULT_COUNTER.lock().unwrap().to_string()).unwrap();
        std::fs::write("logs/joined_server_total.txt", JOINED_SERVER_COUNTER.lock().unwrap().to_string()).unwrap();
        std::fs::write("logs/left_server_total.txt", LEFT_SERVER_COUNTER.lock().unwrap().to_string()).unwrap();
        std::fs::write("logs/new_messages_total.txt", NEW_MESSAGE_COUNTER.lock().unwrap().to_string()).unwrap();
        std::fs::write("logs/total_servers.txt", TOTAL_SERVERS.lock().unwrap().to_string()).unwrap();
        println!("Saved values! Shutting down rocket!");
        // Exit, probably doesn't matter we don't gracefully kill rocket. Who's gonna cry about it? Not me!
        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler!");
    println!("Ctrl+C handler set!");

    // Load saved values, if the files exist.
    println!("Attempting to load saved values...");
    if std::path::Path::new("logs/pfp_comparisons_total.txt").exists() {
        *PFP_COMPARISON_RESULT_COUNTER.lock().unwrap() = std::fs::read_to_string("logs/pfp_comparisons_total.txt").unwrap().parse::<i32>().unwrap();
        println!("Loaded pfp_comparisons_total.txt!: {}", PFP_COMPARISON_RESULT_COUNTER.lock().unwrap());
    }
    if std::path::Path::new("logs/joined_server_total.txt").exists() {
        *JOINED_SERVER_COUNTER.lock().unwrap() = std::fs::read_to_string("logs/joined_server_total.txt").unwrap().parse::<i32>().unwrap();
        println!("Loaded joined_server_total.txt!: {}", JOINED_SERVER_COUNTER.lock().unwrap());
    }
    if std::path::Path::new("logs/left_server_total.txt").exists() {
        *LEFT_SERVER_COUNTER.lock().unwrap() = std::fs::read_to_string("logs/left_server_total.txt").unwrap().parse::<i32>().unwrap();
        println!("Loaded left_server_total.txt!: {}", LEFT_SERVER_COUNTER.lock().unwrap());
    }
    if std::path::Path::new("logs/new_messages_total.txt").exists() {
        *NEW_MESSAGE_COUNTER.lock().unwrap() = std::fs::read_to_string("logs/new_messages_total.txt").unwrap().parse::<i32>().unwrap();
        println!("Loaded new_messages_total.txt!: {}", NEW_MESSAGE_COUNTER.lock().unwrap());
    }
    if std::path::Path::new("logs/total_servers.txt").exists() {
        *TOTAL_SERVERS.lock().unwrap() = std::fs::read_to_string("logs/total_servers.txt").unwrap().parse::<i32>().unwrap();
        println!("Loaded total_servers.txt!: {}", TOTAL_SERVERS.lock().unwrap());
    }

    let _ = result.launch().await.expect("Failed to launch rocket!");
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct LogData {
    log_type: LogType,
    log_message: String,
    key: String
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
enum LogType {
    PfpComparisonResult,
    JoinedServer,
    LeftServer,
    NewMessage,
}