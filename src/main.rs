/*
git config --unset core.hooksPath
git config --get core.hooksPath
git config core.hooksPath <path>
git rev-parse --show-superproject-working-tree --show-toplevel
git log --pretty=format:%H origin..HEAD
git show --pretty=format:%b hash
*/

mod check;
mod config;
mod hook;
mod utils;

use check::check;
use clap::{App, Arg, SubCommand};
use config::get_config_path;
use hook::{set_global_hook, unset_global_hook};
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut app = App::new("validate-git-push")
        .version("1.0.0")
        .about("校验 Git Commits 中的敏感信息")
        .subcommand(
            SubCommand::with_name("check")
                .about("检查当前分支中未提交到远程的 commits，必须传入远程仓库的名称")
                .arg(
                    Arg::with_name("REMOTE_NAME")
                        .help("远程仓库名称\nexample:\n\tvalidate-git-push check origin")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("REMOTE_URL")
                        .help("远程仓库地址，为空时自动获取：git config --get remote.<name>.url")
                        .index(2),
                ),
        )
        .subcommand(SubCommand::with_name("checkall").about("检查当前分支下的所有 commits"))
        .subcommand(
            SubCommand::with_name("config")
                .about("编辑配置文件；-p 参数返回文件地址")
                .arg(
                    Arg::with_name("PATH")
                        .short("p")
                        .long("path")
                        .help("返回配置文件的地址"),
                ),
        )
        .subcommand(
            SubCommand::with_name("set-global-hook")
                .about("将程序配置为 git global core.hooksPath"),
        )
        .subcommand(
            SubCommand::with_name("unset-global-hook").about("清除 git global core.hooksPath 配置"),
        );

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("check", Some(sub_m)) => {
            let remote_name = sub_m.value_of("REMOTE_NAME").unwrap();
            let remote_url = sub_m.value_of("REMOTE_URL").unwrap_or("");
            check(remote_name, remote_url).await;
        }
        ("checkall", Some(_)) => {
            check("", "").await;
        }
        ("config", Some(sub_m)) => {
            let path = get_config_path().await;
            let path = path.to_str().unwrap();
            if sub_m.is_present("PATH") {
                println!("{}", path)
            } else {
                let mut child = Command::new("vi").arg(path).spawn().expect("vim 启动失败");
                child.wait().await.unwrap();
            }
        }
        ("set-global-hook", Some(_)) => {
            set_global_hook().await;
        }
        ("unset-global-hook", Some(_)) => {
            unset_global_hook().await;
        }
        _ => {
            app.print_help().unwrap();
        }
    }
}
