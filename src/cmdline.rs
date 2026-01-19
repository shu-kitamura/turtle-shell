/// コマンドラインの型
#[derive(Debug, PartialEq)]
pub struct CommandLine {
    pub commands: Vec<ParsedCommand>,
}

/// パース済みコマンド
#[derive(Debug, PartialEq)]
pub struct ParsedCommand {
    pub index: usize,
    pub name: String,
    pub args: Vec<String>,
}

impl CommandLine {
    pub fn new(rawline: &str) -> Self {
        Self {
            commands: parse_cli(rawline),
        }
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
        return None;
    };
    // 引数を取得する
    // 引数がない場合、空の Vec を返す。
    let args: Vec<String> = tokens.map(|token| token.to_string()).collect();

    Some((command, args))
}

/// コマンドラインをパースする
fn parse_cli(cli: &str) -> Vec<ParsedCommand> {
    let commands = cli.split('|').map(|command| command.trim());
    let mut parsed: Vec<ParsedCommand> = Vec::new();
    for (i, command) in commands.enumerate() {
        if let Some((cmd, args)) = parse_command(command) {
            parsed.push(ParsedCommand {
                index: i,
                name: cmd,
                args,
            })
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
            commands: vec![ParsedCommand {
                index: 0,
                name: "ls".to_string(),
                args: vec!["-l".to_string()],
            }],
        };
        let actual_ls: CommandLine = CommandLine::new("ls -l");
        assert_eq!(actual_ls, expect_ls);

        // grep -v a.c test.txt を受け取るケース
        // (複数のオプションを受け取る)
        let expect_grep: CommandLine = CommandLine {
            commands: vec![ParsedCommand {
                index: 0,
                name: "grep".to_string(),
                args: vec!["-v".to_string(), "a.c".to_string(), "test.txt".to_string()],
            }],
        };
        let actual_grep: CommandLine = CommandLine::new("grep -v a.c test.txt");
        assert_eq!(actual_grep, expect_grep);

        // pwd を受け取るケース
        // (オプションを受け取らない)
        let expect_pwd: CommandLine = CommandLine {
            commands: vec![ParsedCommand {
                index: 0,
                name: "pwd".to_string(),
                args: vec![],
            }],
        };
        let actual_pwd: CommandLine = CommandLine::new("pwd");
        assert_eq!(actual_pwd, expect_pwd);
    }

    #[test]
    fn test_parse_cli() {
        // "ls -l | grep test" を受け取るケース
        let expect: Vec<ParsedCommand> = vec![
            ParsedCommand {
                index: 0,
                name: "ls".to_string(),
                args: vec!["-l".to_string()],
            },
            ParsedCommand {
                index: 1,
                name: "grep".to_string(),
                args: vec!["test".to_string()],
            },
        ];

        let actual: Vec<ParsedCommand> = parse_cli("ls -l | grep test");
        assert_eq!(actual, expect);
    }
}
