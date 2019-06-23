use super::DbConn;
use super::Message;
use mongodb::coll::options::FindOptions;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc};
use rocket_contrib::json::JsonValue;

#[get("/query?<name>")]
pub fn query(conn: DbConn, name: String) -> JsonValue {
    let collection = conn.collection("movies");

    let result = collection
        .find(
            Some(doc! {
                "name": name
            }),
            Some(FindOptions {
                projection: Some(doc! {"_id":false}),
                ..FindOptions::default()
            }),
        )
        .unwrap()
        .into_iter()
        .map(|doc| doc.unwrap())
        .collect::<Vec<_>>();

    json!(Message::new(
        200,
        Some("Successfully queried".to_string()),
        Some(json! {result})
    ))
}
