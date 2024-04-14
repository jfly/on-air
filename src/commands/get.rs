use crate::webcam::Webcam;
use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(Parser)]
/// Print the current status of attached webcams
pub struct Get {
    /// Output format
    #[arg(long, short, value_enum, default_value_t = Output::Text)]
    output: Output,
}

#[derive(ValueEnum, Clone)]
enum Output {
    Text,
    Json,
}

#[derive(Serialize)]
struct Device {
    path: String,
    name: String,
    streaming: bool,
}

impl Get {
    pub fn execute(&self) {
        let devices: Vec<Device> = Webcam::all()
            .into_iter()
            .map(|webcam| {
                let Webcam { path, name, .. } = &webcam;
                let path = path.to_string_lossy().to_string();

                let default_name = "???".to_string();
                let name = name.as_ref().unwrap_or(&default_name).to_string();

                Device {
                    path,
                    name,
                    streaming: webcam.is_streaming(),
                }
            })
            .collect();

        match self.output {
            Output::Text => {
                for Device {
                    path,
                    name,
                    streaming,
                } in devices
                {
                    println!("{path}");
                    println!("\tname: {name}");
                    println!("\tstreaming: {streaming}");
                }
            }
            Output::Json => println!("{}", serde_json::to_string_pretty(&devices).unwrap()),
        }
    }
}
