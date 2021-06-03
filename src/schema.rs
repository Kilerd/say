use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::validator::Validator;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    root: DataType,
    validators: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DataType {
    Dict(Box<DictType>),
    List(Box<ListType>),
    String(Box<StringType>),
    Literal(Box<LiteralType>),
    Boolean(Box<BooleanType>),
    Number(Box<NumberType>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictType {
    #[serde(default = "bool::default")]
    pub optional: bool,
    #[serde(default = "bool::default")]
    pub nullable: bool,
    pub fields: HashMap<String, DataType>,
    pub any_fields: Option<HashMap<String, DataType>>,
    pub others: Option<DataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListType {
    #[serde(default = "bool::default")]
    pub optional: bool,
    #[serde(default = "bool::default")]
    pub nullable: bool,
    element_type: DataType,
    limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiteralType {
    #[serde(default = "bool::default")]
    pub optional: bool,
    #[serde(default = "bool::default")]
    pub nullable: bool,
    pub candidate: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringType {
    #[serde(default = "bool::default")]
    pub optional: bool,
    #[serde(default = "bool::default")]
    pub nullable: bool,
    length: Option<u64>,
    regex: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanType {
    #[serde(default = "bool::default")]
    pub optional: bool,
    #[serde(default = "bool::default")]
    pub nullable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberType {
    #[serde(default = "bool::default")]
    pub optional: bool,
    #[serde(default = "bool::default")]
    pub nullable: bool,
    max: Option<i32>,
    min: Option<i32>,
}
