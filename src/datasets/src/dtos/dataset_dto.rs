use serde::Serialize;

#[derive(Serialize)]
pub struct DatasetDto {
    pub dataset_id: String
}

#[cfg(test)]
mod tests {
    use super::DatasetDto;
    #[test]
    fn test_dataset_dto () {
        let dataset_dto = DatasetDto {
            dataset_id: String::from("test_id")
        };

        assert_eq!(dataset_dto.dataset_id, "test_id")
    }
}
