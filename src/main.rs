use std::io::{Write, stdin};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await;

    let stream = match stream {
        Ok(mut stream) => {
            stream
        }
        _ => {
            println!("Failed to connect to server. Exiting.");
            return Ok(());
        }
    };
    
    println!(
        "Successfully connected to server at [{}].",
        stream.peer_addr()?
    );

    let (mut read_stream, mut write_stream) = tokio::io::split(stream);

    let mut buffer = [0u8; 1024];

    // Reading task
    tokio::spawn(async move {
        loop {
            let read_bytes = read_stream.read(&mut buffer).await.unwrap();

            if read_bytes == 0 {
                continue;
            }

            let message = String::from_utf8_lossy(&buffer);

            println!("[Server]: {}", message);
        }
    });

    // Writing task
    tokio::spawn(async move {
        loop {
            print!("> ");

            std::io::stdout().flush().unwrap();

            let mut user_message: String = String::new();

            stdin()
                .read_line(&mut user_message)
                .expect("Unable to process input.");

            match user_message.replace("\n", "").as_str().trim() {
                "/quit" => break,
                "" => continue,
                _ => {}
            }

            write_stream
                .write_all(user_message.as_bytes())
                .await
                .unwrap();
        }
    })
        .await?;
    // Wait for the writing to finish

    Ok(())
}
