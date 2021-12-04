use rocket::get;
use rocket::response::content::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use std::str::FromStr;

use crate::establish_connection;
use crate::models::Sanduk;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

// function will return Json response which contains (get_all_with_sender and get_all_with_receiver function response, which get their response from our indexed table).
#[get("/<pubkey>")]
pub fn get_all_sanduk(pubkey: &str) -> Json<Value> {
    let pubkey_string = String::from_str(pubkey).unwrap();
    let conn = establish_connection();
    Json(
        json!({"status":"success","sending":Sanduk::get_all_with_sender(&pubkey_string, &conn),"receiving":Sanduk::get_all_with_receiver(&pubkey_string, &conn)}),
    )
}
