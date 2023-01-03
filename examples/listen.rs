use std::env;
use rusty_pusher::Pusher;

extern crate rusty_pusher;

#[tokio::main]
async fn main() {
    let name = env::args().skip(1).next();
    // let result = rusty_pusher::add(1,2);
    // println!("{}", result);
    // println!("Hello, {}!", name.unwrap_or("world".into()));

    // [scheme]://ws-[cluster_name].pusher.com:[port]/app/[key]
    let host = "wss://ws-us2.pusher.com:443/app/a50477b84c8ee1f2b8e3?client=rust-rusty_pusher&version=0.1&protocol=7";
    let pusher_client = Pusher{ host: host.to_string() };
    pusher_client.connect().await;
}