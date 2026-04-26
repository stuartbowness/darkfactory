use clap::Parser;

#[expect(
    dead_code,
    reason = "supervisor is consumed by the upcoming engine module"
)]
mod supervisor;

#[derive(Parser)]
#[command(
    name = "darkfactory-dispatch",
    version,
    about = "Process supervisor for Codex CLI subprocesses"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    Status { id: String },
    Result { id: String },
    Wait { id: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        match command {
            Commands::Status { id } | Commands::Result { id } | Commands::Wait { id } => {
                let _ = id;
            }
        }
    }
    println!("darkfactory-dispatch stub");
}
