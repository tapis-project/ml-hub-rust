use super::*;

#[test]
fn list_input_defaults_and_caps_limit() {
    let query = ListHpcClustersQuery {
        limit: None,
        cursor: None,
        include_count: None,
    };

    let input = query.into_input(DataCenter::Tacc);

    assert_eq!(input.limit(), ListHpcClustersInput::DEFAULT_LIMIT);
    assert!(input.cursor().is_none());
    assert!(!input.include_count());

    let query = ListHpcClustersQuery {
        limit: Some(101),
        cursor: Some("next".into()),
        include_count: Some(true),
    };

    let input = query.into_input(DataCenter::Tacc);

    assert_eq!(input.limit(), ListHpcClustersInput::MAX_LIMIT);
    assert_eq!(input.cursor(), Some("next"));
    assert!(input.include_count());
}

#[test]
fn data_center_path_is_case_sensitive() {
    assert!(matches!(DataCenter::try_from("Tacc"), Ok(DataCenter::Tacc)));
    assert!(DataCenter::try_from("tacc").is_err());
}
