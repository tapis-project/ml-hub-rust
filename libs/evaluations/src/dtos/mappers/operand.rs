use crate::{EvaluatorError, FieldPath, LiteralOperand, Operand, ParameterOperand};

use super::{convert_operation, invalid};
use crate::dtos::{Accessor as AccessorDto, Operand as OperandDto, OperandType, ValueField};

pub(crate) fn convert_operand(
    value: OperandDto,
    parameters: &[String],
) -> Result<Operand, EvaluatorError> {
    let operations = value
        .operations
        .into_iter()
        .map(convert_operation)
        .collect::<Result<Vec<_>, _>>()?;

    match value.operand_type {
        OperandType::Parameter => {
            if !matches!(value.value, ValueField::Missing) {
                return Err(invalid("parameter operands cannot define value"));
            }

            let parameter = value
                .parameter
                .filter(|parameter| !parameter.trim().is_empty())
                .ok_or_else(|| invalid("parameter operands require a nonempty parameter"))?;

            if !parameters.contains(&parameter) {
                return Err(invalid(format!(
                    "operand references undeclared parameter: {parameter}"
                )));
            }

            let accessor = value.accessor.map(convert_accessor).transpose()?;

            Ok(Operand::Parameter(ParameterOperand::new(
                parameter, accessor, operations,
            )))
        }
        OperandType::Literal => {
            if value.parameter.is_some() || value.accessor.is_some() {
                return Err(invalid(
                    "literal operands cannot define parameter or accessor",
                ));
            }

            let ValueField::Present(value) = value.value else {
                return Err(invalid("literal operands require value"));
            };

            Ok(Operand::Literal(LiteralOperand::new(value, operations)))
        }
    }
}

fn convert_accessor(value: AccessorDto) -> Result<FieldPath, EvaluatorError> {
    if value.field_path.is_empty() || value.field_path.iter().any(|part| part.trim().is_empty()) {
        return Err(invalid("accessor field_path must contain nonempty parts"));
    }

    Ok(FieldPath::new(value.field_path))
}
