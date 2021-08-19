use bridge::Bridge;
use clap::{load_yaml, App};
use mockserver::Client;
use ohmymock::Processor;

pub mod bridge;
pub mod mockserver;
pub mod ohmymock;

#[tokio::main]
async fn main() -> Result<(), String> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let host = matches.value_of("host").unwrap();

    let client = Client::new(&host);
    let processor = Processor::new();
    let bridge = Bridge::new(client, processor);

    match matches.subcommand() {
        ("create", Some(_sub_matches)) => apply_create_command(&bridge).await,
        _ => apply_search_command(&bridge).await,
    }

    Ok(())
}

async fn apply_create_command(b: &Bridge) {
    if let Err(s) = b.create_expectation("mock.json").await {
        panic!("🔥 ha ocurrido un error... {}", s);
    }
}

async fn apply_search_command(b: &Bridge) {
    match b.search_expectations(None).await {
        Ok(expectations) =>  expectations.iter().for_each(|e| println!("{},", e)),
        Err(s) => panic!("🔥 ha ocurrido un error... {}", s),
    }
}
