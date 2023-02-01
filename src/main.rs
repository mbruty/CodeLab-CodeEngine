use std::borrow::Borrow;
use std::{env, fs};
use amiquip::{
    AmqpProperties, Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish,
    QueueDeclareOptions, Result,
};
use std::time::Instant;
use crate::languages::Languages;
use crate::rpc_handler::handle;
use crate::strategies::get_strategy_for;
mod strategies;
mod utils;
mod languages;
mod rpc_handler;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No language provided!");
    }

    let lang: Languages = match args[1].to_lowercase().as_str() {
        "--language=dotnet" => Languages::Dotnet,
        "--language=javascript" => Languages::JavaScript,
        "--language=typescript" => Languages::TypeScript,
        "--language=bun" => Languages::BunJavaScript,
        "--language=python" => Languages::Python,
        _ => panic!("No language provided!")
    };

    let ctx = get_strategy_for(lang);
    // Create the code folder
    fs::create_dir_all("./unsafe").expect("TODO: panic message");

    // Open connection.
    let mut connection = Connection::insecure_open("amqp://user:2ANj6hH47G1ViFCuX23jLIBpX@213.171.211.224:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Tell rabbitmq to only send us 1 message at a time.
    // Without this, each consumer in the cluster will auto acknowledge the message
    // So if we then scale up the cluster size, the new nodes won't get anything that was in the queue
    // before they were created.
    // Doing this fixes that issue
    channel.qos(0, 1, false)?;
    // Get a handle to the default direct exchange.
    let exchange = Exchange::direct(&channel);

    // Declare the queue that will receive RPC requests.
    let queue = channel.queue_declare(ctx.get_queue_name(), QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    ctx.print_greeting();
    for (_i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                println!("[.] Received a request");
                let body = String::from_utf8_lossy(&delivery.body);

                let (reply_to, corr_id) = match (
                    delivery.properties.reply_to(),
                    delivery.properties.correlation_id(),
                ) {
                    (Some(r), Some(c)) => (r.clone(), c.clone()),
                    _ => {
                        println!("received delivery without reply_to or correlation_id");
                        consumer.ack(delivery)?;
                        continue;
                    }
                };
                let now = Instant::now();
                let response = handle(body.borrow(), lang);
                let elapsed = now.elapsed();
                println!("[.] Replied in: {:.2?}", elapsed);
                exchange.publish(Publish::with_properties(
                    response.as_bytes(),
                    reply_to,
                    AmqpProperties::default().with_correlation_id(corr_id),
                ))?;
                consumer.ack(delivery)?;
                break;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    connection.close()
}