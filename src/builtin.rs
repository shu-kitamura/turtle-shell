use std::{
    env::{self, set_current_dir},
    path::PathBuf, str::FromStr
};
use dirs::home_dir;

use crate::error::ShellError;

pub fn is_built_in(command: &str) -> bool {
    match command {
        "exit" | "cd" | "pwd" => true,
        _ => false,
    }
}

pub fn exec_built_in(i:&usize, command: &str, args: Vec<String>) -> Result<(), ShellError> {
    match command {
        "exit" => exit(),
        "cd" => change_directory(*i, args),
        "pwd" => print_working_directory(),
        _ => unreachable!()
    }
}

/// exit コマンド
fn exit() -> Result<(), ShellError> {
    println!("tsh: bye-bye");
    std::process::exit(0);
}

/// cd コマンド
fn change_directory(i: usize, args: Vec<String>) -> Result<(), ShellError> {
    if i != 0 {
        eprintln!("tsh: cd command have to execute parent command.")
    }
    // 引数が 2つ以上の場合、エラーを返す。
    let usage: &str = "cd [DIR_NAME]";
    if args.len() >= 2 {
        return Err(ShellError::CommandExecError(
            "cd".to_string(),
            format!("Too many arguments are specified.\nUSAGE: {}", usage),
        ))
    }

    // 引数のディレクトリをカレントディレクトリに設定
    // 引数が指定されていない場合、ホームディレクトリをカレントディレクトリに設定
    let path: PathBuf = if let Some(path) = args.get(0) {
        PathBuf::from_str(&path).unwrap()
    } else {
        home_dir().unwrap()
    };

    match set_current_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(
            ShellError::CommandExecError("cd".to_string(), e.to_string())
        )
    }
}

/// pwd コマンド
fn print_working_directory() -> Result<(), ShellError>{
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.to_str().unwrap());
            Ok(())
        },
        Err(e) => Err(
            ShellError::CommandExecError("pwd".to_string(), e.to_string())
        )
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::{
        builtin::*,
        error::ShellError
    };
    #[test]
    fn test_is_built_in() {
        // 組み込みコマンド(exit)を受け取るケース
        let actual_exit: bool = is_built_in("exit");
        assert_eq!(actual_exit, true);

        // 組み込みではないコマンド(ls)を受け取るケース
        let actual_ls: bool = is_built_in("ls");
        assert_eq!(actual_ls, false);
    }

    #[test]
    fn test_change_directory() {
        // 引数 0 で実行するケース
        let expect: PathBuf = home_dir().unwrap();
        let _ = change_directory(0, vec![]);
        assert_eq!(current_dir().unwrap(), expect);

        // 引数 1 で実行するケース
        let _ = change_directory(0, vec![expect.to_str().unwrap().to_string()]);
        assert_eq!(current_dir().unwrap(), expect);

        // 引数 2 で実行するケース (Error)
        let expect_error: ShellError = ShellError::CommandExecError(
            "cd".to_string(),
            "Too many arguments are specified.\nUSAGE: cd [DIR_NAME]".to_string()
        );
        let actual_error: ShellError = change_directory(0, vec!["a".to_string(), "b".to_string()]).unwrap_err();
        assert_eq!(actual_error, expect_error);

        // 存在しないディレクトリを指定するケース (Error)
        let expect_error: ShellError = ShellError::CommandExecError(
            "cd".to_string(),
            "No such file or directory (os error 2)".to_string()
        );
        let actual_error: ShellError = change_directory(0, vec!["NOT_EXIST_DIR".to_string()]).unwrap_err();
        assert_eq!(actual_error, expect_error);
    }
}