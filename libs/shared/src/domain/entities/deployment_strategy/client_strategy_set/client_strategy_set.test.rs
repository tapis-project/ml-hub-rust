#[cfg(test)]
mod client_strategy_set_test {
    use crate::domain::entities::deployment_strategy::{
            client_strategy_set::{ClientStrategySet, ClientStrategySetError},
            test_fixtures::ReconstitutedClientStrategyBuilder,
        };
    
    #[test]
    fn test_valid_client_strategy_set_reconsititution() -> Result<(), ClientStrategySetError>{
        ClientStrategySet::reconstitute(
            platforms::Platform::TapisPods,
            None,
            vec![
                ReconstitutedClientStrategyBuilder::new()
                    .build_reconstituted()
                    .unwrap()
            ],
            None,
            None,
        )?;

        Ok(())
    }
}