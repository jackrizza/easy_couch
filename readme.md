## easy_couch

Easy_couch is a simple and easy to use CouchDB client for rust. It is built on top of the [couch_rs](https://github.com/mibes/couch-rs) library and provides a simple interface to interact with CouchDB. The project goal is to make a all application api that requires minimal code.

```rust
#[derive(Debug, Serialize, Deserialize, QueryMacro)]
struct Foo {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    _rev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bar: Option<String>,
}

async fn select() {
	let foo = Foo {
    _id: None,
    _rev: None,
    bar: Some("baz".to_string()),
  };

  let mut conn = Conn::new().await;
  let _ = conn.db("foobar").await;

  let select = conn.select(Input::Raw(foo)).await;
 	println!("{:#?}", select);
}


```



### .env

a `.env` file is required for operation in your working directory. 

```bash
USERNAME=INSERT_USERNAME
PASSWORD=INSERT_PASSWORD
URL=INSERT_URL_WITH_PORT
```




#### Egui Todo Example
The `examples/todo` directory contains a simple todo application that uses EasyCouch to interact with a CouchDB instance. The application uses the egui library for the GUI. To run the example, you need to have a CouchDB instance running.

![Alt text](examples/todo/example_screenshot.png "a title")
