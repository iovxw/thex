#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket_contrib::{Json, Value};
use rocket::State;

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Debug, Serialize, Deserialize)]
struct Chest {
    x: i64,
    y: i64,
    z: i64,
    lv: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: String,
    uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewChestReq {
    player: Player,
    chest: Chest,
}

#[post("/", format = "application/json", data = "<message>")]
fn newchest(message: Json<NewChestReq>, map: State<MessageMap>) -> Json<Value> {
    println!("{:?}", message);
    Json(json!({ "status": "ok" }))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

#[error(400)]
fn bad_request() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Bad request."
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/newchest", routes![newchest])
        .catch(errors![not_found, bad_request])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}
