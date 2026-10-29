use actix_web::{http::StatusCode, test, web, App};

use super::*;

async fn status(health: Arc<ControllerHealth>) -> StatusCode {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(health))
            .service(healthcheck),
    )
    .await;

    let request = test::TestRequest::get().uri("/healthcheck").to_request();

    test::call_service(&app, request).await.status()
}

#[actix_web::test]
async fn remains_unavailable_until_every_control_plane_task_is_ready() {
    let health = Arc::new(ControllerHealth::default());

    assert_eq!(
        status(health.clone()).await,
        StatusCode::SERVICE_UNAVAILABLE
    );

    health.mark_event_listener_ready();

    assert_eq!(
        status(health.clone()).await,
        StatusCode::SERVICE_UNAVAILABLE
    );

    health.mark_reconciliation_loop_ready();

    assert_eq!(
        status(health.clone()).await,
        StatusCode::SERVICE_UNAVAILABLE
    );

    health.mark_observation_loop_ready();

    assert_eq!(
        status(health.clone()).await,
        StatusCode::SERVICE_UNAVAILABLE
    );

    health.mark_synchronization_loop_ready();

    assert_eq!(status(health).await, StatusCode::OK);
}
