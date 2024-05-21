use couch_rs::database::Database;
use couch_rs::error::CouchError;
use couch_rs::types::find::FindQuery;
use dotenv::dotenv;

pub mod conn;
pub mod query;
pub mod traits;

#[cfg(test)]
mod tests {

    use super::*;

    use self::traits::QueryGeneric;
    use conn::Conn;
    use query::new_id;
    use serde_json::Value;
    use traits::{BasicOperations, Input, Output, QGEnum, Queries};
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Test {
        #[serde(skip_serializing_if = "Option::is_none")]
        _id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        _rev: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        age: Option<i32>,
    }

    impl Queries<Test> for Test {
        fn query_fmt(&self) -> Result<Value, String> {
            Ok(serde_json::to_value(self).unwrap())
        }
        fn query<T: QueryGeneric<T>>(&self, input: Value) -> Result<T, String> {
            let res: T = T::new_with_input(QGEnum::Val(input));
            Ok(res)
        }
    }
    #[tokio::test]
    async fn select() {
        let test = Test {
            _id: None,
            _rev: None,
            name: Some("jack".to_string()),
            age: None,
        };

        let mut conn = Conn::new().await;
        conn.db("test").await;

        let select = conn.select(Input::Raw(test)).await;
        println!("{:#?}", select);
    }

    #[tokio::test]
    async fn all() {
        let mut conn = Conn::new().await;
        conn.db("test").await;

        let select: Output<Test, String> = conn.all().await;
        println!("{:#?}", select);
    }

    #[tokio::test]
    async fn insert_or_update() {
        let test = Test {
            _id: Some(new_id()),
            _rev: None,
            name: Some("jack".to_string()),
            age: Some(25),
        };

        let mut conn = Conn::new().await;
        conn.db("test").await;

        let upsert = conn.insert_or_update(Input::Raw(test)).await;
        println!("{:#?}", upsert);
    }

    #[tokio::test]
    async fn delete() {
        let test = Test {
            _id: None,
            _rev: None,
            name: Some("test".to_string()),
            age: Some(10),
        };

        let mut conn = Conn::new().await;
        conn.db("test").await;

        let select: Output<String, String> = conn.delete(Input::Raw(test)).await;
        println!("{:?}", select);
    }
}
