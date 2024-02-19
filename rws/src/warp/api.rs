use serde::Deserialize;

#[derive(Deserialize)]
pub struct Cloudflare<'w> {
    _endpoint: Option<&'w str>,
    _ver: Option<&'w str>,
    _headers: Option<&'w str>
}