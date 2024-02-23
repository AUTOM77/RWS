use rand::Rng as _;
use rand::distributions::Alphanumeric;

pub fn take<F>(dsize: usize, filter: F) -> &'static str
where
    F: FnMut(&char) -> bool,
{
    let sample = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .filter(filter)
        .take(dsize)
        .collect::<String>();
    sample.leak()
}

// pub fn sample() -> (&'static str, &'static str, &'static str, &'static str) {
//     let _url = take(3, |c| c.is_ascii_digit()); 
//     let _ins = take(43, |_c| true);
//     let _key = take(43, |_c| true);
//     let _tk = take(134, |_c| true);

//     (_url.leak(), _ins.leak(), _key.leak() , _tk.leak())
// }