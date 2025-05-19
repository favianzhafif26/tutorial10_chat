use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use std::error::Error;
use tokio::io::AsyncBufReadExt;
use tokio_websockets::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "ws://127.0.0.1:2000";

    let (ws_stream, _) = tokio_websockets::ClientBuilder::new()
        .uri(url)?
        .connect()
        .await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    println!("Favian's Computer - From server: Welcome to chat! Type a message");

    loop {
        tokio::select! {
            // Handle user input from stdin
            Ok(Some(line)) = stdin.next_line() => {
                ws_sender.send(Message::text(line)).await?;
            }
            // Handle incoming messages from the server
            Some(msg) = ws_receiver.next() => {
                let msg = msg?;
                if msg.is_text() {
                    println!("Favian's Computer - From server: {}", msg.as_text().ok_or("Not a text message")?);
                }
            }
            else => break,
        }
    }

    println!("Connection closed");
    Ok(())
}