use std::{
    error::Error,
    fmt::{self, Display},
};

/// シェルの処理で発生するエラーを表す型
#[derive(Debug, PartialEq)]
pub enum ShellError {
    CommandExecError(String, String),
}

/// ShellErrorを表示するため、Displayトレイトを実装
impl Display for ShellError {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::CommandExecError(cmd, msg) => {
                write!(f, "Error: The following Error is occured when execute '{cmd}'.\n{msg}")
            }
        }
    }
}

/// エラー用にErrorトレイトを実装
impl Error for ShellError {} // デフォルト実装を使うだけの場合、これだけでいい
