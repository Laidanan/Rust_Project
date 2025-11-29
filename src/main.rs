mod crypto;

use clap::{Parser, Subcommand};
use std::net::{TcpListener, TcpStream};

#[derive(Parser)]
#[command(version, about = "Secure Chat v0.1")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server { port: u16 },
    Client { address: String },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Server { port } => {
            let address = format!("0.0.0.0:{}", port);
            println!("[SERVER] Initializing on {}", address);
            let listener = TcpListener::bind(&address).expect("Failed to bind");
            
            println!("Waiting for connections...");
            // Accepte une seule connexion pour le MVP
            if let Ok((stream, _)) = listener.accept() {
                println!("Client connected!");
                handle_connection(stream, true);
            }
        }
        Commands::Client { address } => {
            println!("[CLIENT] Connecting to {}...", address);
            match TcpStream::connect(&address) {
                Ok(stream) => {
                    println!("Connection success!");
                    handle_connection(stream, false);
                }
                Err(e) => eprintln!("Connection error: {}", e),
            }
        }
    }
}

fn handle_connection(_stream: TcpStream, _is_server: bool) {
    // TODO: implémenter le handshake Diffie-Hellman ici
    println!("TODO: Implement Key Exchange");

    // TODO: doit implémenter la boucle de lecture/écriture (chat loop)
    println!("TODO: Implement Chat Loop");
    
    // Pour l'instant, on quitte direct
    println!("Session ended (Not implemented yet).");
}