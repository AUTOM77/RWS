use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Cloudflare<'w> {
    _endpoint: &'w str,
    _ver: &'w str,
    _headers: &'w str,
}