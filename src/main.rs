mod models;

use anyhow::Ok;
use futures_util::StreamExt;
use models::BinanceTrade;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@aggTrade";
    println!("---Connecting to Biance...---");

    let (ws_stream, _) = connect_async(url).await?;
    println!("Connect Successful");

    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        let msg = message?;
        if let Message::Text(text) = msg {
            let trade: BinanceTrade = serde_json::from_str(&text)?;
            println!(
                "Price: {}| Quantity: {}| At: {}",
                trade.price, trade.quantity, trade.event_time
            );
        }
    }

    Ok(())
}
