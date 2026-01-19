use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
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
        return Err(ShellError::CommandExecError(
            String::from("cd"),
            Error::new(
                ErrorKind::InvalidInput,
                "cd command have to execute parent command.",
            ),
        ));
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
        PathBuf::from(path.as_str())
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
            println!("{}", path.to_string_lossy());
            Ok(())
        }
        Err(e) => Err(ShellError::CommandExecError(String::from("pwd"), e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::builtin::*;
    use std::{
        env::{current_dir, remove_var, set_current_dir, set_var, var},
        sync::Mutex,
    };

    static ENV_LOCK: Mutex<()> = Mutex::new(());
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
        let _guard = ENV_LOCK.lock().unwrap();
        let original_dir = current_dir().unwrap();
        let original_home = var("HOME").ok();
        let temp_home = std::env::temp_dir();
        // Safety: tests serialize env changes with ENV_LOCK.
        unsafe {
            set_var("HOME", temp_home.to_string_lossy().to_string());
        }

        // 引数 0 で実行するケース
        let expect = get_home_directory().unwrap();
        change_directory(0, &[]).unwrap();
        assert_eq!(current_dir().unwrap(), expect);

        // 引数 1 で実行するケース
        change_directory(0, &[expect.to_string_lossy().to_string()]).unwrap();
        assert_eq!(current_dir().unwrap(), expect);

        set_current_dir(original_dir).unwrap();
        // Safety: tests serialize env changes with ENV_LOCK.
        unsafe {
            if let Some(home) = original_home {
                set_var("HOME", home);
            } else {
                remove_var("HOME");
            }
        }
    }

    #[test]
    fn test_change_directory_too_many_args() {
        let _guard = ENV_LOCK.lock().unwrap();
        let err = change_directory(0, &[String::from("a"), String::from("b")]).unwrap_err();
        assert!(err.to_string().contains("Too many arguments"));
    }

    #[test]
    fn test_change_directory_in_pipeline() {
        let _guard = ENV_LOCK.lock().unwrap();
        let err = change_directory(1, &[]).unwrap_err();
        assert!(
            err.to_string()
                .contains("cd command have to execute parent command")
        );
    }

    #[test]
    fn test_get_home_directory_missing_home() {
        let _guard = ENV_LOCK.lock().unwrap();
        let original_home = var("HOME").ok();
        // Safety: tests serialize env changes with ENV_LOCK.
        unsafe {
            remove_var("HOME");
        }

        let err = get_home_directory().unwrap_err();
        assert!(err.to_string().contains("Home directory is not found"));

        // Safety: tests serialize env changes with ENV_LOCK.
        unsafe {
            if let Some(home) = original_home {
                set_var("HOME", home);
            } else {
                remove_var("HOME");
            }
        }
    }
}
