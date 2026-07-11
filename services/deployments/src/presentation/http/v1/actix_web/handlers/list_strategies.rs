use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use shared::domain::entities::deployment_strategy::strategy::Strategy;
use crate::presentation::http::v1::responses;
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
        (status=200, description="A list of all deployment strategies", body=contracts::responses::ListDeploymentStrategiesResponse),
        (status=400, description="Bad Request", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not Found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Internal Server Error", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("deployments-api/strategies")]
async fn list_strategies(data: web::Data<AppState>,) -> impl Responder {
    let mut strats: Vec<Strategy> = vec![];
    for set in data.client_strategy_sets.iter() {
        strats.extend(set.strategies().clone())
    }

    let mut resp: Vec<Value> = Vec::with_capacity(strats.len());
    for strat in strats.into_iter() {
        match to_value(responses::Strategy::from(strat)) {
            Ok(v) => { resp.push(v); },
            Err(err) => return build_error_response(500, format!("Error serializing strategies: {}", err.to_string()))
        };
    };


    build_success_response(Some(Value::Array(resp)), Some("Success".into()), None)
}