use rand_core::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};
use base64::{engine::general_purpose::STANDARD as BASE64,  Engine as _};

pub fn sample() -> (&'static str, &'static str){
    let _secret = StaticSecret::random_from_rng(OsRng);
    let _key = PublicKey::from(&_secret);
    let _pvt = BASE64.encode(_secret);
    let _pub = BASE64.encode(_key);
    (_pvt.leak(), _pub.leak())
}

pub fn public_key() -> String{
    let _secret = StaticSecret::random_from_rng(OsRng);
    let _key = PublicKey::from(&_secret);
    BASE64.encode(_key)
}

pub fn private_key() -> String{
    let _secret = StaticSecret::random_from_rng(OsRng);
    BASE64.encode(_secret)
}