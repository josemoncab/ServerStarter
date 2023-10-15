pub fn user(msg: &str) {
    println!("[\x1b[95mUSER\x1b[0m] {msg}")
}

pub fn success(msg: &str) {
    println!("[\x1b[92mSUCCESS\x1b[0m] {msg}")
}

pub fn info(msg: &str) {
    println!("[\x1b[94mINFO\x1b[0m] {msg}")
}

pub fn warn(msg: &str) {
    println!("[\x1b[33mWARN\x1b[0m] {msg}")
}

pub fn error(msg: &str) {
    println!("[\x1b[31mERROR\x1b[0m] {msg}")
}