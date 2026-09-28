use crate::{EvaluatorError, Operation};

use super::invalid;
use crate::dtos::{Operation as OperationDto, OperationType};

pub(crate) fn convert_operation(value: OperationDto) -> Result<Operation, EvaluatorError> {
    match (value.operation, value.coalesce, value.multiply) {
        (OperationType::Coalesce, Some(value), None) => Ok(Operation::Coalesce {
            from: value.from,
            to: value.to,
        }),
        (OperationType::Multiply, None, Some(factors))
            if !factors.is_empty() && factors.iter().all(|factor| factor.is_finite()) =>
        {
            Ok(Operation::Multiply(factors))
        }
        (OperationType::Multiply, None, Some(_)) => {
            Err(invalid("multiply requires finite, nonempty factors"))
        }
        (OperationType::Coalesce, _, _) => Err(invalid(
            "coalesce discriminator requires only the coalesce field",
        )),
        (OperationType::Multiply, _, _) => Err(invalid(
            "multiply discriminator requires only the multiply field",
        )),
    }
}
