use super::json_to_doc;
use super::DbConn;
use super::Message;
use super::Movie;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc};
use rocket_contrib::json::{Json, JsonValue};

#[post("/insert", data = "<movie>")]
pub fn insert(conn: DbConn, movie: Json<Movie>) -> JsonValue {
    let collection = conn.collection("movies");

    movie
        .validate()
        .map(|_| {
            let doc = doc! {
                "name": &movie.name,
                "director": &movie.director,
            };
            let result = collection.find_one(Some(doc), None).unwrap();
            if result.is_none() {
                let _ = collection.insert_one(json_to_doc(movie), None).unwrap();
                Message::<JsonValue>::new(200, Some("Successfully inserted".to_string()), None)
            } else {
                Message::<JsonValue>::new(200, Some("Record has already exist".to_string()), None)
            }
        })
        .unwrap_or_else(|err| Message::<JsonValue>::new(500, Some(format!("{}", err)), None))
}
