mod builtin;
mod cmdline;
mod error;

use std::{
    io::{self, Error, Write},
    process::{Child, Command, Stdio},
};

use crate::{builtin::*, cmdline::CommandLine, error::ShellError};

fn main() {
    loop {
        print!("$ ");

        match io::stdout().flush() {
            Ok(()) => {}
            Err(e) => eprintln!("tsh: {e}"),
        };

        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {} // 読み込んだ byte数(usize型) が返るが、使用しないため _ とする
            Err(e) => eprintln!("tsh: {e}"),
        };

        let cli: CommandLine = CommandLine::new(&input);
        match execute_command(cli) {
            Ok(_) => {}
            Err(e) => eprintln!("tsh: {e}"),
        }
    }
}

fn execute_command(cli: CommandLine) -> Result<(), ShellError<Error>> {
    let mut commands_peekable = cli.commands.iter().peekable();
    let mut prev: Option<(String, Child)> = None;
    let mut children: Vec<(String, Child)> = Vec::new();
    let mut execution_error: Option<ShellError<Error>> = None;

    while let Some(parsed) = commands_peekable.next() {
        let index = &parsed.index;
        let cmd = parsed.name.as_str();
        let args = &parsed.args;

        if is_built_in(cmd) {
            if let Err(err) = exec_built_in(index, cmd, args) {
                execution_error = Some(err);
                break;
            }
        } else {
            let input: Option<Stdio> = match prev.take() {
                Some((prev_cmd, mut child)) => {
                    let stdout = match child.stdout.take() {
                        Some(stdout) => Some(stdout),
                        None => {
                            execution_error = Some(ShellError::CommandExecError(
                                prev_cmd.clone(),
                                Error::other("stdout pipe is missing."),
                            ));
                            None
                        }
                    };
                    children.push((prev_cmd, child));
                    stdout.map(Stdio::from)
                }
                None => Some(Stdio::inherit()),
            };

            let input = match input {
                Some(input) => input,
                None => break,
            };

            let output: Stdio = commands_peekable
                .peek()
                .map_or(Stdio::inherit(), |_| Stdio::piped());

            let child: Child = match Command::new(cmd)
                .args(args)
                .stdin(input)
                .stdout(output)
                .spawn()
            {
                Ok(child) => child,
                Err(err) => {
                    execution_error = Some(ShellError::CommandExecError(cmd.to_string(), err));
                    break;
                }
            };
            prev = Some((cmd.to_string(), child));
        }
    }

    if let Some(final_command) = prev {
        children.push(final_command);
    }

    let mut wait_error: Option<ShellError<Error>> = None;
    for (cmd, mut child) in children {
        if let Err(err) = child.wait() {
            if wait_error.is_none() {
                wait_error = Some(ShellError::CommandExecError(cmd, err));
            }
        }
    }

    if let Some(err) = execution_error {
        return Err(err);
    }

    if let Some(err) = wait_error {
        return Err(err);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{CommandLine, execute_command};
    use std::process::Command;

    #[test]
    fn test_execute_command_nonexistent() {
        let cli = CommandLine::new("this-command-does-not-exist");
        assert!(execute_command(cli).is_err());
    }

    #[test]
    fn test_execute_command_pipeline() {
        if Command::new("true").status().is_err() {
            return;
        }

        let cli = CommandLine::new("true | true");
        assert!(execute_command(cli).is_ok());
    }
}
