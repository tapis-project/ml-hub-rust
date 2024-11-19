use serde::Serialize;

#[derive(Serialize)]
pub struct Model {
    pub model_id: String
}

#[cfg(test)]
mod tests {
    use super::Model;
    #[test]
    fn test_model () {
        let model = Model {
            model_id: String::from("test_id")
        };

        assert_eq!(model.model_id, "test_id")
    }
}
