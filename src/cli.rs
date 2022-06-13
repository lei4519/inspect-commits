use clap::{Arg, Command};

pub fn build_cli() -> Command<'static> {
    Command::new("inspect-commits")
        .version("0.0.2")
        .about("检查 Git Commits 中的敏感词")
        .subcommand(
            Command::new("check")
                .about("检查当前分支中未提交到远程仓库的 commits")
                .arg(
                    Arg::new("REMOTE_REPO_NAME")
                        .help("远程仓库名称, e.g. origin / upstream\nexample:\n\tinspect-commits check origin")
                        .default_value("origin")
                        .index(1),
                )
                .arg(
                    Arg::new("REMOTE_BRANCH_NAME")
                        .help("远程分支名称，默认同本地分支名称").index(2),
                )
                .arg(
                    Arg::new("LOCAL_BRANCH_NAME")
                        .help("本地分支名称，默认为当分支")
                        .index(3),
                ),
        )
        .subcommand(Command::new("checkall").about("检查当前分支下的所有 commits"))
        .subcommand(
            Command::new("config")
                .about("编辑配置文件；-p 参数返回文件地址")
                .arg(
                    Arg::new("PATH")
                        .short('p')
                        .long("path")
                        .help("返回配置文件的地址"),
                ),
        )
        .subcommand(
            Command::new("set-global-hook")
                .about("将程序配置为 git global core.hooksPath"),
        )
        .subcommand(
            Command::new("unset-global-hook").about("清除 git global core.hooksPath 配置"),
        )
}
