use crate::wireguard::crypto;
use crate::random;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Deserialize, Default)]
pub struct Shot {
    endpoint: String,
    account: String,
    token: String,
    license: String,
    secret: String,
    refer: String,
}

impl fmt::Debug for Shot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TEST_SIMPLE")
            .field("endpoint", &self.endpoint)
            .field("account", &self.account)
            .field("token", &self.token)
            .field("license", &self.license)
            .field("private_key", &self.secret)
            .field("private_key_length", &self.secret.len())
            .field("refer", &self.refer)
            .finish()
    }
}

impl Shot {
    pub fn new() -> Self {
        Shot::default().generate()
    }

    pub fn generate(mut self) -> Self {
        self.refer = format!("NjM5NWZjMzItNzViOC00YzY4LWIzYWMtZWM4MDU3NWM4N2E3");
        self.secret = crypto::private_key();
        self
    }

    pub fn w_refer(mut self, _r: &str) -> Self {
        self.refer = _r.to_owned();
        self
    }

    pub fn from(_refer: &str) -> Self {
        Shot::new()
    }

    pub fn post(self) {
        let _api = "https://api.cloudflareclient.com";
        let _headers= vec![
            ("authority", "cloudflareclient.com"),
            ("host", "api.cloudflareclient.com"),
            ("User-Agent", "okhttp/4.12.1"),
            ("accept-encoding", "gzip"),
            ("accept-language", "en-US,en;q=0.9"),
            ("origin", "https://cloudflareclient.com"),
            ("referer", "https://warp.plus"),
            ("connection", "Keep-Alive"),
        ];

        let (d, v) = random::dev::sample();
        let (t, z) = random::tz::sample(); 

        let _i = format!("v0i{}", random::fake::take(3, |c| c.is_ascii_digit()));
        let _a = format!("v0a{}", random::fake::take(10, |c| c.is_ascii_digit()));
        let _ver = match d.chars().next() {
            Some('i') | Some('I') => _i,
            Some('a') | Some('A') => _a,
            _ => format!("v0")
        };

        let api = format!("{}/{}", _api, _ver);

        let payload = json!({
            "key": &self.secret,
            "tos": &t,
            "locale": &z,
            "model": &d,
            "type": &v,
            "referrer": &crypto::decode(&self.refer),
        });
    }

}
