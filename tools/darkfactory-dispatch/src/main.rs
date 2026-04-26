use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use ulid::Ulid;

mod cli;
mod engine;
mod event;
#[expect(
    dead_code,
    reason = "supervisor has helper APIs consumed across upcoming async CLI work"
)]
mod supervisor;
mod types;

use cli::{Cli, Commands};
use event::{Emitter, StreamMode};
use types::DispatchError;

const EXIT_DISPATCH_FAILURE: u8 = 1;
const EXIT_CLI_ERROR: u8 = 2;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(code) => ExitCode::from(code),
        Err((code, message)) => {
            eprintln!("error: {message}");
            ExitCode::from(code)
        }
    }
}

async fn run() -> Result<u8, (u8, String)> {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        return handle_lifecycle_command(command);
    }

    let json = cli.json;
    let spec = cli.dispatch_spec().map_err(cli_error)?;
    let dispatch_id = Ulid::new().to_string();
    let emitter = Arc::new(Emitter::new(dispatch_id.clone(), StreamMode::Normal));

    let result = engine::dispatch(spec, dispatch_id, emitter)
        .await
        .map_err(engine_error)?;
    cli::print_result(&result, json).map_err(cli_error)?;

    Ok(cli::exit_code_for_result(&result))
}

fn handle_lifecycle_command(command: Commands) -> Result<u8, (u8, String)> {
    match command {
        Commands::Status { id } => Err(cli_error(format!("status is not implemented yet: {id}"))),
        Commands::Result { id } => Err(cli_error(format!("result is not implemented yet: {id}"))),
        Commands::Wait { id } => Err(cli_error(format!("wait is not implemented yet: {id}"))),
    }
}

fn cli_error(message: impl Into<String>) -> (u8, String) {
    (EXIT_CLI_ERROR, message.into())
}

fn engine_error(error: DispatchError) -> (u8, String) {
    let code = match error {
        DispatchError::BinaryNotFound => EXIT_CLI_ERROR,
        _ => EXIT_DISPATCH_FAILURE,
    };
    (code, error.to_string())
}
