use crate::warp::random;
use crate::wireguard::crypto;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct WClientBuilder<'w> {
    _id: Option<&'w str>,
    _sec: Option<&'w str>,
    _key: Option<&'w str>,
    _token: Option<&'w str>,
    _model: Option<&'w str>,
    _type: Option<&'w str>,
    _tos: Option<&'w str>,
    _locale: Option<&'w str>,
}

impl<'w> fmt::Debug for WClientBuilder<'w> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("install_id", &self._id.unwrap_or("default_id"))
            .field("private_key", &self._sec.unwrap_or("default_private_key"))
            .field("public_key", &self._key.unwrap_or("default_public_key"))
            .field("fcm_token", &format!("{}:APA91b{}",
                &self._id.unwrap_or("default_id"),
                &self._token.unwrap_or("default_token"))
            )
            .field("device_model", &self._model.unwrap_or("default_model"))
            .field("device_type", &self._type.unwrap_or("default_type"))
            .field("datetime", &self._tos.unwrap_or("default_tos"))
            .field("locale", &self._locale.unwrap_or("default_locale"))
            .finish()
    }
}

impl<'w> WClientBuilder<'w> {
    pub fn new() -> WClientBuilder<'w> {
        Self {
            _id: None,
            _sec: None,
            _key: None,
            _token: None,
            _model: None,
            _type: None,
            _tos: None,
            _locale: None,
        }
    }

    pub fn w_id(mut self, _id: &'w str) -> Self {
        self._id = Some(_id);
        self
    }
    pub fn w_sec(mut self, _sec: &'w str) -> Self {
        self._sec = Some(_sec);
        self
    }
    pub fn w_key(mut self, _key: &'w str) -> Self {
        self._key = Some(_key);
        self
    }
    pub fn w_token(mut self, _token: &'w str) -> Self {
        self._token = Some(_token);
        self
    }
    pub fn w_model(mut self, _model: &'w str) -> Self {
        self._model = Some(_model);
        self
    }
    pub fn w_type(mut self, _type: &'w str) -> Self {
        self._type = Some(_type);
        self
    }
    pub fn w_tos(mut self, _tos: &'w str) -> Self {
        self._tos = Some(_tos);
        self
    }
    pub fn w_locale(mut self, _locale: &'w str) -> Self {
        self._locale = Some(_locale);
        self
    }

    pub fn build(self) -> WClientBuilder<'w> {
        WClientBuilder {
            _id: Some(self._id.unwrap_or("default_id")),
            _sec: Some(self._sec.unwrap_or("default_private_key")),
            _key: Some(self._key.unwrap_or("default_public_key")),
            _token: Some(self._token.unwrap_or("default_token")),
            _model: Some(self._model.unwrap_or("default_model")),
            _type: Some(self._type.unwrap_or("default_type")),
            _tos: Some(self._tos.unwrap_or("default_tos")),
            _locale: Some(self._locale.unwrap_or("default_locale")),
        }
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
        WClientBuilder::new()
        .random_id()
        .wg_key()
        .random_token()
        .random_dev()
        .random_tz()
        .build()
    }
    pub fn from_id(_id:&'w str) -> Self {
        WClientBuilder::new()
        .w_id(_id)
        .wg_key()
        .random_token()
        .random_dev()
        .random_tz()
        .build()
    }
}