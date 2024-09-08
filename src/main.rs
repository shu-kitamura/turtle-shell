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

use crate::{
    builtin::*,
    cmdline::CommandLine,
    error::ShellError,
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
        let cli: CommandLine = CommandLine::new(line);
        match execute_command(cli) {
            Ok(_) => {},
            Err(e) => eprintln!("tsh: {e}")
        }
    }
}

fn execute_command(cli: CommandLine) -> Result<(), ShellError> {
    for (cmd, args) in cli.commands {
        if is_built_in(&cmd) {
            match exec_built_in(&cmd, args) {
                Ok(_) => {},
                Err(e) => return Err(e)
            }
        } else {
            let mut child: Child = match Command::new(&cmd)
                                    .args(args)
                                    .spawn() {
                Ok(c) => c,
                Err(e) => return Err(
                    ShellError::CommandExecError(cmd, e.to_string())
                )
            };
            match child.wait() {
                Ok(_) => {},
                Err(e) => return Err(
                    ShellError::CommandExecError(cmd, e.to_string())
                )
            }
        }
    }
    Ok(())
}
