use futures_util::{future, pin_mut, SinkExt, StreamExt};
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Pusher {
    pub host: String,
}

impl Pusher {
    pub async fn connect(self) {
        println!("Attempting to connect to: {}", self.host);

        let url = url::Url::parse(&self.host).unwrap();

        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();

        tokio::spawn(Pusher::read_stdin(stdin_tx));

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (mut write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let msg = Message::Text(
            r#"{
        "event": "ping",
        "reqid": 42
      }"#
            .to_string()
                + "\n",
        );
        // write.send(msg).await.unwrap();

        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                let value = str::from_utf8(&*data);
                println!("received...");

                println!("{:?}", value.unwrap());
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }

    // Our helper method which will read data from stdin and send it along the
    // sender provided.
    async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
        let mut stdin = tokio::io::stdin();
        loop {
            let mut buf = vec![0; 1024];
            let n = match stdin.read(&mut buf).await {
                Err(_) | Ok(0) => break,
                Ok(n) => n,
            };
            let msg = Message::Text(
                r#"{
        "event": "pusher:ping",
        "reqid": 42
      }"#
                .to_string()
                    + "\n",
            );
            buf.truncate(n);
            tx.unbounded_send(msg).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
