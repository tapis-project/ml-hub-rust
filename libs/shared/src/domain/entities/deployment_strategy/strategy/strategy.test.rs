#[cfg(test)]
mod strategy_test {
    mod strategy {
        use crate::domain::entities::deployment_strategy::test_fixtures::ReconstitutedStrategyConfigBuilder;

        #[test]
        fn test_valid_strategy_config_reconsititution() {
            assert!(ReconstitutedStrategyConfigBuilder::new().build_reconstituted().is_ok())
        }
    }
    
    mod config {
        use crate::domain::entities::deployment_strategy::{
            strategy::StrategyConfigError,
            test_fixtures::ReconstitutedStrategyConfigBuilder
        };

        #[test]
        fn test_valid_config_reconsititution() {
            assert!(ReconstitutedStrategyConfigBuilder::new().build_reconstituted().is_ok())
        }

        #[test]
        fn test_reconstitution_fails_when_min_replicas_exceed_max() {
            // Min higher than max
            let err = ReconstitutedStrategyConfigBuilder::new()
                .with_max_replicas(1)
                .with_min_replicas(2)
                .build_reconstituted()
                .unwrap_err();

            assert!(matches!(err, StrategyConfigError::DataIntegrityError(..)))
        }

        #[test]
        fn test_reconstitution_fails_when_deployment_modalites_empty() {
            // Missing at least one deployment modality
            let err = ReconstitutedStrategyConfigBuilder::new()
                .with_modalities(vec![]) 
                .build_reconstituted()
                .unwrap_err();

            assert!(matches!(err, StrategyConfigError::DataIntegrityError(..)))
        }
    }
}

