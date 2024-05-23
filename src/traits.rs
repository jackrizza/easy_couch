use serde_json::Value;

#[allow(unused)]
#[derive(Debug)]
pub enum Input<B>
where
    B: Queries<B> + serde::Serialize + serde::de::DeserializeOwned,
{
    Raw(B),
    Formatted(B),
    Multiple(Vec<B>),
    None,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Output<C, E> {
    Single(C),
    Multiple(Vec<C>),
    Sucsess(E),
    Error(E),
    None,
}
pub enum QGEnum {
    Val(Value),
    Vec(Vec<Value>),
    None,
}

pub trait BasicOperations<B>
where
    B: Queries<B> + serde::Serialize + serde::de::DeserializeOwned,
{
    fn select(&self, input: Input<B>) -> impl std::future::Future<Output = Output<B, String>>;
    fn all(&self) -> impl std::future::Future<Output = Output<B, String>>;
    fn insert_or_update(
        &self,
        input: Input<B>,
    ) -> impl std::future::Future<Output = Output<Value, String>>;
    fn delete(&self, input: Input<B>) -> impl std::future::Future<Output = Output<String, String>>;
}

pub trait Queries<B> {
    fn query_fmt(&self) -> Result<Value, String>;
    fn query<T: QueryGeneric<T>>(&self, input: Value) -> Result<T, String>;
}
pub trait QueryGeneric<T> {
    fn new() -> T;
    fn new_with_input(input: QGEnum) -> T;
}
