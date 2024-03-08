use client::Client;

mod client;

pub fn main() {
    let client = Client::new();
    client.run();
}
