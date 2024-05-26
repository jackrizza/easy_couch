use crate::scheme::Todo;
use easy_couch::conn::Conn;
use easy_couch::traits::{BasicOperations, Input, Output};

pub struct MyApp {
    pub todos: Vec<Todo>,
    pub form_item: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            todos: Vec::new(),
            form_item: String::new(),
        }
    }
}

impl MyApp {
    pub fn tokio_get(&mut self) {
        let _: Result<_, String> = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { Ok(self.get().await) });
    }

    pub async fn get(&mut self) {
        let mut conn = Conn::new().await;
        let _ = conn.db("todos").await;

        let select: Output<Todo, String> = conn.all().await;
        match select {
            Output::Single(val) => self.todos = vec![val],
            Output::Multiple(val) => self.todos = val,
            _ => {}
        };
    }

    pub fn tokio_update_or_insert(&self, input: Todo) {
        let _: Result<_, String> = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { Ok(self.update_or_insert(input).await) });
    }

    pub async fn update_or_insert(&self, input: Todo) {
        let mut conn = Conn::new().await;
        let _ = conn.db("todos").await;

        let _ = conn.insert_or_update(Input::Raw(input)).await;
    }

    pub fn tokio_delete(&self, input: Todo) {
        let _: Result<_, String> = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { Ok(self.delete(input).await) });
    }

    pub async fn delete(&self, input: Todo) {
        let mut conn = Conn::new().await;
        let _ = conn.db("todos").await;

        let _ = conn.delete(Input::Raw(input)).await;
    }
}
