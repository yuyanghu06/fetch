use clap::{Parser, Subcommand};
use fetch_local::development::{DevelopmentOptions, DevelopmentServer, shutdown_timeout};
use std::{path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[command(
    name = "fetch",
    version,
    about = "Fetch local developer backend (MCP interoperability is not implemented yet)"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the shared backend with private developer state and ephemeral HTTPS.
    Dev {
        /// Separate developer state. Does not use macOS Application Support.
        #[arg(long, default_value = ".fetch-dev")]
        data_dir: PathBuf,
        /// IPv4 loopback port. Use zero to select an available port.
        #[arg(long, default_value_t = 4141)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Fetch could not start: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> std::io::Result<()> {
    match Cli::parse().command {
        Command::Dev { data_dir, port } => {
            let server = DevelopmentServer::prepare(DevelopmentOptions { data_dir, port }).await?;
            println!("Fetch developer backend: {}", server.origin);
            println!(
                "Public development certificate: {}",
                server.certificate_path.display()
            );
            println!(
                "This certificate changes on each run. Its private key exists only in memory."
            );
            println!("No system trust, Keychain or personal client configuration is changed.");
            println!(
                "Native app approval, tokens, installation and MCP sharing are not implemented yet."
            );
            println!(
                "Check HTTPS with: curl --cacert '{}' {}/.well-known/oauth-protected-resource/mcp",
                server
                    .certificate_path
                    .display()
                    .to_string()
                    .replace('\'', "'\"'\"'"),
                server.origin
            );
            println!("Press Ctrl-C to stop.");
            let handle = axum_server::Handle::new();
            let shutdown_handle = handle.clone();
            tokio::spawn(async move {
                if tokio::signal::ctrl_c().await.is_ok() {
                    shutdown_handle.graceful_shutdown(Some(shutdown_timeout()));
                }
            });
            server.run(handle).await
        }
    }
}
