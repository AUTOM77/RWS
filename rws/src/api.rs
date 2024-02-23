use crate::random::fake::take;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Default)]
pub struct CloudflareBuilder<'w> {
    _endpoint: &'w str,
    _ver: &'w str,
    _headers: Vec<(&'w str, &'w str)>,
}

// 'https://api.cloudflareclient.com/v0/reg'
// 'https://api.cloudflareclient.com/v0a{digitString(3)}/reg'
// 'https://api.cloudflareclient.com/v0i{digitString(10)}/reg'
impl<'w> fmt::Debug for CloudflareBuilder<'w> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Cloudflare API")
            .field("URL", &format!("{}/{}/reg",
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
    pub fn w_headers(mut self, _headers: Vec<(&'w str, &'w str)>) -> Self {
        self._headers = _headers;
        self
    }

    pub fn build(self) -> CloudflareBuilder<'w> {
        self
    }

    pub fn random_endpoint(self) -> Self {
        self.w_endpoint("https://api.cloudflareclient.com")
    }

    pub fn random_ver(self, dev: &'w str) -> Self {
        let _i = format!("v0i{}", take(3, |c| c.is_ascii_digit()));
        let _a = format!("v0a{}", take(10, |c| c.is_ascii_digit()));
        let _ver = match dev.chars().next() {
            Some('i') | Some('I') => _i,
            Some('a') | Some('A') => _a,
            _ => format!("v0")
        };
        self.w_ver(_ver.leak())
    }

    pub fn random_headers(self) -> Self {
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
        self.w_headers(_headers)
    }

    pub fn get_api(self) -> &'w str{
        format!("{}/{}/reg",
            &self._endpoint,
            &self._ver).leak()
    }

    pub fn random() -> Self {
        CloudflareBuilder::default()
        .random_endpoint()
        .random_ver("")
        .random_headers()
        .build()
    }

    pub fn from_dev(dev: &'w str) -> Self {
        CloudflareBuilder::default()
        .random_endpoint()
        .random_ver(dev)
        .random_headers()
        .build()
    }
}