use std::borrow::Cow;
use rand::Rng as _;
use chrono::Local;
use chrono_tz::Tz;

fn process<'a>(_pair: (&'a str, &'a str)) -> (Cow<'a, str>, Cow<'a, str>){
    let (locale, timezone) = _pair;
    let utc = Local::now();
    match timezone.parse::<Tz>() {
        Ok(tz) => {
            let tz_time = utc.with_timezone(&tz).format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string();
            (std::borrow::Cow::Borrowed(locale), tz_time.into())
        }
        Err(_) => {
            let tz_time = utc.format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string();
            (std::borrow::Cow::Borrowed(locale), tz_time.into())
        }
    }
}

pub fn sample()-> (Cow<'static, str>, Cow<'static, str>){
    let tz_pool = vec![
        ("en-US","America/New_York"),
        ("en-GB","Europe/London"),
        ("fr-FR","Europe/Paris"),
        ("de-DE","Europe/Berlin"),
        ("ja-JP","Asia/Tokyo"),
        ("es-ES","Europe/Madrid"),
        ("pt-BR","America/Sao_Paulo"),
        ("zh-CN","Asia/Shanghai"),
        ("ru-RU","Europe/Moscow"),
        ("ar-EG","Africa/Cairo"),
        ("it-IT","Europe/Rome"),
        ("ko-KR","Asia/Seoul"),
        ("es-MX","America/Mexico_City"),
        ("fr-CA","America/Montreal"),
        ("nl-NL","Europe/Amsterdam"),
        ("tr-TR","Europe/Istanbul"),
        ("pl-PL","Europe/Warsaw"),
        ("hi-IN","Asia/Kolkata"),
        ("sv-SE","Europe/Stockholm"),
        ("cs-CZ","Europe/Prague"),
        ("el-GR","Europe/Athens"),
        ("da-DK","Europe/Copenhagen"),
        ("fi-FI","Europe/Helsinki"),
        ("he-IL","Asia/Jerusalem"),
        ("hu-HU","Europe/Budapest"),
        ("id-ID","Asia/Jakarta"),
        ("no-NO","Europe/Oslo"),
        ("ro-RO","Europe/Bucharest"),
        ("sk-SK","Europe/Bratislava"),
        ("th-TH","Asia/Bangkok"),
        ("uk-UA","Europe/Kiev"),
        ("en-CA","America/Toronto"),
        ("fr-BE","Europe/Brussels"),
        ("de-AT","Europe/Vienna"),
        ("es-AR","America/Buenos_Aires"),
        ("de-CH","Europe/Zurich"),
        ("ro-MD","Europe/Chisinau"),
        ("ka-GE","Asia/Tbilisi"),
        ("el-CY","Asia/Nicosia"),
        ("be-BY","Europe/Minsk"),
        ("kk-KZ","Asia/Almaty"),
        ("ky-KG","Asia/Bishkek"),
        ("uz-UZ","Asia/Tashkent"),
        ("tk-TM","Asia/Ashgabat"),
        ("az-AZ","Asia/Baku"),
        ("hy-AM","Asia/Yerevan"),
        ("lt-LT","Europe/Vilnius"),
        ("lv-LV","Europe/Riga"),
        ("et-EE","Europe/Tallinn"),
        ("fi-FI","Europe/Helsinki"),
        ("bg-BG","Europe/Sofia"),
        ("sr-RS","Europe/Belgrade"),
        ("hr-HR","Europe/Zagreb"),
        ("sl-SI","Europe/Ljubljana"),
        ("mk-MK","Europe/Skopje"),
        ("bs-BA","Europe/Sarajevo"),
        ("sq-AL","Europe/Tirane"),
        ("mt-MT","Europe/Malta"),
        ("he-IL","Asia/Jerusalem"),
        ("ar-IL","Asia/Jerusalem"),
        ("en-IL","Asia/Jerusalem"),
        ("ar-JO","Asia/Amman"),
        ("en-AE","Asia/Dubai"),
        ("ar-AE","Asia/Dubai"),
        ("en-QA","Asia/Qatar"),
        ("ar-QA","Asia/Qatar"),
        ("ar-SA","Asia/Riyadh"),
        ("en-OM","Asia/Muscat"),
        ("ar-KW","Asia/Kuwait"),
        ("en-LB","Asia/Beirut"),
        ("ar-SY","Asia/Damascus"),
        ("en-BH","Asia/Bahrain"),
        ("en-YE","Asia/Aden"),
        ("ar-IQ","Asia/Baghdad"),
        ("en-EG","Africa/Cairo"),
        ("ar-LY","Africa/Tripoli"),
        ("en-AS", "Pacific/Pago_Pago"),
        ("en-CK", "Pacific/Rarotonga"),
        ("en-PF", "Pacific/Tahiti"),
        ("en-US", "America/Anchorage"),
        ("en-MX", "America/Tijuana"),
        ("es-CR", "America/Costa_Rica"),
        ("en-PA", "America/Panama"),
        ("es-PY", "America/Asuncion"),
        ("en-AG", "America/Antigua"),
        ("pt-CV", "Atlantic/Cape_Verde"),
        ("en-GH", "Africa/Accra"),
        ("de-DE", "Europe/Berlin"),
        ("ar-EG", "Africa/Cairo"),
        ("ar-SA", "Asia/Riyadh"),
        ("fa-AF", "Asia/Kabul"),
        ("ur-PK", "Asia/Karachi"),
        ("hi-IN", "Asia/Kolkata"),
        ("my-MM", "Asia/Rangoon"),
        ("th-TH", "Asia/Bangkok"),
        ("zh-CN", "Asia/Shanghai"),
        ("ja-JP", "Asia/Tokyo"),
        ("en-AU", "Australia/Adelaide"),
        ("en-AU", "Australia/Sydney"),
        ("en-VU", "Pacific/Efate"),
        ("en-UM", "Pacific/Wake"),
        ("en-NZ", "Pacific/Auckland"),
        ("en-KI", "Pacific/Kiritimati")
    ];

    let mut rng = rand::thread_rng();
    let _idx = rng.gen_range(0..tz_pool.len());
    
    process(tz_pool[_idx])
}