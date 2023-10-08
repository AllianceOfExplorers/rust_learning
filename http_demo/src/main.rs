pub(crate) mod config;
pub(crate) mod csms;

fn main() {
    println!("Hello, world!");
    let c = crate::config::Config::new("127.0.0.1".to_string(), 7890);
    println!("Config: {:?}", c);
}
