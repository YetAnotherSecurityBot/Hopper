#![forbid(unsafe_code)]
#[macro_use]
extern crate rocket;

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

#[post("/", format = "json")]
fn log_error() -> String {
    // TODO: log this somewhere
    "Logged".to_string()
}

#[launch]
fn rocket() -> _ {
    // Ignore the error on the underscore. It works fine, trust me.
    rocket::build()
        .mount("/", routes![index, log_error])
        .register("/", catchers![not_found_general, bad_request])
}
