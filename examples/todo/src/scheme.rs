use query_macro::QueryMacro;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use EasyCouch::traits::{QGEnum, Queries, QueryGeneric};

#[derive(Debug, Clone, Serialize, Deserialize, QueryMacro)]
pub struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _rev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<bool>,
}
