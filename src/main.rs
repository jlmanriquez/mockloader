use bridge::Bridge;
use mockserver::Client;
use ohmymock::Processor;

pub mod ohmymock;
pub mod bridge;
pub mod mockserver;

#[tokio::main]
async fn main() -> Result<(), String> {
    let client = Client::new("http://127.0.0.1:1080");
    let processor = Processor::new();
    let bridge = Bridge::new(client, processor);

    // let _ = bridge.create_expectation("mock.json").await?;
    let expectations = bridge.search_expectations(None).await?;
    expectations.iter().for_each(|e| println!("{}", e));

    Ok(())
}