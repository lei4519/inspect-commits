use colored::*;
use std::process::exit;

use crate::config::read_config;
use crate::utils::{exec_out_call, exec_out_str, spawn};

pub async fn check(remote_name: &str, remote_url: &str) {
    println!("{}", "Validate Git Push running...".cyan());

    let mut c = spawn("git", ["rev-parse"]).await;
    let status = c.wait().await.expect("执行 git rev-parse 命令失败");

    if matches!(status.code(), Some(code) if code != 0) {
        println!("当前工作目录不是 git 仓库");
        return
    }

    let (config, ..) = read_config().await;
    let mut remote_url = remote_url.to_string();

    if let Some(rules) = config.rules {
        if !remote_name.is_empty() && remote_url.is_empty() {
            remote_url = exec_out_str(
                "git",
                [
                    "config",
                    "--get",
                    format!("remote.{}.url", remote_name).as_str(),
                ],
            )
            .await;
            if remote_url.is_empty() {
                println!("远程仓库不存在，请检查仓库名称：{}", remote_name);
                exit(1);
            }
        }

        let arg = if remote_name.is_empty() {
            // 没有 remote_name 说明要检查所有的 commits
            // 不能为空，必须放一个参数，这个参数不影响输出
            "--skip=0".to_string()
        } else {
            format!("{}..HEAD", remote_name)
        };
        let commits = exec_out_str("git", ["log", "--pretty=format:%H", arg.as_str()]).await;

        if commits.is_empty() {
            println!("{}", "当前分支没有未提交的 commit");
            return
        }

        let commits = commits
            .split("\n")
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        println!("{}\n{:#?}\n", "Check Commits List:".cyan(), commits);

        for mut rule in rules.into_iter() {
            rule.excludes.retain(|s|!s.is_empty());
            rule.words.retain(|s|!s.is_empty());

            if rule.words.is_empty() {
                continue;
            }

            if rule.excludes.iter().any(|word| remote_url.contains(word)) {
                continue;
            }

            for commit_hash in commits.iter() {
                exec_out_call("git", ["show", commit_hash, "--pretty=format:%b"], |line| {
                    rule.words.iter().any(|word| {
                        if line.contains(word) {
                            println!(
                                "{}\n{} {}\n{} {}\n{}\n{}",
                                "Commit Not Secure!!!".red().bold(),
                                "Discover Word:".red().bold(),
                                word,
                                "Commit Hash:".red().bold(),
                                commit_hash,
                                "Content:".red().bold(),
                                line.replace(word, &format!("\x1b[41;37m{}\x1b[0m", word))
                                    .on_bright_black()
                            );
                            exit(1);
                        }
                        false
                    });
                    true
                })
                .await;
            }
        }
    } else {
        println!("{}", " 未配置 rules".cyan(),);
    }
    println!("{}", "Commits is secure".green(),);
}
