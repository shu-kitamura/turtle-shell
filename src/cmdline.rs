/// コマンドラインの型
#[derive(Debug, PartialEq)]
pub struct CommandLine {
    pub commands: Vec<(String, Vec<String>)>
}

impl CommandLine {
    pub fn new(rawline: &str) -> Self {
        Self { commands: parse_cli(rawline) }
    }
}

/// コマンドをパースする
fn parse_command(line: &str) -> Option<(String, Vec<String>)> {
    let mut tokens = line.split_whitespace(); // スペースで区切る
    // コマンドを取得する
    // 取得できない場合には None を返す
    let command: String = if let Some(cmd) = tokens.next() {
        cmd.to_string()
    } else {
        return None
    };
    // 引数を取得する
    // 引数がない場合、空の Vec を返す。
    let args: Vec<String> = tokens.map(|token| token.to_string()).collect();

    Some((command, args))
}

/// コマンドラインをパースする
fn parse_cli(cli:&str) -> Vec<(String, Vec<String>)> {
    let commands: Vec<&str> = cli.split('|').map(|command| command.trim()).collect();
    let mut parsed: Vec<(String, Vec<String>)> = Vec::new();
    for command in commands {
        if let Some((cmd, args)) = parse_command(command) {
            parsed.push((cmd, args))
        }
    }
    parsed
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::cmdline::*;
    #[test]
    fn test_command_line_new() {
        // ls -l を受け取るケース
        // (1つのオプションを受け取る)
        let expect_ls: CommandLine = CommandLine {
            commands: vec![
                ("ls".to_string(), vec!["-l".to_string()])
            ],
        };
        let actual_ls: CommandLine = CommandLine::new("ls -l");
        assert_eq!(actual_ls, expect_ls);

        // grep -v a.c test.txt を受け取るケース
        // (複数のオプションを受け取る)
        let expect_grep: CommandLine = CommandLine {
            commands: vec![
                ("grep".to_string(), vec!["-v".to_string(), "a.c".to_string(), "test.txt".to_string()])
            ],
        };
        let actual_grep: CommandLine = CommandLine::new("grep -v a.c test.txt");
        assert_eq!(actual_grep, expect_grep);

        // pwd を受け取るケース
        // (オプションを受け取らない)
        let expect_pwd: CommandLine = CommandLine {
            commands: vec![("pwd".to_string(),vec![])]
        };
        let actual_pwd: CommandLine = CommandLine::new("pwd");
        assert_eq!(actual_pwd, expect_pwd);
    }

    #[test]
    fn test_parse_cli() {
        // "ls -l | grep test" を受け取るケース
        let expect: Vec<(String, Vec<String>)> = vec![
            ("ls".to_string(), vec!["-l".to_string()]),
            ("grep".to_string(), vec!["test".to_string()])
        ];

        let actual: Vec<(String, Vec<String>)> = parse_cli("ls -l | grep test");
        assert_eq!(actual, expect);
    }
}