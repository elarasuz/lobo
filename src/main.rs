mod cli;
mod settings;

#[macro_use]
extern crate log;

use clap::Parser;
use rumqttc::{AsyncClient, Incoming, MqttOptions, QoS};
use settings::{Settings, MQTT};
// use config::Config;
use std::time::Duration;
use std::str;

use crate::cli::Commands;

async fn mqtt_stream_topic(cfg: &MQTT) {
    let mut mqttoptions = MqttOptions::new("rumqtt-async", &cfg.host, cfg.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    info!(
        "[{}] starting listeners on [{}:{}]",
        cfg.topic, cfg.host, cfg.port
    );

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(cfg.topic.clone(), QoS::AtMostOnce).await.unwrap();

    while let Ok(notification) = eventloop.poll().await {
        // println!("Received = {:?}", notification);
        match notification {
            rumqttc::Event::Incoming(Incoming::Publish(msg)) => {
                let s = match str::from_utf8(&msg.payload) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };            
                // let msg: Packet = notification as Packet;
                info!("[{}] {}", msg.topic, s);
            }
            _ => {
                // println!("NOTIFICATION {:?}", notification);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // RUST_LOG=debug
    env_logger::init();
    let settings = Settings::new().unwrap();
    let cli = cli::Cli::parse();
    info!("{:?}", settings);
    let cfg = settings.mqtt;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Test { config: _ } => {
            mqtt_stream_topic(&cfg).await;
        }
    }
}
