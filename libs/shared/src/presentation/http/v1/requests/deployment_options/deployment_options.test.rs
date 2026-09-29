use super::ListDeploymentOptionsQuery;
use crate::application::inputs::deployment_option::ListDeploymentOptionsInput;

#[test]
fn converts_query_to_paginated_input() {
    let query = ListDeploymentOptionsQuery {
        limit: Some(25),
        cursor: Some("cursor".into()),
        include_count: Some(true),
    };

    let input = ListDeploymentOptionsInput::from(query);

    assert_eq!(input.limit(), 25);
    assert_eq!(input.cursor(), Some("cursor"));
    assert!(input.include_count());
}
