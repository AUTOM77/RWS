use rand::Rng as _;

pub fn sample()-> &'static str{
    let tp_pool = vec![
        "iOS",
        "iPadOS",
        "macOS",
        "Android",
        "Windows",
        "Ubuntu",
        "Debian",
        "Fedora",
        "RedHat",
        "CentOS",
        "SUSE",
        "Arch",
        "Manjaro",
        "Mint",
        "Gentoo",
        "Slackware",
        "Mageia",
        "elementary",
        "Zorin",
        "Solus",
        "Bodhi",
        "antiX",
        "Deepin",
        "Puppy",
        "Void",
        "ChromeOS",
    ];

    let mut rng = rand::thread_rng();
    let _idx = rng.gen_range(0..tp_pool.len());
    
    tp_pool[_idx]
}