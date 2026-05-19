use crate::domain::entities::operator as entities;
use crate::infra::operators::dtos;

impl From<dtos::Operator> for entities::Operator {
    fn from(value: dtos::Operator) -> Self {
        match value {
            dtos::Operator::In => entities::Operator::In,
            dtos::Operator::AllIn => entities::Operator::AllIn,
            dtos::Operator::AnyIn => entities::Operator::AnyIn,
            dtos::Operator::NoneIn => entities::Operator::NoneIn,
            dtos::Operator::NotIn => entities::Operator::NotIn,
            dtos::Operator::Contains => entities::Operator::Contains,
            dtos::Operator::Eq => entities::Operator::Eq,
            dtos::Operator::Neq => entities::Operator::Neq,
            dtos::Operator::Gt => entities::Operator::Gt,
            dtos::Operator::Gte => entities::Operator::Gte,
            dtos::Operator::Lt => entities::Operator::Lt,
            dtos::Operator::Lte => entities::Operator::Lte,
        }
    }
}