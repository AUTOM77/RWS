use rand::Rng as _;
use rand::distributions::Alphanumeric;

fn take<F>(dsize: usize, filter: F) -> String
where
    F: FnMut(&char) -> bool,
{
    let sample = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .filter(filter)
        .take(dsize)
        .collect();
    sample
}

pub fn sample() -> (String, String, String ) {
    let _url_r = take(3, |c| c.is_ascii_digit());
    let _install_r = take(43, |_c| true);
    let _token_r = take(134, |_c| true);

    (_url_r, _install_r, _token_r)
}