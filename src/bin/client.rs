use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();


    loop {
        tokio::select! {
            // Read user input from stdin and send to server
            line = stdin.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        ws_stream.send(Message::text(line)).await?;
                    }
                    Ok(None) => break,
                    Err(e) => {
                        eprintln!("Error reading stdin: {e}");
                        break;
                    }
                }
            }
            // Receive messages from server and display
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) if msg.is_text() => {
                        let text = msg.as_text().unwrap_or("");
                        println!("Server: {text}");
                    }
                    Some(Ok(msg)) if msg.is_close() => {
                        println!("Server closed connection");
                        break;
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving from server: {e}");
                        break;
                    }
                    None | Some(Ok(_)) => break,
                }
            }
        }
    }

    Ok(())
}