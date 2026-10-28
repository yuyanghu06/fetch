use clap::{Parser,Subcommand};
use fetch_local::{application::{ApplicationHost,default_app_port,default_data_root,home_directory,show_existing},development::{DevelopmentOptions,DevelopmentServer,shutdown_timeout}};
use std::{path::PathBuf,process::ExitCode,sync::Arc};

#[derive(Parser)]
#[command(name="fetch",version,about="Share installed MCP servers through one local Fetch connection")]
struct Cli {#[command(subcommand)]command:Option<Command>}
#[derive(Subcommand)]
enum Command {
    /// Open the native Fetch app. Closing its window keeps the backend running.
    App {#[arg(long)]data_dir:Option<PathBuf>,#[arg(long,default_value_t=default_app_port())]port:u16,/// Separate discovery home for an explicitly selected isolated environment.
        #[arg(long)]discovery_home:Option<PathBuf>},
    /// Run the original fixture-only developer foundation with ephemeral HTTPS and denied app authority.
    Dev {#[arg(long,default_value=".fetch-dev")]data_dir:PathBuf,#[arg(long,default_value_t=53671)]port:u16},
}
fn main()->ExitCode {
    match run(){Ok(())=>ExitCode::SUCCESS,Err(error)=>{eprintln!("Fetch could not start: {error}");ExitCode::FAILURE}}
}
fn run()->Result<(),Box<dyn std::error::Error>> {
    let runtime=tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    match Cli::parse().command.unwrap_or(Command::App{data_dir:None,port:default_app_port(),discovery_home:None}) {
        Command::Dev{data_dir,port}=>runtime.block_on(async move{
            let server=DevelopmentServer::prepare(DevelopmentOptions{data_dir,port}).await?;
            println!("Fetch fixture-only developer backend: {}",server.origin);
            println!("Public ephemeral certificate: {}",server.certificate_path.display());
            println!("This mode has no app grants or managed-server startup. Native MVP source uses the app command.");
            let handle=axum_server::Handle::new();let shutdown=handle.clone();
            tokio::spawn(async move{if tokio::signal::ctrl_c().await.is_ok(){shutdown.graceful_shutdown(Some(shutdown_timeout()));}});
            server.run(handle).await
        }).map_err(Into::into),
        Command::App{data_dir,port,discovery_home}=>{
            let root=data_dir.map(Ok).unwrap_or_else(default_data_root)?;
            let inherit_codex_home=discovery_home.is_none();
            let home=discovery_home.map(Ok).unwrap_or_else(home_directory)?;
            let host=match runtime.block_on(ApplicationHost::prepare(root.clone(),port,home,inherit_codex_home)){
                Ok(host)=>host,
                Err(error)if error.kind()==std::io::ErrorKind::AlreadyExists=>{runtime.block_on(show_existing(&root))?;return Ok(());},
                Err(error)=>return Err(error.into()),
            };
            {let _context=runtime.enter();host.start()?;}
            install_shutdown_signal(&runtime,host.clone());
            // Iced and AppKit run on the original main thread. Tokio owns backend work.
            let ui=fetch_local::ui::run(host.clone());
            runtime.block_on(host.shutdown());ui.map_err(Into::into)
        }
    }
}
fn install_shutdown_signal(runtime:&tokio::runtime::Runtime,host:Arc<ApplicationHost>){runtime.spawn(async move{
    #[cfg(unix)]{
        use tokio::signal::unix::{signal,SignalKind};
        if let Ok(mut termination)=signal(SignalKind::terminate()){
            tokio::select!{_=tokio::signal::ctrl_c()=>{},_=termination.recv()=>{}}
        }else{let _=tokio::signal::ctrl_c().await;}
    }
    #[cfg(not(unix))]{let _=tokio::signal::ctrl_c().await;}
    host.shutdown().await;
});}
