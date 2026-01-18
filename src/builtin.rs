use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
    str::FromStr,
};

use crate::error::ShellError;

const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "cd", "pwd"];

pub fn is_built_in(command: &str) -> bool {
    BUILT_IN_COMMANDS.contains(&command)
}

pub fn exec_built_in(i: &usize, command: &str, args: &[String]) -> Result<(), ShellError<Error>> {
    match command {
        "exit" => exit(),
        "cd" => change_directory(*i, args),
        "pwd" => print_working_directory(),
        _ => unreachable!(),
    }
}

/// exit コマンド
fn exit() -> Result<(), ShellError<Error>> {
    println!("tsh: bye-bye");
    std::process::exit(0);
}

/// cd コマンド
fn change_directory(i: usize, args: &[String]) -> Result<(), ShellError<Error>> {
    if i != 0 {
        eprintln!("tsh: cd command have to execute parent command.")
    }

    // 引数が 2つ以上の場合、エラーを返す。
    let usage: &str = "cd [DIR_NAME]";
    if args.len() >= 2 {
        return Err(ShellError::CommandExecError(
            String::from("cd"),
            Error::new(
                ErrorKind::InvalidInput,
                format!("Too many arguments is inputed.\nUSAGE: {usage}"),
            ),
        ));
    }

    // 引数のディレクトリをカレントディレクトリに設定
    // 引数が指定されていない場合、ホームディレクトリをカレントディレクトリに設定
    let path: PathBuf = if let Some(path) = args.first() {
        PathBuf::from_str(path).unwrap()
    } else {
        get_home_directory()?
    };

    match std::env::set_current_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(ShellError::CommandExecError(String::from("cd"), e)),
    }
}

fn get_home_directory() -> Result<PathBuf, ShellError<Error>> {
    match std::env::var("HOME") {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_) => Err(ShellError::CommandExecError(
            String::from("cd"),
            Error::new(ErrorKind::NotFound, "Home directory is not found."),
        )),
    }
}

/// pwd コマンド
fn print_working_directory() -> Result<(), ShellError<Error>> {
    match std::env::current_dir() {
        Ok(path) => {
            println!("{}", path.to_str().unwrap());
            Ok(())
        }
        Err(e) => Err(ShellError::CommandExecError(String::from("pwd"), e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::builtin::*;
    use std::env::current_dir;
    #[test]
    fn test_is_built_in() {
        // 組み込みコマンド(exit)を受け取るケース
        let actual_exit: bool = is_built_in("exit");
        assert!(actual_exit);

        // 組み込みではないコマンド(ls)を受け取るケース
        let actual_ls: bool = is_built_in("ls");
        assert!(!actual_ls);
    }

    #[test]
    fn test_change_directory() {
        // 引数 0 で実行するケース
        let expect: PathBuf = get_home_directory().unwrap();
        let _ = change_directory(0, &vec![]);
        assert_eq!(current_dir().unwrap(), expect);

        // 引数 1 で実行するケース
        let _ = change_directory(0, &vec![expect.to_str().unwrap().to_string()]);
        assert_eq!(current_dir().unwrap(), expect);
    }
}
