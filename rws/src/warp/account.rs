use serde::Deserialize;

use super::random;

#[derive(Deserialize, Debug)]
pub struct WClient {
    device_id: String,
    device_model: String,
    device_type: String
}

impl WClient {
    pub fn new(_id: &str, _model: &str, _type: &str) -> Self {
        Self{
            device_id: _id.to_owned(),
            device_model: _model.to_owned(),
            device_type: _type.to_owned(),
        }
    }

    pub fn get_header(&self) {
        println!{"{}", "header"};
    }

    pub fn generate_tz(&self) {
        random::tz::sample();
    }
    pub fn generate_fake(&self) {
        random::fake::sample();
    }
}