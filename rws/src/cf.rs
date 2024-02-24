use crate::wireguard::crypto;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Default)]
pub struct Shot {
    account: String,
    token: String,
    license: String,
    secret: String,
    refer: String,
}

impl fmt::Debug for Shot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TEST_SIMPLE")
            .field("account", &self.account)
            .field("token", &self.token)
            .field("license", &self.license)
            .field("private_key", &self.secret)
            .field("private_key_length", &self.secret.len())
            .finish()
    }
}

impl Shot {
    pub fn new() -> Self {
        Shot::default().generate()
    }

    pub fn from(_refer: &str) -> Self {
        Shot::new().set_refer(_refer)
    }

    pub fn set_refer(mut self, _refer: &str) -> Self {
        self.refer = _refer.to_owned();
        self
    }

    pub fn generate(mut self) -> Self {
        self.secret = crypto::private_key();
        self
    }
}
