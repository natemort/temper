#[macro_use]
extern crate serde_derive;


pub mod protocol {
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Response {
        Error {message: String},
        Success,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct TemperatureReading {
        pub temperature: f32,
        pub humidity: f32,
    }
}