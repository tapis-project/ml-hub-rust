use serde::Serialize;

#[derive(Serialize)]
pub struct ModelDto {
    pub model_id: String
}

#[cfg(test)]
mod tests {
    use super::ModelDto;
    #[test]
    fn test_model_dto () {
        let model_dto = ModelDto {
            model_id: String::from("test_id")
        };

        assert_eq!(model_dto.model_id, "test_id")
    }
}
