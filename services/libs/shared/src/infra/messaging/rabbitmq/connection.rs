use amqprs::{
    channel::Channel,
    connection::{
        Connection, 
        OpenConnectionArguments,
    },
};
use crate::infra::messaging::rabbitmq::errors::BrokerError;

pub async fn open_channel(host: String, port: u16, username: String, password: String) -> Result<(Connection, Channel), BrokerError> {
    let args = OpenConnectionArguments::new(
        host.as_str(),
        port,
        username.as_str(),
        password.as_str(),
    ); 

    let conn = match Connection::open(&args).await {
        Ok(conn) => conn,
        Err(err) => return Err(BrokerError::Connection(err.to_string()))
    };

    let channel = conn.open_channel(None)
        .await
        .map_err(|err| BrokerError::Channel(err.to_string()))?;

    Ok((conn, channel))
}

// async fn connect_to_broker(args: &OpenConnectionArguments, max_connection_attempts: i8) -> Connection {
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

