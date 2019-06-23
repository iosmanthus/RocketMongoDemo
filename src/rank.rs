use super::DbConn;
use super::Message;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc};
use rocket_contrib::json::JsonValue;

#[get("/rank/<count>")]
pub fn rank(conn: DbConn, count: u32) -> JsonValue {
    let collection = conn.collection("movies");

    let result = collection
        .aggregate(
            vec![
                doc! {
                    "$sort": doc!{
                        "rate": -1
                    }
                },
                doc! {
                    "$limit": count
                },
                doc! {
                    "$project": doc!{
                        "_id": 0
                    }
                },
            ],
            None,
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
