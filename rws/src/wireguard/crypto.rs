use rand_core::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};
use base64::{engine::general_purpose::STANDARD as BASE64,  Engine as _};

pub fn sample() -> &'static str{
    let _pvt = StaticSecret::random_from_rng(OsRng);
    let _pub = PublicKey::from(&_pvt);
    let _b64 = BASE64.encode(_pub);
    _b64.leak()
}