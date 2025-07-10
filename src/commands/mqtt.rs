use rumqttc::{Client, MqttOptions, QoS};
use serde::Serialize;
use std::{
    fs, panic,
    path::PathBuf,
    process,
    thread::{self, sleep},
    time::Duration,
};

use crate::webcam::Webcam;
use clap::Parser;

#[derive(Parser)]
/// Connect to an MQTT broker and publish the status to a topic.
pub struct Mqtt {
    /// The MQTT broker to connect to. Example: "mqtts://mqtt.snow.jflei.com".
    #[arg(long, short, env("ON_AIR_MQTT_BROKER"))]
    broker: String,

    #[arg(long, short, env("ON_AIR_MQTT_USERNAME"))]
    username: String,

    #[arg(long, short, env("ON_AIR_MQTT_PASSWORD"))]
    password: Option<String>,

    #[arg(long, env("ON_AIR_MQTT_PASSWORD_FILE"), conflicts_with = "password")]
    password_file: Option<PathBuf>,

    #[arg(long, short)]
    device_name: String,

    #[arg(long, default_value = "homeassistant")]
    discovery_prefix: String,

    #[arg(long, default_value_t = 5)]
    poll_seconds: u64,
}

/// https://www.home-assistant.io/integrations/mqtt/#discovery-payload
#[derive(Serialize)]
struct MqttDiscoveryPayload {
    name: Option<String>,
    device_class: BinarySensorDeviceClass,
    state_topic: String,
    unique_id: String,
    device: Device,
    expire_after: Option<u64>,
}

/// A *very* incomplete set of options from
/// https://www.home-assistant.io/integrations/binary_sensor/#device-class
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum BinarySensorDeviceClass {
    Motion,
}

#[derive(Serialize)]
struct Device {
    identifiers: Vec<String>,
    name: String,
}

impl Mqtt {
    pub fn execute(self) {
        let Self {
            broker,
            username,
            password,
            password_file,
            device_name,
            discovery_prefix,
            poll_seconds,
        } = self;
        let unique_id = format!("webcam-{device_name}");

        let mut broker = url::Url::parse(&broker).unwrap();
        // If the given broker url doesn't already have a client_id specified, add one.
        if !broker.query_pairs().any(|(k, _v)| k == "client_id") {
            broker
                .query_pairs_mut()
                .append_pair("client_id", &format!("on-air-{device_name}"));
        }

        let password =
            password.unwrap_or_else(|| fs::read_to_string(password_file.unwrap()).unwrap());

        let mut mqttoptions = MqttOptions::parse_url(broker).unwrap();
        mqttoptions.set_credentials(username, password);
        mqttoptions.set_keep_alive(Duration::from_secs(10));

        let (client, mut connection) = Client::new(mqttoptions, 1);

        // See https://www.home-assistant.io/integrations/mqtt/#motion-detection-binary-sensor for
        // the example this is based on.
        let configuration_topic = format!("{discovery_prefix}/binary_sensor/{unique_id}/config");
        let state_topic = format!("{discovery_prefix}/binary_sensor/{unique_id}/state");

        let discovery_payload = MqttDiscoveryPayload {
            name: None,
            device_class: BinarySensorDeviceClass::Motion,
            state_topic: state_topic.clone(),
            unique_id: unique_id.clone(),
            device: Device {
                identifiers: vec![unique_id.clone()],
                name: device_name,
            },
            expire_after: Some(2 * poll_seconds),
        };
        let discovery_payload_json = serde_json::to_string_pretty(&discovery_payload).unwrap();
        println!("Sending discovery message to {configuration_topic}");
        client
            .publish(
                configuration_topic,
                QoS::AtLeastOnce,
                // Retain this message. Note: a better option than retaining a discovery message is to implement support
                // for Home Assistant's "Birth and Last Will and Testament" messages. See
                // <https://www.home-assistant.io/integrations/mqtt/#birth-and-last-will-messages>
                // for details.
                true,
                discovery_payload_json,
            )
            .unwrap();

        // Force the whole process to exit if any threads panic.
        // Without this, the a sub-thread panics, it just stops running but the rest of the process
        // keeps running. Not very useful for us :p
        exit_on_panic();

        thread::spawn(move || {
            loop {
                let something_streaming = Webcam::all()
                    .into_iter()
                    .any(|webcam| webcam.is_streaming());
                let payload = if something_streaming { "ON" } else { "OFF" };
                client
                    .publish(state_topic.clone(), QoS::AtLeastOnce, false, payload)
                    .unwrap();

                sleep(Duration::from_secs(poll_seconds));
            }
        });

        for notification in connection.iter() {
            match &notification {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error talking to MQTT broker: {:?}", notification);

                    // If something's going wrong (most likely a network connectivity issue), sleep to
                    // slow things down so we're not pegging the CPU.
                    sleep(Duration::from_secs(15));
                }
            }
        }
    }
}

/// Copied from https://stackoverflow.com/a/36031130.
/// Useful to ensure a process exits if a thread panics.
fn exit_on_panic() {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Invoke the default handler and exit the process.
        orig_hook(panic_info);
        process::exit(1);
    }));
}
