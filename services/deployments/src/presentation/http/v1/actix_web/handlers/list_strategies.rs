use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::presentation::http::v1::responses::ClientStrategySet;
use crate::bootstrap::state::AppState;
use crate::presentation::http::v1::contracts;
use actix_web::{
    get,
    web,
    Responder
};
use serde_json::{Value, to_value};

#[utoipa::path(
    get,
    path="/deployments-api/strategies",
    tag="Strategies",
    description="Lists all available deployment strategies for all platforms",
    responses(
        (status=200, description="Listed platforms", body=contracts::responses::ListDeploymentStrategiesResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("deployments-api/strategies")]
async fn list_strategies(data: web::Data<AppState>,) -> impl Responder {
    let mut strats: Vec<Value> = vec![];

    for set in data.client_strategy_sets.iter() {
        match to_value(ClientStrategySet::from(set.clone())) {
            Ok(v) => { strats.push(v); },
            Err(err) => return build_error_response(500, format!("Error serializing client strategies: {}", err.to_string()))
        };
    }

    build_success_response(Some(Value::Array(strats)), Some("Success".into()), None)
}