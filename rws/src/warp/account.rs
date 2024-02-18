pub struct WClientBuilder<'w> {
    _id: Option<&'w str>,
    _model: Option<&'w str>,
    _type: Option<&'w str>,
    _key: Option<&'w str>,
}

impl<'w> WClientBuilder<'w> {
    pub fn new() -> WClientBuilder<'w> {
        Self {
            _id: None,
            _model: None,
            _type: None,
            _key: None,
        }
    }

    pub fn w_id(mut self, _id: &'w str) -> Self {
        self._id = Some(_id);
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

    pub fn w_key(mut self, _key: &'w str) -> Self {
        self._key = Some(_key);
        self
    }

    pub fn build(self) -> WClientBuilder<'w> {
        WClientBuilder {
            _id: Some(self._id.unwrap_or("default_id")),
            _model: Some(self._model.unwrap_or("default_model")),
            _type: Some(self._type.unwrap_or("default_type")),
            _key: Some(self._key.unwrap_or("default_key")),
        }
    }
}