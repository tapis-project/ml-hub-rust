use amqprs::channel::BasicConsumeArguments;
use log::info;
use model_deployment_controller::bootstrap::{
    bootstrap, model_deployment_controller_builder, BootstrapOutput,
};
use shared::infra::messaging::rabbitmq::queues::MODEL_DEPLOYMENT_RECONCILIATION_QUEUE;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    env_logger::init();

    let BootstrapOutput {
        messaging_context,
        client,
        db_name,
        site_context,
    } = bootstrap().await;

    let controller = model_deployment_controller_builder(
        &client,
        db_name,
        messaging_context.channel.clone(),
        site_context,
    )
    .await
    .unwrap_or_else(|err| panic!("Failed to initialize Model Deployment Controller: {}", err));

    let consumer_tag = Uuid::now_v7();

    let consume_arguments = BasicConsumeArguments::default()
        .queue(MODEL_DEPLOYMENT_RECONCILIATION_QUEUE.into())
        .consumer_tag(consumer_tag.to_string())
        .finish();

    let event_listener_channel = messaging_context.channel.clone();
    let event_listener_controller = controller.clone();

    let event_listener_task = tokio::spawn(async move {
        event_listener_channel
            .basic_consume(event_listener_controller, consume_arguments)
            .await
            .unwrap_or_else(|err| panic!("Failed to consume deployment events: {}", err));

        info!("Ready to receive messages...");

        std::future::pending::<()>().await;
    });

    let reconciliation_loop_controller = controller.clone();

    let reconciliation_loop_task = tokio::spawn(async move {
        reconciliation_loop_controller
            .run_reconciliation_loop()
            .await
    });

    let synchronization_loop_controller = controller.clone();

    let synchronization_loop_task = tokio::spawn(async move {
        synchronization_loop_controller
            .run_synchronization_loop()
            .await;
    });

    let observation_loop_task =
        tokio::spawn(async move { controller.run_observation_loop().await });

    tokio::select! {
        result = event_listener_task => {
            panic!("Event-listener task exited unexpectedly: {:?}", result)
        }
        result = reconciliation_loop_task => {
            panic!("Reconciliation-loop task exited unexpectedly: {:?}", result)
        }
        result = observation_loop_task => {
            panic!("Observation-loop task exited unexpectedly: {:?}", result)
        }
        result = synchronization_loop_task => {
            panic!("Synchronization-loop task exited unexpectedly: {:?}", result)
        }
        result = tokio::signal::ctrl_c() => {
            if let Err(err) = result {
                panic!("Failed to listen for the shutdown signal: {}", err)
            }
        }
    }
}
