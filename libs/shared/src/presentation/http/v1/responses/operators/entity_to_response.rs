use crate::domain::entities::operator as entities;
use crate::presentation::http::v1::responses::operators as dtos;

impl From<entities::Operator> for dtos::Operator {
    fn from(value: entities::Operator) -> Self {
        match value {
            entities::Operator::In => dtos::Operator::In,
            entities::Operator::AllIn => dtos::Operator::AllIn,
            entities::Operator::AnyIn => dtos::Operator::AnyIn,
            entities::Operator::NoneIn => dtos::Operator::NoneIn,
            entities::Operator::NotIn => dtos::Operator::NotIn,
            entities::Operator::Contains => dtos::Operator::Contains,
            entities::Operator::Eq => dtos::Operator::Eq,
            entities::Operator::Neq => dtos::Operator::Neq,
            entities::Operator::Gt => dtos::Operator::Gt,
            entities::Operator::Gte => dtos::Operator::Gte,
            entities::Operator::Lt => dtos::Operator::Lt,
            entities::Operator::Lte => dtos::Operator::Lte,
        }
    }
}