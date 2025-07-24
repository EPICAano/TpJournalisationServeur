use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if !Path::new("logs").exists() {
        std::fs::create_dir("logs")?;
    }

    let log_path = "logs/server.log";

    // Création fichier de log
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    let log_file = Arc::new(Mutex::new(file));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur en écoute sur 127.0.0.1:8080...");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Connexion de : {}", addr);

        let log_file = log_file.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, log_file).await {
                eprintln!("Erreur avec le client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, log_file: Arc<Mutex<std::fs::File>>) -> std::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let now = Utc::now().to_rfc3339();
        let log_entry = format!("[{}] {}\n", now, line);

        let mut file = log_file.lock().await;
        file.write_all(log_entry.as_bytes())?;
    }

    Ok(())
}
