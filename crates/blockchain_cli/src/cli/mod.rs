pub mod args;
pub mod commands;
pub mod handlers;

use blockchain_core::config::NodeConfig;
use args::CliArgs;
use commands::CliCommand;

pub fn run() {
    let args = match CliArgs::parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Ошибка CLI: {err}");
            return;
        }
    };

    let config = match handlers::load_config(args.config_path.as_deref()) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Ошибка конфигурации: {err}");
            return;
        }
    };

    dispatch(args.command, &config);
}

fn dispatch(command: CliCommand, config: &NodeConfig) {
    match command {
        CliCommand::Init => handlers::handle_init(config),
        CliCommand::AddTx => handlers::handle_add_tx(config),
        CliCommand::Mine => handlers::handle_mine(config),
        CliCommand::Print => handlers::handle_print(config),
    }
}
