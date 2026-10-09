use anyhow::Result;
use serde_json::json;
use smartconnect_shared::protocol::Envelope;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 9000;
const SECRET: &str = "super_private_secret_change_this";

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(format!("{HOST}:{PORT}")).await?;
    println!("SmartConnect server listening on {}:{}", HOST, PORT);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Incoming connection from: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;

    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;

    let message: Envelope = serde_json::from_slice(&buf)?;
    println!("Received message: {:?}", message);

    if !message.verify(SECRET) {
        let error = Envelope::new("server", &message.from, "error", "invalid_signature", SECRET);
        send_envelope(&mut stream, &error).await?;
        return Ok(());
    }

    let response_payload = match message.kind.as_str() {
        "request_messages" => {
            let fake_messages = vec![
                "Mom: Please call me when you get home.",
                "Boss: Please send the report.",
                "Sarah: I’m waiting outside.",
            ];
            json!(fake_messages).to_string()
        }
        "ping" => json!({"status": "ok"}).to_string(),
        _ => json!({"status": "unknown_command"}).to_string(),
    };

    let response = Envelope::new(
        "server",
        &message.from,
        "messages_response",
        &response_payload,
        SECRET,
    );

    send_envelope(&mut stream, &response).await?;

    Ok(())
}

async fn send_envelope(stream: &mut TcpStream, envelope: &Envelope) -> Result<()> {
    let data = serde_json::to_vec(envelope)?;
    let len = (data.len() as u32).to_be_bytes();
    stream.write_all(&len).await?;
    stream.write_all(&data).await?;
    stream.flush().await?;
    Ok(())
}