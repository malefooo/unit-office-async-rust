/// 工具类，后面转成lib
use std::time::{SystemTime, UNIX_EPOCH};

/// String -> &'static str
/// 先是把
pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


/// 获取时间戳
pub fn timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}

pub fn u8_array_to_string(array:&[u8]) -> String{
    use std::fmt::Write;

    let mut signature_string = String::new();

    for a in array.iter() {
        write!(signature_string, "{:02x}", a);
    }

    signature_string
}