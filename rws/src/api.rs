use crate::random;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Default)]
pub struct CloudflareBuilder<'w> {
    _endpoint: &'w str,
    _ver: &'w str,
    _headers: &'w str,
}

// 'https://api.cloudflareclient.com/v0a{digitString(3)}/reg'
// 'https://api.cloudflareclient.com/v0i{digitString(10)}/reg'
impl<'w> fmt::Debug for CloudflareBuilder<'w> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Cloudflare API")
            .field("URL", &format!("{}/v0i{}/reg",
                &self._endpoint,
                &self._ver)
            )
            .field("HTTP_Headers", &self._headers)
            .finish()
    }
}

impl<'w> CloudflareBuilder<'w> {
    pub fn w_endpoint(mut self, _endpoint: &'w str) -> Self {
        self._endpoint = _endpoint;
        self
    }
    pub fn w_ver(mut self, _ver: &'w str) -> Self {
        self._ver = _ver;
        self
    }
    pub fn w_headers(mut self, _headers: &'w str) -> Self {
        self._headers = _headers;
        self
    }

    pub fn build(self) -> CloudflareBuilder<'w> {
        self
    }

    pub fn random_ver(self) -> Self {
        self.w_ver(random::fake::take(10, |c| c.is_ascii_digit()))
    }

    pub fn random_headers(self) -> Self {
        self.w_headers(random::fake::take(43, |_c| true))
    }

    pub fn random() -> Self {
        CloudflareBuilder::default()
        .random_ver()
        .random_headers()
        .build()
    }
}