use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum FilterOperation {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    In,
    Nin,
    Pattern,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Filter {
    pub field: String,
    pub operation: FilterOperation,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListAll {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub fields: Option<Vec<String>>,
    pub filters: Option<Vec<Filter>>,
    pub sort_by: Option<String>,
    pub order_by: Option<Order>,
}