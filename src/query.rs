use super::*;

use self::traits::{Input, QGEnum, Queries, QueryGeneric};
use nanoid::nanoid;
use serde_json::Value;

impl QueryGeneric<FindQuery> for FindQuery {
    fn new() -> Self {
        FindQuery::new(serde_json::to_value("{}").unwrap())
    }

    fn new_with_input(input: QGEnum) -> Self {
        match input {
            QGEnum::Val(val) => FindQuery::new(val),
            QGEnum::Vec(vec) => FindQuery::new(serde_json::to_value(vec).unwrap()),
            QGEnum::None => FindQuery::new(serde_json::to_value("{}").unwrap()),
        }
    }
}

impl QueryGeneric<Input<Value>> for Input<Value> {
    fn new() -> Self {
        Input::None
    }
    fn new_with_input(input: QGEnum) -> Self {
        match input {
            QGEnum::Val(val) => Input::Formatted(val),
            QGEnum::Vec(vec) => Input::Multiple(vec.iter().map(|v| v.clone()).collect()),
            QGEnum::None => Input::None,
        }
    }
}

impl Queries<Value> for QGEnum {
    fn query_fmt(&self) -> Result<Value, String> {
        match self {
            QGEnum::Val(val) => Ok(val.clone()),
            QGEnum::Vec(vec) => Ok(serde_json::to_value(vec).unwrap()),
            QGEnum::None => Ok(serde_json::to_value("{}").unwrap()),
        }
    }

    fn query<T: QueryGeneric<T>>(&self, input: Value) -> Result<T, String> {
        let res: T = T::new_with_input(QGEnum::Val(input));
        Ok(res)
    }
}

impl Queries<Value> for Value {
    fn query_fmt(&self) -> Result<Value, String> {
        Ok(self.clone())
    }

    fn query<T: QueryGeneric<T>>(&self, input: Value) -> Result<T, String> {
        let res: T = T::new_with_input(QGEnum::Val(input));
        Ok(res)
    }
}

pub fn new_id() -> String {
    nanoid!(32)
}

impl<B> Input<B>
where
    B: Queries<B> + serde::Serialize + serde::de::DeserializeOwned,
{
    pub fn matcher<F: QueryGeneric<F>>(&self) -> F {
        match self {
            Input::Raw(input) | Input::Formatted(input) => {
                let query = input.query_fmt();
                match query {
                    Ok(query) => F::new_with_input(QGEnum::Val(query)),
                    Err(e) => F::new_with_input(QGEnum::Val(e.into())),
                }
            }
            Input::Multiple(mul) => {
                let mut vec: Vec<Value> = Vec::new();
                for input in mul {
                    let query = input.query_fmt();
                    match query {
                        Ok(query) => vec.push(query),
                        Err(e) => vec.push(e.into()),
                    }
                }
                F::new_with_input(QGEnum::Vec(vec))
            }
            Input::None => F::new(),
        }
    }
}
