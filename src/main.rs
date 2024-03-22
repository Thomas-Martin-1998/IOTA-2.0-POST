
use iota_sdk::{
    client::{Client, Result},
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let node_url: &str = &args[1];

    let tag = &args[2];

    let data = &args[3].replace("\\", "\"");

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Create and send the block with the custom payload.
    let block = client
        .build_block()
        .with_tag(tag.as_bytes().to_vec())
        .with_data(data.as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "{}",
        block.id()
    );

    Ok(())
}
