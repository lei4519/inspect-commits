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

use std::{env, io, process::Command};

use check::check;
use clap::{App, Arg, SubCommand};
use config::get_config_path;
use hook::{set_global_hook, unset_global_hook};

fn main() -> io::Result<()> {
    let mut app = App::new("inspect-commits")
        .version("0.0.2")
        .about("检查 Git Commits 中的敏感词")
        .subcommand(
            SubCommand::with_name("check")
                .about("检查当前分支中未提交到远程仓库的 commits")
                .arg(
                    Arg::with_name("REMOTE_REPO_NAME")
                        .help("远程仓库名称, e.g. origin / upstream\nexample:\n\tinspect-commits check origin")
                        .default_value("origin")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("REMOTE_BRANCH_NAME")
                        .help("远程分支名称，默认同本地分支名称") .index(2),
                )
                .arg(
                    Arg::with_name("LOCAL_BRANCH_NAME")
                        .help("本地分支名称，默认为当分支")
                        .index(3),
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
            let remote_repo_name = sub_m.value_of("REMOTE_REPO_NAME");
            let remote_branch_name = sub_m.value_of("REMOTE_BRANCH_NAME");
            let local_branch_name = sub_m.value_of("LOCAL_BRANCH_NAME");
            check(remote_repo_name, remote_branch_name, local_branch_name)?;
        }
        ("checkall", Some(_)) => {
            check(None, None, None)?;
        }
        ("config", Some(sub_m)) => {
            let path = get_config_path();
            let path = path.to_str().unwrap();
            if sub_m.is_present("PATH") {
                println!("{}", path);
            } else {
                let editor = env::var_os("EDITOR").unwrap_or("vi".into());
                Command::new(editor)
                    .arg(path)
                    .spawn()
                    .expect(&format!(
                        "open editor failed, please manual edit config: {}",
                        path
                    ))
                    .wait()?;
            }
        }
        ("set-global-hook", Some(_)) => {
            set_global_hook()?;
        }
        ("unset-global-hook", Some(_)) => {
            unset_global_hook()?;
        }
        _ => {
            app.print_help().unwrap();
        }
    }
    Ok(())
}
