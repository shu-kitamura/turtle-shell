mod cmdline;
mod builtin;
mod error;

use std::{
    io::{self, Write},
    process::{
        Child,
        Command,
        Stdio,
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
    let mut commands_peekable = cli.commands.iter().peekable();
    let mut prev: Option<Child> = None;

    while let Some((i, cmd, args)) = commands_peekable.next() {
        if is_built_in(cmd) {
            match exec_built_in(i, cmd, args) {
                Ok(_) => {},
                Err(e) => return Err(
                    ShellError::CommandExecError(cmd.to_owned(), e.to_string())
                )
            }
        } else {
            let input: Stdio = prev.map_or(
                Stdio::inherit(),
                |child| Stdio::from(child.stdout.unwrap())
            );

            let output: Stdio = commands_peekable.peek().map_or(
                Stdio::inherit(),
                 |_| Stdio::piped()
            );

            let child: Child = Command::new(cmd)
                                    .args(args.to_owned())
                                    .stdin(input)
                                    .stdout(output)
                                    .spawn()
                                    .unwrap();
            prev = Some(child)
        }
    }

    if let Some(mut final_command) = prev {
        final_command.wait().unwrap();
    }
    Ok(())
}
