mod cmdline;
mod builtin;
mod error;

use std::{
    io::{self, Write},
    process::{
        Child,
        Command,
    }
};
use error::ShellError;

use crate::{
    builtin::*,
    cmdline::CommandLine,
};

fn main() {
    loop {
        print!("$ ");

        match io::stdout().flush() {
            Ok(()) => {},
            Err(e) => eprintln!("tsh: {e}")
        };

        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}, // 読み込んだ byte数(usize型) が返るが、使用しないため _ とする
            Err(e) => eprintln!("tsh: {e}")
        };

        let line: &str = input.trim();
        let cli: CommandLine = match CommandLine::new(line) {
            Some(cli) => cli,
            None => continue
        };
        match execute_command(cli) {
            Ok(_) => {},
            Err(e) => eprintln!("tsh: {e}")
        }
    }
}

fn execute_command(cli: CommandLine) -> Result<(), ShellError> {
    if is_built_in(&cli.command) {
        exec_built_in(&cli.command, cli.args)
    } else {
        let mut child: Child = match Command::new(&cli.command)
                                        .args(cli.args)
                                        .spawn() {
            Ok(c) => c,
            Err(e) => return Err(
                ShellError::CommandExecError(cli.command, e.to_string())
            )
        };
        match child.wait() {
            Ok(_) => Ok(()), // ExitStatus が返るが、使用しないため _ とする
            Err(e) => return Err(
                ShellError::CommandExecError(cli.command, e.to_string())
            )
        }
    }
}
