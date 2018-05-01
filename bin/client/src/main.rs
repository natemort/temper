#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate dht22_pi;
extern crate reqwest;
extern crate temper_common;

use dht22_pi::{read, Reading};
use std::fs::File;
use std::{thread, time};
use reqwest::Client;
use temper_common::protocol::TemperatureReading;

#[derive(Serialize, Deserialize, Debug)]
struct ClientConfig {
    hostname: String,
    device_id: u64,
    pin: u8,
    frequency: u64,
}

fn main() {
    let config = read_config();
    loop {
        let result = read(config.pin);
        match result {
            Ok(reading) => send_reading(&config, reading),
            Err(e) => println!("Failed to read temperature: {:?}", e)
        }
        thread::sleep(time::Duration::from_secs(config.frequency));
    }
}

fn read_config() -> ClientConfig {
    let file = File::open("client.yml").expect("Unable to find client.yml");
    serde_yaml::from_reader(file).expect("Failed to parse client.yml")
}

fn send_reading(config: &ClientConfig, reading: Reading) {
    let body = TemperatureReading {temperature: reading.temperature, humidity: reading.humidity};
    let result = Client::new().put(&config.hostname)
        .json(&body)
        .send();
    match result {
        Ok(result) => println!("Successfully sent reading. Received: {:?}", result),
        Err(e) => println!("Failed to send reading: {:?}", e),
    }
}
