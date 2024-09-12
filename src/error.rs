use std::{
    error::Error,
    fmt::{self, Display, Debug},
};

/// シェルの処理で発生するエラーを表す型
#[derive(Debug, PartialEq)]
pub enum ShellError<E: ToString> {
    CommandExecError(String, E),
}

/// ShellErrorを表示するため、Displayトレイトを実装
impl<E: ToString> Display for ShellError<E> {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::CommandExecError(cmd, err) => {
                write!(f, "Error: The following Error is occured when execute '{}'.\n{}", cmd, err.to_string())
            }
        }
    }
}

/// エラー用にErrorトレイトを実装
impl<E: ToString + Debug> Error for ShellError<E> {} // デフォルト実装を使うだけの場合、これだけでいい
