use serde::Serialize;

#[derive(Serialize)]
pub struct TrainingDto {
    pub training_id: String
}

#[cfg(test)]
mod tests {
    use super::TrainingDto;
    #[test]
    fn test_training_dto () {
        let training_dto = TrainingDto {
            training_id: String::from("test_id")
        };

        assert_eq!(training_dto.training_id, "test_id")
    }
}
