use serde::Serialize;

#[derive(Serialize)]
pub struct InferenceDto {
    pub inference_id: String
}

#[cfg(test)]
mod tests {
    use super::InferenceDto;
    #[test]
    fn test_inference_dto () {
        let inference_dto = InferenceDto {
            inference_id: String::from("test_id")
        };

        assert_eq!(inference_dto.inference_id, "test_id")
    }
}
