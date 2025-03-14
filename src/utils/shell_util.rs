use crate::types::errors::CommonsError;
use clap_complete::Shell;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn execute_command(command: &str) -> Result<(), CommonsError> {
    let shell = current_shell().to_string();

    Command::new(shell.as_str())
        .arg("-i")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn a process.")
        .wait()
        .expect("Failed to wait for the process.");

    Ok(())
}

pub fn execute_commands(commands: Vec<&str>) -> Result<(), CommonsError> {
    let command = format!("({})", commands.join(" ; "));
    execute_command(&command)
}

pub fn execute_commands_in_dir(dir: &PathBuf, commands: Vec<&str>) -> Result<(), CommonsError> {
    let dir = dir.as_os_str().to_str().unwrap();
    let mut updated_commands = commands;
    let cd_command = format!("cd {}", dir);
    updated_commands.insert(0, &cd_command);
    execute_commands(updated_commands)
}

fn current_shell() -> Shell {
    clap_complete::Shell::from_env().unwrap_or(Shell::Zsh)
}
