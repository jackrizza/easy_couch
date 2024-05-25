use super::*;

use self::traits::{Input, Output, Queries};

use couch_rs::types::document::DocumentCreatedDetails;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use traits::BasicOperations;

pub struct Conn {
    _conn: Result<couch_rs::Client, CouchError>,
    database: Option<Database>,
}

#[allow(unused)]
impl Conn {
    pub async fn new() -> Self {
        dotenv().ok();

        let username = std::env::var("USERNAME").expect("No Username");
        let password = std::env::var("PASSWORD").expect("No Password");
        let url = std::env::var("URL").expect("No URL");

        Self {
            _conn: couch_rs::Client::new(&url, &username, &password),
            database: None,
        }
    }

    pub async fn db(&mut self, db: &str) -> Result<(), String> {
        match self._conn {
            Ok(ref mut conn) => {
                self.database = conn.db(db).await.ok();
                Ok(())
            }
            Err(ref e) => Err(format!("Error: {}", e)),
        }
    }

    pub fn raw_database(&self) -> &Database {
        self.database.as_ref().expect("Database is not set")
    }
}

impl<B> BasicOperations<B> for Conn
where
    B: Serialize + DeserializeOwned + Queries<B>,
{
    async fn select(&self, input: Input<B>) -> Output<B, String> {
        let db = self.raw_database();
        let query = input.matcher();
        let result = db.find(&query).await;

        match result {
            Ok(res) => {
                let docs = res.rows;
                let mut vec = Vec::new();
                for doc in docs {
                    vec.push(serde_json::from_value(doc).unwrap());
                }
                Output::Multiple(vec)
            }
            Err(e) => Output::Error(e.to_string()),
        }
    }

    async fn all(&self) -> Output<B, String> {
        let db = self.raw_database();
        let res = db.get_all().await;
        match res {
            Ok(res) => {
                let docs = res.rows;
                let mut vec = Vec::new();
                for doc in docs {
                    vec.push(serde_json::from_value(doc).unwrap());
                }
                Output::Multiple(vec)
            }
            Err(e) => Output::Error(e.to_string()),
        }
    }

    async fn insert_or_update(&self, input: Input<B>) -> Output<Value, String> {
        let db = self.raw_database();
        let query: Input<Value> = input.matcher();

        let result: Output<Result<DocumentCreatedDetails, CouchError>, String> = match query {
            Input::Formatted(mut q) => Output::Single(db.upsert(&mut q).await),
            Input::Multiple(vec) => {
                let mut res = Vec::new();
                for mut query in vec {
                    let result = db.upsert(&mut query).await;
                    res.push(result);
                }
                Output::Multiple(res)
            }
            Input::Raw(_) | Input::None => Output::Error("No input".to_string()),
        };

        match result {
            Output::Single(res) => match res {
                Ok(res) => Output::Single(serde_json::to_value(&res).unwrap()),
                Err(e) => Output::Error(e.to_string()),
            },
            Output::Multiple(res) => {
                let mut vec = Vec::new();
                for r in res {
                    match r {
                        Ok(r) => vec.push(serde_json::to_value(&r).unwrap()),
                        Err(e) => vec.push(serde_json::from_str(&e.to_string()).unwrap()),
                    }
                }
                Output::Multiple(vec)
            }
            Output::Error(e) => Output::Error(e),
            Output::Sucsess(_) | Output::None => Output::None,
        }
    }

    async fn delete(&self, input: Input<B>) -> Output<String, String> {
        let db = self.raw_database();
        match input.matcher() {
            Input::Raw(q) | Input::Formatted(q) => {
                let res = db.remove(&serde_json::to_value(q).unwrap()).await;
                match res {
                    true => Output::Sucsess("Deleted".to_string()),
                    false => Output::Error("Unable to delete".to_string()),
                }
            }
            Input::Multiple(q) => {
                let mut res: Vec<String> = Vec::new();
                for query in q {
                    let ex = match db.remove(&serde_json::to_value(query).unwrap()).await {
                        true => "Deleted".to_string(),
                        false => "Unable to delete".to_string(),
                    };
                    res.push(ex);
                }

                Output::Multiple(res)
            }
            Input::None => Output::None,
        }
    }
}
