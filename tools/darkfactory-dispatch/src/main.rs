use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use ulid::Ulid;

mod cli;
mod engine;
mod event;
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
    let async_mode = cli.async_mode;
    let mut spec = cli.dispatch_spec().map_err(cli_error)?;
    let dispatch_id = Ulid::new().to_string();
    let (store_dir, _host_pid_guard, emitter) = if async_mode {
        let store_dir = cli::prepare_async_store(&dispatch_id).map_err(cli_error)?;
        let host_pid_guard = cli::write_host_pid(&store_dir).map_err(cli_error)?;
        cli::write_initial_status(&store_dir, &dispatch_id).map_err(cli_error)?;
        let events = cli::open_events_log(&store_dir).map_err(cli_error)?;
        cli::print_async_started(&dispatch_id).map_err(cli_error)?;
        spec.store_dir = Some(store_dir.clone());
        (
            Some(store_dir),
            Some(host_pid_guard),
            Arc::new(Emitter::with_writer(
                dispatch_id.clone(),
                StreamMode::Normal,
                events,
            )),
        )
    } else {
        (
            None,
            None,
            Arc::new(Emitter::new(dispatch_id.clone(), StreamMode::Normal)),
        )
    };

    let result = match engine::dispatch(spec, dispatch_id.clone(), emitter).await {
        Ok(result) => result,
        Err(error) => {
            if let Some(store_dir) = &store_dir {
                let _ = cli::write_result(
                    store_dir,
                    &cli::failed_start_result(dispatch_id, error.to_string()),
                );
            }
            return Err(engine_error(error));
        }
    };
    if let Some(store_dir) = &store_dir {
        cli::write_result(store_dir, &result).map_err(cli_error)?;
    } else {
        cli::print_result(&result, json).map_err(cli_error)?;
    }

    Ok(cli::exit_code_for_result(&result))
}

fn handle_lifecycle_command(command: Commands) -> Result<u8, (u8, String)> {
    match command {
        Commands::Status { id } => {
            cli::print_status(&id).map_err(cli_error)?;
            Ok(0)
        }
        Commands::Result { id } => {
            cli::print_result_for_id(&id).map_err(cli_error)?;
            Ok(0)
        }
        Commands::Wait { id } => cli::wait_for_id(&id).map_err(cli_error),
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
