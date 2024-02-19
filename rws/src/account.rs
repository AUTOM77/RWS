use crate::random;
use crate::wireguard::crypto;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Default)]
pub struct WClientBuilder<'w> {
    _id: &'w str,
    _sec: &'w str,
    _key: &'w str,
    _token: &'w str,
    _model: &'w str,
    _type: &'w str,
    _tos: &'w str,
    _locale: &'w str,
}

impl<'w> fmt::Debug for WClientBuilder<'w> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("install_id", &self._id)
            .field("private_key", &self._sec)
            .field("public_key", &self._key)
            .field("fcm_token", &format!("{}:APA91b{}",
                &self._id,
                &self._token)
            )
            .field("device_model", &self._model)
            .field("device_type", &self._type)
            .field("datetime", &self._tos)
            .field("locale", &self._locale)
            .finish()
    }
}

impl<'w> WClientBuilder<'w> {
    pub fn w_id(mut self, _id: &'w str) -> Self {
        self._id = _id;
        self
    }
    pub fn w_sec(mut self, _sec: &'w str) -> Self {
        self._sec = _sec;
        self
    }
    pub fn w_key(mut self, _key: &'w str) -> Self {
        self._key = _key;
        self
    }
    pub fn w_token(mut self, _token: &'w str) -> Self {
        self._token = _token;
        self
    }
    pub fn w_model(mut self, _model: &'w str) -> Self {
        self._model = _model; 
        self
    }
    pub fn w_type(mut self, _type: &'w str) -> Self {
        self._type = _type;
        self
    }
    pub fn w_tos(mut self, _tos: &'w str) -> Self {
        self._tos = _tos;
        self
    }
    pub fn w_locale(mut self, _locale: &'w str) -> Self {
        self._locale = _locale;
        self
    }

    pub fn build(self) -> WClientBuilder<'w> {
        self
    }

    pub fn random_id(self) -> Self {
        self.w_id(random::fake::take(22, |_c| true))
    }

    pub fn random_key(self) -> Self {
        self.w_key(random::fake::take(43, |_c| true))
    }

    pub fn wg_key(self) -> Self {
        let (s, k) = crypto::sample();
        self.w_sec(s)
            .w_key(k)
    }

    pub fn random_token(self) -> Self {
        self.w_token(random::fake::take(134, |_c| true))
    }
    
    pub fn random_dev(self) -> Self {
        let (t, z) = random::dev::sample(); 
        self.w_model(t)
            .w_type(z)
    }
    pub fn random_tz(self) -> Self {
        let (t, z) = random::tz::sample(); 
        self.w_tos(t)
            .w_locale(z)
    }
    pub fn random() -> Self {
        WClientBuilder::default()
        .random_id()
        .wg_key()
        .random_token()
        .random_dev()
        .random_tz()
        .build()
    }
    pub fn from_id(_id:&'w str) -> Self {
        WClientBuilder::default()
        .w_id(_id)
        .wg_key()
        .random_token()
        .random_dev()
        .random_tz()
        .build()
    }
}