use endpoint_router::{bootstrap::bootstrap, processor::EndpointExternalProcessor};
use envoy_types::pb::envoy::service::ext_proc::v3::external_processor_server::ExternalProcessorServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let application = bootstrap().await?;
    let listen_address = application.listen_address.parse()?;
    let processor = EndpointExternalProcessor::new(
        application.authentication,
        application.routing_service,
        application.route_cache,
    );

    Server::builder()
        .add_service(ExternalProcessorServer::new(processor))
        .serve(listen_address)
        .await?;

    Ok(())
}
