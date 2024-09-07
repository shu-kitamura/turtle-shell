use std::{
    env::{home_dir, set_current_dir},
    io::{Error, ErrorKind},
    os::unix::process::ExitStatusExt,
    process::ExitStatus
};

pub fn is_built_in(command: &str) -> bool {
    match command {
        "exit" => true,
        "cd" => true,
        _ => false,
    }
}

pub fn exec_built_in(command: &str, args: Vec<String>) -> Result<ExitStatus, Error> {
    match command {
        "exit" => exit(),
        "cd" => change_directory(args),
        _ => unreachable!()
    }
}

/// exit コマンド
fn exit() -> Result<ExitStatus, Error> {
    println!("tsh: bye-bye");
    std::process::exit(0)
}

fn change_directory(args: Vec<String>) -> Result<ExitStatus, Error> {
    if args.len() == 0 {
        let home = home_dir().unwrap();
        match set_current_dir(home) {
            Ok(()) => Ok(ExitStatus::from_raw(0)),
            Err(e) => Err(e)    
        }
    } else if args.len() == 1 {
        match set_current_dir(args.get(0).unwrap()) {
            Ok(()) => Ok(ExitStatus::from_raw(0)),
            Err(e) => Err(e)
        }
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Too many arguments is inputed."))
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::builtin::*;
    #[test]
    fn test_is_built_in() {
        // 組み込みコマンド(exit)を受け取るケース
        let actual_exit: bool = is_built_in("exit");
        assert_eq!(actual_exit, true);

        // 組み込みではないコマンド(ls)を受け取るケース
        let actual_ls = is_built_in("ls");
        assert_eq!(actual_ls, false);
    }

    #[test]
    fn test_change_directory() {
        // 引数 0 で実行するケース
        let expect = home_dir().unwrap();
        let _ = change_directory(vec![]);
        assert_eq!(current_dir().unwrap(), expect);

        // 引数 1 で実行するケース
        let _ = change_directory(vec![expect.to_str().unwrap().to_string()]);
        assert_eq!(current_dir().unwrap(), expect);
    }
}