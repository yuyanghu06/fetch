use clap::{Args,Parser,Subcommand};
use fetch_local::{application::{ApplicationHost,default_app_port,default_data_root,home_directory,show_existing},development::{DevelopmentOptions,DevelopmentServer,shutdown_timeout}};
use std::{path::PathBuf,process::ExitCode,sync::Arc};

#[derive(Parser)]
#[command(name="fetch",version,about="Share installed MCP servers through one local Fetch connection")]
struct Cli {#[command(subcommand)]command:Option<Command>}
#[derive(Subcommand)]
enum Command {
    /// Connect MCP over stdio; open Fetch for first-time permission approval.
    Connect(ConnectOptions),
    /// Open the native Fetch app. Closing its window keeps the backend running.
    App {#[arg(long)]data_dir:Option<PathBuf>,#[arg(long,default_value_t=default_app_port())]port:u16,/// Separate discovery home for an explicitly selected isolated environment.
        #[arg(long)]discovery_home:Option<PathBuf>,
        /// Native approval handoff used by the bundled connector.
        #[arg(long,hide=true)]connect_client:Option<String>,
        #[arg(long,hide=true)]connect_project:Option<String>},
    /// Run the original fixture-only developer foundation with ephemeral HTTPS and denied app authority.
    Dev {#[arg(long,default_value=".fetch-dev")]data_dir:PathBuf,#[arg(long,default_value_t=53671)]port:u16},
}
#[derive(Args)]
struct ConnectOptions {
    /// Separate approved client connection, for example claude-code or codex.
    #[arg(long,global=true)]client:Option<String>,
    /// Registered project ID; omitted connects only to Shared servers.
    #[arg(long,global=true)]project:Option<String>,
    #[arg(long,global=true)]data_dir:Option<PathBuf>,
    #[arg(long,global=true,default_value_t=default_app_port())]port:u16,
    #[command(subcommand)]command:Option<ConnectCommand>,
}
#[derive(Subcommand)]
enum ConnectCommand {
    /// Open Fetch and request human approval; never edits coding-client settings.
    Setup,
}
fn main()->ExitCode {
    match run(){Ok(())=>ExitCode::SUCCESS,Err(error)=>{eprintln!("Fetch could not start: {error}");ExitCode::FAILURE}}
}
fn run()->Result<(),Box<dyn std::error::Error>> {
    let runtime=tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    match Cli::parse().command.unwrap_or(Command::App{data_dir:None,port:default_app_port(),discovery_home:None,connect_client:None,connect_project:None}) {
        Command::Connect(options)=>{
            let connection=fetch_local::connector::Options{root:options.data_dir.map(Ok).unwrap_or_else(default_data_root)?,port:options.port,client:options.client.ok_or("Specify --client claude-code or --client codex for a separately approved connection.")?,project_id:options.project};
            connection.context().validate().map_err(std::io::Error::other)?;
            runtime.block_on(async move{match options.command{Some(ConnectCommand::Setup)=>fetch_local::connector::setup(connection).await,None=>fetch_local::connector::run(connection).await}}).map_err(|error|std::io::Error::other(error).into())
        },
        Command::Dev{data_dir,port}=>runtime.block_on(async move{
            let server=DevelopmentServer::prepare(DevelopmentOptions{data_dir,port}).await?;
            println!("Fetch fixture-only developer backend: {}",server.origin);
            println!("Public ephemeral certificate: {}",server.certificate_path.display());
            println!("This mode has no app grants or managed-server startup. Native MVP source uses the app command.");
            let handle=axum_server::Handle::new();let shutdown=handle.clone();
            tokio::spawn(async move{if tokio::signal::ctrl_c().await.is_ok(){shutdown.graceful_shutdown(Some(shutdown_timeout()));}});
            server.run(handle).await
        }).map_err(Into::into),
        Command::App{data_dir,port,discovery_home,connect_client,connect_project}=>{
            let context=connect_client.map(|client|fetch_local::connector::ConnectionContext {client,project_id:connect_project});
            if let Some(context)=&context {context.validate().map_err(std::io::Error::other)?;}
            let root=data_dir.map(Ok).unwrap_or_else(default_data_root)?;
            let inherit_codex_home=discovery_home.is_none();
            let home=discovery_home.map(Ok).unwrap_or_else(home_directory)?;
            let host=match runtime.block_on(ApplicationHost::prepare(root.clone(),port,home,inherit_codex_home)){
                Ok(host)=>host,
                Err(error)if error.kind()==std::io::ErrorKind::AlreadyExists=>{if let Some(context)=&context{runtime.block_on(fetch_local::application::show_connection_context(&root,context))?;}else{runtime.block_on(show_existing(&root))?;}return Ok(());},
                Err(error)=>return Err(error.into()),
            };
            fetch_local::native_auth::prepare_app_icon();
            if let Some(context)=context{host.request_connection_context(context);}
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

#[cfg(test)]
mod cli_tests {
    use super::*;
    #[test]
    fn connector_help_and_setup_parse_without_starting_the_app(){
        use clap::CommandFactory;
        Cli::command().debug_assert();
        for args in [vec!["fetch","connect","--client","claude-code"],vec!["fetch","connect","setup","--client","codex"]] {
            let command=Cli::try_parse_from(args).unwrap().command;
            assert!(matches!(command,Some(Command::Connect(ConnectOptions{client:Some(_),..}))));
        }
        assert!(matches!(Cli::try_parse_from(["fetch","app","--connect-client","claude-code"]).unwrap().command,
            Some(Command::App{connect_client:Some(name),..}) if name=="claude-code"));
        assert!(matches!(Cli::try_parse_from(["fetch","connect","--client","codex","--project","project-0123456789abcdef0123456789abcdef"]).unwrap().command,
            Some(Command::Connect(ConnectOptions{project:Some(id),..})) if id=="project-0123456789abcdef0123456789abcdef"));
    }
}
