use serde::Deserialize;

use super::random;

#[derive(Deserialize, Debug)]
pub struct Payload {
    referrer: String,
}

impl Payload {
    pub fn new(_refer: &str) -> Self {
        Self{referrer: _refer.to_owned()}
    }

    pub fn generate(&self) {
        println!{"{}", self.referrer};
    }

    pub fn generate_tz(&self) {
        random::tz::sample();
    }
    pub fn generate_fake(&self) {
        random::fake::sample();
    }
}