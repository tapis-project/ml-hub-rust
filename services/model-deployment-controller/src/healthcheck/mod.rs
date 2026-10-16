use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

const HEALTHCHECK_HOST: &str = "0.0.0.0";
const HEALTHCHECK_PORT: u16 = 8000;

#[derive(Default)]
pub struct ControllerHealth {
    event_listener: AtomicBool,
    reconciliation_loop: AtomicBool,
    observation_loop: AtomicBool,
    synchronization_loop: AtomicBool,
}

impl ControllerHealth {
    pub fn mark_event_listener_ready(&self) {
        self.event_listener.store(true, Ordering::Release);
    }

    pub fn mark_reconciliation_loop_ready(&self) {
        self.reconciliation_loop.store(true, Ordering::Release);
    }

    pub fn mark_observation_loop_ready(&self) {
        self.observation_loop.store(true, Ordering::Release);
    }

    pub fn mark_synchronization_loop_ready(&self) {
        self.synchronization_loop.store(true, Ordering::Release);
    }

    fn snapshot(&self) -> ControllerHealthResponse {
        let event_listener = self.event_listener.load(Ordering::Acquire);
        let reconciliation_loop = self.reconciliation_loop.load(Ordering::Acquire);
        let observation_loop = self.observation_loop.load(Ordering::Acquire);
        let synchronization_loop = self.synchronization_loop.load(Ordering::Acquire);
        let healthy =
            event_listener && reconciliation_loop && observation_loop && synchronization_loop;

        ControllerHealthResponse {
            status: if healthy { "healthy" } else { "starting" },
            event_listener,
            reconciliation_loop,
            observation_loop,
            synchronization_loop,
        }
    }
}

#[derive(Debug, Serialize)]
struct ControllerHealthResponse {
    status: &'static str,
    event_listener: bool,
    reconciliation_loop: bool,
    observation_loop: bool,
    synchronization_loop: bool,
}

#[get("/healthcheck")]
async fn healthcheck(health: web::Data<Arc<ControllerHealth>>) -> impl Responder {
    let response = health.snapshot();

    if response.status == "healthy" {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

pub fn healthcheck_server(health: Arc<ControllerHealth>) -> io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(health.clone()))
            .service(healthcheck)
    })
    .bind((HEALTHCHECK_HOST, HEALTHCHECK_PORT))?
    .disable_signals()
    .run();

    Ok(server)
}

#[cfg(test)]
#[path = "healthcheck.test.rs"]
mod healthcheck_test;
