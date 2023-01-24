use std::net::TcpListener;
use dota_analytics::{configuration::{self, get_configuration}}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get configuration");
    
    // let listener = TcpListener::new()

    // run()

    Ok(())
}
