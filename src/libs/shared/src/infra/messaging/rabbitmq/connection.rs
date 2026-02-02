use crate::infra::messaging::rabbitmq::exchanges::{delcare_exchanges, ARTIFACT_INGESTION_EXCHANGE, ARTIFACT_PUBLICATION_EXCHANGE, MODEL_DEPLOYMENT_EXCHANGE};
use crate::application::ports::commands::CommandPublisherError;
use amqprs::{
    channel::Channel,
    connection::{
        Connection, 
        OpenConnectionArguments,
    }
};

pub async fn amqp_connection_builder(host: &String, port: &String, username: &String, password: &String) -> Result<Channel, CommandPublisherError> {
    let connection_args = OpenConnectionArguments::new(
        host.as_str(),
        port.parse::<u16>().unwrap_or(5672),
        username.as_str(),
        password.as_str()
    );

    let conn = match Connection::open(&connection_args).await {
        Ok(conn) => conn,
        Err(err) => return Err(CommandPublisherError::AmqpError(err.to_string()))
    };

    let channel = conn.open_channel(None).await.expect("Open channel failed");

    delcare_exchanges(
        &channel, 
        vec![
            (ARTIFACT_INGESTION_EXCHANGE, "topic"),
            (ARTIFACT_PUBLICATION_EXCHANGE, "topic"),
            (MODEL_DEPLOYMENT_EXCHANGE, "topic"),
        ]
    ).await?;

    Ok(channel)
}

// async fn connect_to_channel(args: &OpenConnectionArguments, max_connection_attempts: i8) -> Connection {
//     println!("Attempting to connect to broker");
    
//     let mut connection_attempts: i8 = 0;
//     while connection_attempts <= max_connection_attempts {
//         // Attempt to connect. Out of all the possible errors, we only want to retry
//         // the connection on the two IO errors below
        
//         // Open connection
//         let maybe_connection = Connection::open(args)
//             .await;

//         match maybe_connection {
//             Ok(conn) => return conn, // Return the successful connection
//             Err(err) => {
//                 connection_attempts += 1;
//                 match err {
//                     Error::NetworkError(_) => {
//                         println!("Failed to connect to message broker: Attempt {} of {}", connection_attempts, max_connection_attempts);
//                         connection_attempts += 1;
//                         continue;
//                     },
//                     other => panic!("Failed to connect to message broker: {}", other.to_string())
//                 };
//             }
//         }
//     }

//     panic!("Failed to connect to message broker. Max attempts reached: {}", max_connection_attempts);
// }

// async fn ack(channel: &Channel, deliver: &Deliver, multiple: Option<bool>) {
//     let args = BasicAckArguments {
//         delivery_tag: deliver.delivery_tag(),
//         multiple: multiple.unwrap_or(false)
//     };

//     if let Err(err) = channel.basic_ack(args).await {
//         eprintln!("CRITICAL: Failed to ack message: {}", err.to_string());
//         panic!("Cannot ack. Shutting down to avoid inconsistent state.");
//     }
// }

// async fn nack(channel: &Channel, deliver: &Deliver, requeue: Option<bool>, multiple: Option<bool>) {
//     let args = BasicNackArguments {
//         delivery_tag: deliver.delivery_tag(),
//         requeue: requeue.unwrap_or(false),
//         multiple: multiple.unwrap_or(false)
//     };

//     if let Err(err) = channel.basic_nack(args).await {
//         eprintln!("CRITICAL: Failed to nack message: {}", err.to_string());
//         panic!("Cannot nack. Shutting down to avoid inconsistent state.");
//     }
// }