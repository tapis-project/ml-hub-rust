use crate::presentation::http::v1::requests::deployment::Argument;
use crate::application::inputs::deployment as inputs;

impl From<Argument> for inputs::Argument {
    fn from(value: Argument) -> Self {
        Self {
            parameter_name: value.parameter_name,
            value: value.value,
        }
    }
}