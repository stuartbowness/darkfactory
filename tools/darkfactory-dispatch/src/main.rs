use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use ulid::Ulid;

mod cli;
mod engine;
mod event;
mod supervisor;
mod types;

use cli::Cli;
use event::Emitter;
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

    let json = cli.json;
    let spec = cli.dispatch_spec().map_err(cli_error)?;
    let dispatch_id = Ulid::new().to_string();
    let emitter = Arc::new(Emitter::new(dispatch_id.clone()));

    let result = match engine::dispatch(spec, dispatch_id.clone(), emitter).await {
        Ok(result) => result,
        Err(error) => {
            let code = exit_code_for_engine_error(&error);
            let result = cli::failed_start_result(dispatch_id, error.to_string());
            cli::print_result(&result, true).map_err(cli_error)?;
            eprintln!("error: {error}");
            return Ok(code);
        }
    };
    cli::print_result(&result, json).map_err(cli_error)?;

    Ok(cli::exit_code_for_result(&result))
}

fn cli_error(message: impl Into<String>) -> (u8, String) {
    (EXIT_CLI_ERROR, message.into())
}

fn exit_code_for_engine_error(error: &DispatchError) -> u8 {
    match error {
        DispatchError::BinaryNotFound => EXIT_CLI_ERROR,
        _ => EXIT_DISPATCH_FAILURE,
    }
}
