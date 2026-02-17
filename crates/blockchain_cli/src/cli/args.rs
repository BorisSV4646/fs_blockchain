use super::commands::CliCommand;
use std::env;

#[derive(Debug, Clone)]
pub struct CliArgs {
    pub config_path: Option<String>,
    pub command: CliCommand,
}

impl CliArgs {
    pub fn parse() -> Result<Self, String> {
        let mut config_path: Option<String> = None;
        let mut command: Option<CliCommand> = None;
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            if arg == "--config" {
                let value = args
                    .next()
                    .ok_or_else(|| "ожидался путь после --config".to_string())?;
                config_path = Some(value);
                continue;
            }

            if command.is_none() {
                command = Some(parse_command(&arg)?);
                continue;
            }

            return Err(format!("лишний аргумент: {arg}"));
        }

        let command = command.ok_or_else(|| {
            "команда не указана. Используй: init | add-tx | mine | print".to_string()
        })?;

        Ok(Self {
            config_path,
            command,
        })
    }
}

fn parse_command(value: &str) -> Result<CliCommand, String> {
    match value {
        "init" => Ok(CliCommand::Init),
        "add-tx" => Ok(CliCommand::AddTx),
        "mine" => Ok(CliCommand::Mine),
        "print" => Ok(CliCommand::Print),
        other => Err(format!("неизвестная команда: {other}")),
    }
}
