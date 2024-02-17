use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WarpAPI {
    _endpoint: String,
    _ver: String,
}