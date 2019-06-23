#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

use mongodb::db::Database;
use mongodb::{doc, Bson, Document};
use rocket_contrib::json::{Json, JsonValue};
use serde::Serialize;
use serde_json::value::Value;
use time::strptime;

mod insert;
mod query;
mod rank;

type Result<T> = std::result::Result<T, String>;

#[database("mongodb")]
pub struct DbConn(Database);

#[derive(Serialize, Deserialize, Clone)]
pub struct Movie {
    name: String,
    release_time: String,
    director: String,
    rate: f64,
}

impl Movie {
    pub fn validate(&self) -> Result<()> {
        strptime(&self.release_time, "%F").map_err(|err| format!("{:?}", err))?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct Message<T: Serialize> {
    status: u32,
    msg: Option<String>,
    payload: Option<T>,
}

impl<T: Serialize> Message<T> {
    pub fn new(status: u32, msg: Option<String>, payload: Option<T>) -> JsonValue {
        json!(Message {
            status,
            msg,
            payload
        })
    }
}

fn json_to_bson<T: Serialize>(json: Json<T>) -> Bson {
    Bson::from(<JsonValue as Into<Value>>::into(json!(json.into_inner())))
}

fn bson_to_document(bson: Bson) -> Document {
    bson.as_document().unwrap().clone()
}

fn json_to_doc<T: Serialize>(json: Json<T>) -> Document {
    bson_to_document(json_to_bson(json))
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount(
            "/",
            routes![self::insert::insert, self::query::query, self::rank::rank],
        )
        .launch();
}
