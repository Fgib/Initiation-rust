use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::Utc;

struct LogWriter {
    file: Arc<Mutex<tokio::fs::File>>,
}

impl LogWriter {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/server.log")
            .await?;
        
        Ok(LogWriter {
            file: Arc::new(Mutex::new(file)),
        })
    }

    async fn write_log(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
        let log_entry = format!("[{}] {}\n", timestamp, message);
        
        let mut file = self.file.lock().await;
        file.write_all(log_entry.as_bytes()).await?;
        file.flush().await?;
        
        println!("Log écrit: [{}] {}", timestamp, message);
        Ok(())
    }
}

async fn handle_client(mut stream: TcpStream, log_writer: Arc<LogWriter>) {
    let (reader, _writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    println!("Nouveau client connecté");

    loop {
        line.clear();
        match buf_reader.read_line(&mut line).await {
            Ok(0) => {
                println!("Client déconnecté");
                break;
            }
            Ok(_) => {
                let message = line.trim();
                if !message.is_empty() {
                    if let Err(e) = log_writer.write_log(message).await {
                        eprintln!("Erreur lors de l'écriture du log: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Erreur lors de la lecture du client: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Démarrage du serveur de journalisation...");

    let log_writer = Arc::new(LogWriter::new().await?);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur en écoute sur 127.0.0.1:8080");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Nouvelle connexion de: {}", addr);
                
                let log_writer_clone = Arc::clone(&log_writer);
                
                tokio::spawn(async move {
                    handle_client(stream, log_writer_clone).await;
                });
            }
            Err(e) => {
                eprintln!("Erreur lors de l'acceptation de connexion: {}", e);
            }
        }
    }
}
