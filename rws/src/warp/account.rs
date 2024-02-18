use super::random;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WClientBuilder<'w> {
    _id: Option<&'w str>,
    _key: Option<&'w str>,
    _model: Option<&'w str>,
    _type: Option<&'w str>,
    _tos: Option<&'w str>,
    _locale: Option<&'w str>,
}

impl<'w> WClientBuilder<'w> {
    pub fn new() -> WClientBuilder<'w> {
        Self {
            _id: None,
            _key: None,
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
    pub fn w_key(mut self, _key: &'w str) -> Self {
        self._key = Some(_key);
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
            _key: Some(self._key.unwrap_or("default_key")),
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
}