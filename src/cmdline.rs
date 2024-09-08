/// コマンドラインの型
#[derive(Debug, PartialEq)]
pub struct CommandLine {
    pub command: String,
    pub args: Vec<String>,
}

impl CommandLine {
    pub fn new(line: &str) -> Option<Self> {
        let (command, args) = match parse_command(line) {
            Some((command, args)) => (command, args),
            None => return None
        };
        Some(Self {
            command: command.to_string(),
            args: args.iter().map(|s| s.to_string()).collect()
        })
    }
}

/// コマンドをパースする
fn parse_command(line: &str) -> Option<(&str, Vec<&str>)> {
    let mut tokens = line.split_whitespace(); // スペースで区切る
    // コマンドを取得する
    // 取得できない場合には None を返す
    let command: &str = if let Some(cmd) = tokens.next() {
        cmd
    } else {
        return None
    };
    // 引数を取得する
    // 引数がない場合、空の Vec を返す。
    let args: Vec<&str> = tokens.collect();

    Some((command, args))
}

#[cfg(test)]
mod tests {
    use crate::cmdline::*;
    #[test]
    fn test_command_line_new() {
        // ls -l を受け取るケース
        // (1つのオプションを受け取る)
        let expect_ls: CommandLine = CommandLine {
            command: String::from("ls"),
            args: vec![String::from("-l")]
        };
        let actual_ls: CommandLine = CommandLine::new("ls -l").unwrap();
        assert_eq!(actual_ls, expect_ls);

        // grep -v a.c test.txt を受け取るケース
        // (複数のオプションを受け取る)
        let expect_grep: CommandLine = CommandLine {
            command: String::from("grep"),
            args: vec![
                String::from("-v"),
                String::from("a.c"),
                String::from("test.txt")
            ]
        };
        let actual_grep: CommandLine = CommandLine::new("grep -v a.c test.txt").unwrap();
        assert_eq!(actual_grep, expect_grep);

        // pwd を受け取るケース
        // (オプションを受け取らない)
        let expect_pwd: CommandLine = CommandLine {
            command: String::from("pwd"),
            args: vec![]
        };
        let actual_pwd: CommandLine = CommandLine::new("pwd").unwrap();
        assert_eq!(actual_pwd, expect_pwd);
    }
}